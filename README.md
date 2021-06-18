# rust-cfs

A completely fair process scheduler in Rust.

## Background

What is CFS? The completely fair scheduler is a process scheduler used in the Linux kernel since
kernel version 2.6.23, replacing its predecessor the O(1) scheduler.
A completely fair scheduler makes use of a red-black tree for its ready queue. All processes are
put into the tree as a `(key, value)` pair, with the key being its _virtual runtime_.
Virtual runtime, or "vruntime," is a calculated time spent for a task in its existence with a correction
for its weight. The weight of the task is essentially its priority: the larger the weight, the longer it will
be allowed to hang on the processor. Some implementations allow for a dynamic manipulation of this weight,
which allows the scheduler to punish processes that starve others by hogging the CPU.

## Implementation

This implementation of the scheduler is **based** on the one found in the Linux kernel.
Although the algorithm is similar, the specifics as to how the scheduler functions is different.

### Spawning

Since this is a simulated scheduler, properties like weight, maximum allocated CPU time, CPU burst length
(how long it is allowed to run on the processor before it becomes idle), and I/O burst length
(how long it is allowed to remain idle before being re-scheduled) are required to be determined before
the scheduler runs its instance.

In an actual scheduler, these might be calculated by the kernel at the birth of a task based on the situation.

This implementation runs the scheduler instance by performing a `scheduler.run(tasks)` call, where
the `tasks` is a vector of `TaskChar` -- a task characterization. A task characterization holds
the properties of a task as mentioned before plus its identification value (otherwise known as a pid).
A task is then created in a **spawning thread** that looks at the current system time to produce its
start time (or time of birth).

### Feeding

The feeding process is where the newborn tasks get 'fed' into the ready queue via a
"born queue." From this born queue, it is then put into the scheduler through the feeding process.
This born queue is a priority-based queue with a priority set to the task's time of birth.
This way, tasks get continuously put into the ready queue in order of conception.

The exact process dequeues the tasks with the most recent start time as a vector of tasks.
All of these tasks are then pushed into the scheduler's red-black tree and has its initial virtual runtime
calculated.

An implementation of this might look something like this:
```rust
let mut born_queue = TaskQueue::new();

// let there be some sender that sends
// a vector of tasks from this born queue

let running = thread::spawn(move || {
    let fed: Vec<Task> = match feeding_receiver.try_recv() {
        Ok(newborns) => newborns,
	_ => Vec::new()
    };
    ready_queue.push(fed);  // nothing will happen if an empty vector is pushed

    ...
});
```

### Running

The running of the scheduler algorithm is done in a **running** thread, which you saw part of in the sample
in the previous section on task feeding. The running thread waits for the next systick, which is a tick
of the internal (virtual) clock. Once the tick is received, it runs one iteration of the running event.
This running event pops the highest priority task off the ready queue and lets it run on the CPU, performing
part of its CPU burst. The task is then re-scheduled into the ready queue and its vruntime is recalculated on
arrival.

When a task finishes its CPU burst, it gets put into the idle queue. The idle queue is simply a FIFO queue
that runs one iteration of the task's I/O burst before either putting it back into the ready queue at the end
of its I/O burst or pushing it to the back of the idle queue.
**This is very inefficient, and I apologize**.

The implementation of this part of the scheduler might appear as such:
```rust
let running = thread::spawn(move || {
    ...

    if !ready_queue.is_empty() {
        let mut curr_task = ready_queue.pop();
        curr.cpu_cycle();  // runs one iteration of the task's CPU burst

	ready_queue.push(*curr);
    }
    ready_queue.idle();  // runs one iteration of the idle sequence
                         // for the dequeued task in the FIFO idle queue
});
```

## Usage

You can generate a sequence of random tasks if you have python3 on your device with
`python3 generate_tasks.py` in the the root of this project. This will write
a sequence of needed characteristics to a `tasks.txt` file, which is then read by the
`main` function in the Rust program to generate the born tasks. To run this, execute
`cargo run` if you have cargo (which you should if you're sane).
