use super::clock::Clock;
use super::fair::FairAlgorithm;

use crate::proc::task::{Task, TaskChar};
use crate::proc::queue::TaskQueue;

use std::thread;
use std::sync::{Arc, Mutex, mpsc};

pub struct Scheduler {
    clock: Arc<Mutex<Clock>>,
}

impl Scheduler {
    pub fn new() -> Self {
        let clock = Arc::new(Mutex::new(Clock::new()));
        
        Self { clock }
    }

    pub fn run(&mut self, tasks: Vec<TaskChar>) {
        let clk_1 = Arc::clone(&mut self.clock);
        let clk_2 = Arc::clone(&mut self.clock);

        let mut threads = vec![];

        let (clock_sender_1, spawner_clock_recv) = mpsc::channel();
        let (clock_sender_2, clock_recv) = mpsc::channel();
        let (born_sender, born_recv) = mpsc::channel();

        let clocking = thread::spawn(move || {
            for _ in 0..u128::MAX {
                let mut lock = clk_1.lock().unwrap();
                match clock_sender_1.send(lock.time()) {
                    Ok(_) => {},
                    _ => {}
                };
                match clock_sender_2.send(lock.time()) {
                    Ok(_) => {},
                    _ => break
                };
                lock.tick();
            }
            drop(clock_sender_1);
            drop(clock_sender_2);
        });
        threads.push(clocking);

        let my_tasks = Arc::new(Mutex::new(tasks));
        let tasks_cp_1 = Arc::clone(&my_tasks);
        let tasks_cp_2 = Arc::clone(&my_tasks);

        let spawning = thread::spawn(move || {
            let mut time = spawner_clock_recv.recv().unwrap();
            
            let born_tasks = tasks_cp_1.lock().unwrap().clone();

            for raw in born_tasks {
                time = match spawner_clock_recv.try_recv() {
                    Ok(tick) => tick,
                    _ => time
                };

                let task = Task::new(
                    raw.get_id(),
                    raw.get_cpu_time(),
                    raw.get_cpu_burst_length(),
                    raw.get_io_burst_length(),
                    time,
                    raw.get_weight()
                );

                match born_sender.send(task) {
                    Ok(_) => {},
                    _ => panic!("Running thread dropped unexpectedly")
                };
            }

            tasks_cp_1.lock().unwrap().clear();
            drop(spawner_clock_recv);
            drop(born_sender);
        });
        threads.push(spawning);

        let running = thread::spawn(move || {
            let mut task_queue = TaskQueue::new();
            let mut rq = FairAlgorithm::new(&mut clk_2.lock().unwrap());

            loop {
                let time = match clock_recv.recv() {
                    Ok(tick) => tick,
                    _ => break
                };

                match born_recv.try_recv() {
                    Ok(task) => task_queue.add(task),
                    _ => {}
                };

                let born_tasks = task_queue.pop();
                rq.push(born_tasks);

                if !rq.is_empty() {
                    let mut curr = rq.pop();
                    println!("Running task id {:?} at system time {:?}", curr.get_id(), time);
                    curr.cpu_cycle();
                    rq.insert(*curr);
                }
                rq.idle();

                if rq.is_finished() && tasks_cp_2.lock().unwrap().is_empty() {
                    break;
                }
            }

            drop(clock_recv);
            drop(born_recv);
        });
        threads.push(running);

        for thread in threads {
            thread.join().unwrap();
        }

        println!("Scheduler job complete");
    }
}
