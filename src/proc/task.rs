#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskStatus {
    Idle,
    Running,
    Terminated,
    Waiting,
    New,
}

#[derive(Clone, Copy)]
pub struct Task {
    id: u16,
    cpu_time: u64,
    cpu_burst_length: u64,
    io_burst_length: u64,
    state: TaskStatus,
    runtime: u64,
    vruntime: u64,
    idle_time: u64,
    start_time: u128,
    weight: u32,
}

impl Task {
    pub fn new(
        id: u16,
        cpu_time: u64,
        cpu_burst_length: u64,
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
    pub fn get_runtime(&self) -> u64 {
        self.runtime
    }

    #[inline]
    pub fn terminate(&mut self) {
        self.state = TaskStatus::Terminated
    }

    #[inline]
    pub fn weight(&self) -> u32 {
        self.weight
    }

    #[inline]
    pub fn vruntime(&mut self, now: u128) -> u64 {
        let dt: u64 = (now - self.start_time) as u64;
        let delta_exec_weighted: u64 = dt / (self.weight as u64);
        self.vruntime += delta_exec_weighted;

        self.vruntime
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
