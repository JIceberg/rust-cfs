#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskStatus {
    Idle,
    Running,
    Terminated,
    Waiting,
    New,
}

#[derive(Debug)]
pub struct Task {
    id: u16,
    cpu_time: usize,
    cpu_burst_length: usize,
    io_burst_length: u64,
    state: TaskStatus,
    runtime: usize,
    vruntime: u64,
    idle_time: u64,
    start_time: u128,
    weight: u32,
}

impl Task {
    pub fn new(
        id: u16,
        cpu_time: usize,
        cpu_burst_length: usize,
        io_burst_length: u64,
        start_time: u128,
        weight: u32
    ) -> Self {

        Self {
            id, cpu_time,
            cpu_burst_length,
            io_burst_length,
            state: TaskStatus::New,
            runtime: 0,
            vruntime: 0,
            idle_time: 0,
            start_time,
            weight
        }

    }

    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_cpu_time(&self) -> usize {
        self.cpu_time
    }

    pub fn get_start_time(&self) -> u128 {
        self.start_time
    }

    pub fn get_status(&self) -> TaskStatus {
        self.state
    }

    pub fn get_runtime(&self) -> usize {
        self.runtime
    }

    pub fn terminate(&mut self) {
        self.state = TaskStatus::Terminated
    }

    pub fn weight(&self) -> u32 {
        self.weight
    }

    pub fn vruntime(&mut self, now: u128) -> u64 {
        let dt: u64 = now.overflowing_sub(self.start_time).0 as u64;
        let delta_exec_weighted: u64 = dt / (self.weight as u64);
        self.vruntime += delta_exec_weighted;

        self.vruntime
    }

    pub fn to_idle(&mut self) {
        match self.state {
            TaskStatus::Terminated => panic!("Cannot yield a terminated task ({:?})!", self.id),
            _ => self.state = TaskStatus::Idle
        }
    }

    pub fn schedule(&mut self) {
        match self.state {
            TaskStatus::Terminated => panic!("Cannot schedule a terminated task: ({:?})!", self.id),
            _ => self.state = TaskStatus::Waiting
        }
    }

    pub fn run(&mut self) {
        self.state = TaskStatus::Running
    }

    pub fn restart(&mut self, time: u128) {
        self.runtime = 0;
        self.idle_time = 0;
        self.state = TaskStatus::New;
        self.start_time = time;
    }

    pub fn cpu_cycle(&mut self) {
        match self.state {
            TaskStatus::Running => {
                self.runtime += 1;
                if self.runtime >= self.cpu_time {
                    self.terminate();
                } else if self.runtime % self.cpu_burst_length == 0 {
                    self.to_idle();
                }
            },
            _ => println!("Task {:?} is not running", self.id)
        }
    }

    pub fn io_cycle(&mut self) {
        match self.state {
            TaskStatus::Idle => {
                self.idle_time += 1;
                if self.idle_time >= self.io_burst_length {
                    self.idle_time = 0;
                    self.schedule();
                }
            },
            _ => println!("Task {:?} is currently not idle", self.id)
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Task) -> bool {
        self.id == other.id
    }
}

impl Clone for Task {
    fn clone(&self) -> Self {
        Self {
            id:                 self.id,
            cpu_time:           self.cpu_time,
            cpu_burst_length:   self.cpu_burst_length,
            io_burst_length:    self.io_burst_length,
            state:              self.state,
            runtime:            self.runtime,
            vruntime:           self.vruntime,
            idle_time:          self.idle_time,
            start_time:         self.start_time,
            weight:             self.weight
        }
    }
}

impl Copy for Task {}
