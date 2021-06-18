extern crate rust_cfs as cfs;

use cfs::sched::scheduler::Scheduler;
use cfs::proc::task::TaskChar;

use std::io::Read;
use std::fs::File;

fn main() {
    let mut file = File::open("tasks.txt").expect("Could not find tasks file");
    let mut file_str = String::new();
    file.read_to_string(&mut file_str).expect("Unable to write contents of tasks file to string");

    let task_lines_split = file_str.as_str().split("\n");
    let task_lines: Vec<&str> = task_lines_split.collect::<Vec<&str>>();
    
    let mut task_props = vec![];
    for line in task_lines {
        let props_split = line.split_whitespace().take(4);
        task_props.push(props_split.collect::<Vec<&str>>());
    }

    let mut tasks: Vec<TaskChar> = Vec::new();
    
    let mut idx: u16 = 1;
    for task in task_props {
        if let [cpu_time, cpu_burst_length, io_burst_length, weight] = task[..] {
            tasks.push(TaskChar::new(
                idx,
                cpu_time.parse::<u64>().unwrap(),
                cpu_burst_length.parse::<u64>().unwrap(),
                io_burst_length.parse::<u64>().unwrap(),
                weight.parse::<u32>().unwrap()
            ));
        }
        idx += 1;
    }

    let mut scheduler = Scheduler::new();

    scheduler.run(tasks);
}
