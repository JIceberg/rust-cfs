extern crate rbtree;
extern crate raw_pointer as rptr;

use rbtree::RBTree;
use rptr::Pointer;
use super::clock::Clock;
use crate::proc::task::{Task, TaskStatus};
use std::collections::VecDeque;

pub struct FairAlgorithm {
    tree: RBTree<u64, Task>,
    idle: VecDeque<Task>,
    clock: Pointer<Clock>,
}

impl FairAlgorithm {
    pub fn new(clock: &mut Clock) -> Self {
        Self {
            tree: RBTree::new(),
            idle: VecDeque::new(),
            clock: Pointer::new(clock)
        }
    }

    #[inline]
    pub fn push(&mut self, tasks: Vec<Task>) {
        for task in tasks {
            self.insert(task);
        }
    }

    #[inline]
    pub fn insert(&mut self, mut task: Task) {
        let state = task.get_status();
        if state == TaskStatus::Terminated {
            return;
        } else if state == TaskStatus::Idle {
            self.idle.push_back(task);
            return;
        }
        let key: u64 = task.vruntime(self.clock.time());
        task.schedule();
        self.tree.insert(key, task);
    }

    #[inline]
    pub fn pop(&mut self) -> Box<Task> {
        if self.is_empty() {
            panic!("Attempted to pop from an empty tree");
        }
        let mut task = Box::new(
            self.tree
                .pop_first()
                .unwrap()
                .1
        );
        task.run();

        task
    }

    #[inline]
    pub fn is_empty(&mut self) -> bool {
        self.tree.is_empty()
    }

    pub fn run(&mut self) {
        if self.is_empty() {
            return;
        }

        let mut task = *self.pop();
        task.cpu_cycle();
        self.insert(task);
    }

    pub fn idle(&mut self) {
        if self.idle.len() == 0 {
            return;
        }

        let mut curr = self.idle.pop_front().unwrap();
        curr.io_cycle();
        self.insert(curr);
    }
}

unsafe impl Sync for FairAlgorithm {}
