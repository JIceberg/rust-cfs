use crate::tree::node::NodePtr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskStatus {
    Idle,
    Running,
    Terminated,
    Waiting,
    New,
}

#[derive(Clone)]
pub struct Task {
    id: u16,
    cpu_time: u64,
    io_time_duration: u64,
    state: TaskStatus,
    runtime: u64,
    idle_time: u64,
    start_time: u128,
    run_node: NodePtr<u64, Box<Task>>,
}

impl Task {
    pub fn new(id: u16, cpu_time: u64, io_time_duration: u64, start_time: u128) -> Self {
        Self {
            id, cpu_time,
            io_time_duration,
            state: TaskStatus::New,
            runtime: 0,
            idle_time: 0,
            start_time,
            run_node: NodePtr::null()
        }
    }

    pub fn execute<F, T>(self, f: F) -> T
    where
        F: FnOnce() -> T,
        F: 'static,
        T: 'static,
    {
        f()
    }

    #[inline]
    pub fn get_id(&self) -> u16 {
        self.id
    }

    #[inline]
    pub fn get_cpu_time(&self) -> u64 {
        self.cpu_time
    }

    #[inline]
    pub fn get_start_time(&self) -> u128 {
        self.start_time
    }

    #[inline]
    pub fn get_status(&self) -> TaskStatus {
        self.state
    }

    #[inline]
    pub fn terminate(&mut self) {
        self.state = TaskStatus::Terminated
    }

    #[inline]
    pub fn to_idle(&mut self) {
        match self.state {
            TaskStatus::Terminated => panic!("Cannot yield a terminated task ({:?})!", self.id),
            _ => self.state = TaskStatus::Idle
        }
    }

    #[inline]
    pub fn schedule(&mut self) {
        match self.state {
            TaskStatus::Terminated => panic!("Cannot schedule a terminated task: ({:?})!", self.id),
            _ => self.state = TaskStatus::Waiting
        }
    }

    #[inline]
    pub fn run(&mut self) {
        self.state = TaskStatus::Running
    }

    #[inline]
    pub fn restart(&mut self, time: u128) {
        self.runtime = 0;
        self.idle_time = 0;
        self.state = TaskStatus::New;
        self.start_time = time;
        self.run_node = NodePtr::new(self.id as u64, unsafe { Box::from_raw(self as *mut Task) });
    }

    pub fn cpu_cycle(&mut self) -> &NodePtr<u64, Box<Task>> {
        match self.state {
            TaskStatus::Running => {
                if self.runtime >= self.cpu_time {
                    self.terminate();
                } else {
                    self.runtime = self.runtime + 1;
                }
            },
            _ => println!("Task {:?} is not running", self.id)
        }
        
        self.run_node = NodePtr::new(self.runtime + self.id as u64, unsafe { Box::from_raw(self as *mut Task) });
        &self.run_node
    }

    pub fn io_cycle(&mut self) {
        match self.state {
            TaskStatus::Idle => {
                if self.idle_time >= self.io_time_duration {
                    self.idle_time = 0;
                    self.schedule();
                } else {
                    self.idle_time = self.idle_time + 1;
                }
            },
            _ => println!("Task {:?} is currently not idle", self.id)
        }
    }
}
