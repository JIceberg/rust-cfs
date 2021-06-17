#![cfg(test)]

extern crate rust_cfs as cfs;

use cfs::proc::task::Task;
use cfs::proc::queue::TaskQueue;
use cfs::sched::{clock::Clock, fair::FairAlgorithm};

use std::thread;
use std::sync::{Arc, Mutex};

#[test]
fn test_no_update() {
    let task_one    = Task::new(1, 13191, 10, 5, 1, 5);
    let task_two    = Task::new(2, 13289, 10, 5, 1, 4);
    let task_three  = Task::new(3, 139, 10, 5, 2, 8);
    let task_four   = Task::new(4, 31921, 5, 10, 3, 4);
    let task_five   = Task::new(5, 3874, 7, 3, 5, 2);
    let task_six    = Task::new(6, 17013, 10, 6, 5, 5);

    let mut task_queue = TaskQueue::new();

    task_queue.append(&[
        task_one,
        task_two,
        task_three,
        task_four,
        task_five,
        task_six
    ]);

    let mut sysclock = Clock::new();
    let mut rq = FairAlgorithm::new(&mut sysclock);

    sysclock.tick();

    while !task_queue.is_empty() {
        let tasks = task_queue.pop();
        rq.push(tasks);
        sysclock.tick();
    }

    let first = rq.pop();
    assert_eq!(first.get_id(), 1);

    sysclock.tick();

    let second = rq.pop();
    assert_eq!(second.get_id(), 2);

    sysclock.tick();

    let third = rq.pop();
    assert_eq!(third.get_id(), 3);
}

#[test]
fn test_with_update() {
    let task_one    = Task::new(1, 13191, 10, 5, 1, 5);
    let task_two    = Task::new(2, 13289, 10, 5, 1, 4);
    let task_three  = Task::new(3, 139, 10, 5, 2, 8);
    let task_four   = Task::new(4, 31921, 5, 10, 3, 4);
    let task_five   = Task::new(5, 3874, 7, 3, 5, 2);
    let task_six    = Task::new(6, 17013, 10, 6, 5, 5);

    let mut task_queue = TaskQueue::new();

    task_queue.append(&[
        task_one,
        task_two,
        task_three,
        task_four,
        task_five,
        task_six
    ]);

    let mut sysclock = Clock::new();
    let mut rq = FairAlgorithm::new(&mut sysclock);

    sysclock.tick();

    while !task_queue.is_empty() {
        let tasks = task_queue.pop();
        rq.push(tasks);
        sysclock.tick();
    }

    let mut curr = rq.pop();
    assert_eq!(curr.get_id(), 1);

    curr.cpu_cycle();
    sysclock.tick();
    rq.insert(*curr);

    curr = rq.pop();
    assert_eq!(curr.get_id(), 2);

    curr.cpu_cycle();
    sysclock.tick();
    rq.insert(*curr);

    curr = rq.pop();
    assert_eq!(curr.get_id(), 3);
}

#[test]
fn test_multithreaded_clock() {
    let mut sysclock = Arc::new(Mutex::new(Clock::new()));

    let mut threads = vec![];
    let (sender, receiver) = std::sync::mpsc::channel();

    let clk = Arc::clone(&mut sysclock);
    let sending = thread::spawn(move || {
        for _ in 0..50 {
            let mut lock = clk.lock().unwrap();
            sender.send(lock.time()).unwrap();
            lock.tick();
        }
        drop(sender);
    });
    threads.push(sending);

    let c_clk = Arc::clone(&mut sysclock);
    let receiving = thread::spawn(move || {
        let mut rq = FairAlgorithm::new(&mut c_clk.lock().unwrap());

        let mut curr_time = receiver.recv().unwrap();
        let task_one = Task::new(1, 15, 5, 3, curr_time, 1);
        curr_time = match receiver.try_recv() {
            Ok(tick) => tick,
            Err(_) => curr_time
        };
        let task_two = Task::new(2, 15, 3, 5, curr_time, 1);

        let mut task_queue = TaskQueue::new();
        task_queue.append(&[
            task_one,
            task_two
        ]);

        while !task_queue.is_empty() {
            let tasks = task_queue.pop();
            rq.push(tasks);
        }

        loop {
            let _time = match receiver.recv() {
                Ok(tick) => tick,
                Err(_) => break
            };

            if !rq.is_empty() {
                let mut curr = rq.pop();
                curr.cpu_cycle();
                rq.insert(*curr);
            }
            rq.idle();
        }
        drop(receiver);
    });
    threads.push(receiving);

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(sysclock.lock().unwrap().time(), 50);
}

#[test]
fn test_efficient_threads() {
    let mut sysclock = Arc::new(Mutex::new(Clock::new()));

    let mut threads = vec![];
    let (clock_sender_1, spawner_clock_recv) = std::sync::mpsc::channel();
    let (clock_sender_2, clock_recv) = std::sync::mpsc::channel();
    let (born_sender, born_recv) = std::sync::mpsc::channel();

    let clk = Arc::clone(&mut sysclock);
    let ticking = thread::spawn(move || {
        for _ in 0..100 {
            let mut lock = clk.lock().unwrap();
            match clock_sender_1.send(lock.time()) {
                Ok(_) => {},
                _ => {}
            };
            clock_sender_2.send(lock.time()).unwrap();
            lock.tick();
        }
        drop(clock_sender_1);
        drop(clock_sender_2);
    });
    threads.push(ticking);

    let c_clk = Arc::clone(&mut sysclock);

    let task_spawning = thread::spawn(move || {
        let mut time = match spawner_clock_recv.recv() {
            Ok(tick) => tick,
            Err(_) => panic!("Ran out of time before the processes could be born, check bounds")
        };
        let task_one = Task::new(1, 15, 5, 3, time, 1);
        match born_sender.send(task_one) {
            Ok(_) => {},
            _ => {}
        };
        time = match spawner_clock_recv.try_recv() {
            Ok(tick) => tick,
            Err(_) => time
        };
        let task_two = Task::new(2, 15, 3, 5, time, 1);
        match born_sender.send(task_two) {
            Ok(_) => {},
            _ => {}
        };
    });
    threads.push(task_spawning);

    let receiving = thread::spawn(move || {
        let mut task_queue = TaskQueue::new();
        let mut rq = FairAlgorithm::new(&mut c_clk.lock().unwrap());
        
        loop {
            let _time = match clock_recv.recv() {
                Ok(tick) => tick,
                Err(_) => break
            };

            match born_recv.try_recv() {
                Ok(task) => task_queue.add(task),
                _ => {}
            };

            let born_tasks = task_queue.pop();
            rq.push(born_tasks);

            if !rq.is_empty() {
                let mut curr = rq.pop();
                curr.cpu_cycle();
                rq.insert(*curr);
            }
            rq.idle();
        }
        drop(clock_recv);
    });
    threads.push(receiving);

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(sysclock.lock().unwrap().time(), 100);
}
