pub mod proc;
pub mod sched;

// TODO:
//
// 1. Make the clock multithreaded: tick the system clock on a seprate thread, so..
// 2. Make the clock a thread-safe resource
//
// 3. Make the run and idle stages of the algorithm thread-safe
// 4. Create the scheduler
