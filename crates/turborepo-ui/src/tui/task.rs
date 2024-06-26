#![allow(dead_code)]
use std::time::Instant;

use super::event::TaskResult;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Planned;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Running {
    start: Instant,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Finished {
    start: Instant,
    end: Instant,
    result: TaskResult,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Task<S> {
    name: String,
    state: S,
}

impl<S> Task<S> {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Task<Planned> {
    pub fn new(name: String) -> Task<Planned> {
        Task {
            name,
            state: Planned,
        }
    }

    pub fn start(self) -> Task<Running> {
        Task {
            name: self.name,
            state: Running {
                start: Instant::now(),
            },
        }
    }
}

impl Task<Running> {
    pub fn finish(self, result: TaskResult) -> Task<Finished> {
        let Task {
            name,
            state: Running { start },
        } = self;
        Task {
            name,
            state: Finished {
                start,
                result,
                end: Instant::now(),
            },
        }
    }

    pub fn start(&self) -> Instant {
        self.state.start
    }
}

impl Task<Finished> {
    pub fn start(&self) -> Instant {
        self.state.start
    }

    pub fn end(&self) -> Instant {
        self.state.end
    }

    pub fn result(&self) -> TaskResult {
        self.state.result
    }
}
