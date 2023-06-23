use std::{time::{Instant, Duration}, cell::RefCell, collections::VecDeque};
use crate::IntervalDecision;

pub enum TimeAction {
    Once {
        callback: Box<dyn Fn()>,
        fire_at: Instant
    },
    Repeat {
        callback: Box<dyn FnMut() -> IntervalDecision>,
        fire_at: Instant,
        interval: Duration,
    }
}


pub struct TimeManager {
    queue: RefCell<VecDeque<TimeAction>>,
    maybe_next_interval: RefCell<Instant>,
}

thread_local! {
    static TIME_MANAGER: TimeManager = TimeManager::new();
}

impl TimeManager {
    fn new() -> Self {
        Self {
            queue: RefCell::new(VecDeque::new()),
            maybe_next_interval: RefCell::new(Instant::now()),
        }
    }

    pub fn clear() {
        TIME_MANAGER.with(|manager| {
            let mut queue = manager.queue.borrow_mut();
            queue.clear()
        })
    }

    pub fn add(action: TimeAction) {
        TIME_MANAGER.with(|manager| {

            let mut queue = manager.queue.borrow_mut();
            assert_eq!(queue.len(), 0, "Not implemented!");

            // TODO: add in correct position 
            queue.push_back(action)
        })
    }

    pub fn get_remaining_time() -> Duration {
        TIME_MANAGER.with(|manager| {

            let queue = manager.queue.borrow();
            let task = queue.get(0);

            let next_scheduled = if let Some(task) = task {
                let now = Instant::now();
                
                match task {
                    TimeAction::Once { fire_at, .. } => {
                        *fire_at - now
                    },
                    TimeAction::Repeat { fire_at, .. } => {
                        *fire_at - now
                    }
                }
            } else {
                Duration::ZERO
            };


            let next_maybe = *manager.maybe_next_interval.borrow() - Instant::now();

            println!("Scheduled in: {:?}, Maybe in: {:?}", next_scheduled, next_maybe);

            if next_maybe.is_zero() || (next_scheduled < next_maybe && !next_scheduled.is_zero()){
                next_scheduled
            } else {
                next_maybe
            }
            
        })
    }

    fn wait(mut action: TimeAction) {
        match action {
            TimeAction::Once { callback, fire_at } => {
                let fire_in = fire_at - Instant::now();

                if !fire_in.is_zero() {
                    std::thread::sleep(fire_in)
                }

                callback()
            },
            TimeAction::Repeat { ref mut callback, ref mut fire_at, ref interval } => {
                
                // Wait for the right time
                let fire_in = *fire_at - Instant::now();
                if !fire_in.is_zero() {
                    std::thread::sleep(fire_in)
                }

                // Preapre for next iteration
                *fire_at += *interval;


                // Save in maybe_next_interval for calls of get_ramaing_time
                TIME_MANAGER.with(|manager| {
                    *manager.maybe_next_interval.borrow_mut() = *fire_at;
                });

                let decision = callback();

                // TODO: Remove only this TimeAction
                if let IntervalDecision::Repeat = decision {
                    TimeManager::add(action);
                }
            }
        }
    }

    pub fn spin() {
        TIME_MANAGER.with(|manager| {
            loop {
                let mut queue = manager.queue.borrow_mut();
                let mut action = queue.pop_front();
                
                if let Some(action) = action {
                    drop(queue);
                    TimeManager::wait(action)
                } else {
                    break
                }
            }
        })
    }
}
