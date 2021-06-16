extern crate rbtree;
extern crate raw_pointer as rptr;

use rbtree::RBTree;
use rptr::Pointer;
use super::clock::Clock;
use crate::proc::task::Task;

pub struct FairAlgorithm {
    tree: RBTree<u64, Task>,
    clock: Pointer<Clock>,
}

impl FairAlgorithm {
    pub fn new(clock: &mut Clock) -> Self {
        Self {
            tree: RBTree::new(),
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
        let key: u64 = task.vruntime(self.clock.time());
        task.schedule();
        self.tree.insert(key, task);
    }

    #[inline]
    pub fn pop(&mut self) -> Task {
        let mut task = self.tree
                           .pop_first()
                           .unwrap()
                           .1;
        task.run();

        task
    }
}