use super::{
    task::{TaskId, TaskStatus},
    Async,
};
use crate::prelude::*;
use std::time::Instant;

#[derive(Clone)]
pub enum Future<'a, T>
where
    T: Clone,
{
    Const(Result<T>),
    TaskResult { r#async: Async<'a>, tid: TaskId },
}

impl<'a, T> Future<'a, T>
where
    T: Clone + 'static,
{
    pub fn r#const(value: T) -> Future<'a, T> {
        Future::Const(Ok(value))
    }

    pub fn task_result(r#async: Async<'a>, tid: TaskId) -> Future<'a, T> {
        Future::TaskResult { r#async, tid }
    }

    pub fn completed(&self) -> bool {
        match self {
            Future::Const(_) => true,
            Future::TaskResult { r#async, tid } => {
                match r#async.task_status(*tid) {
                    TaskStatus::Completed(_) => true,
                    TaskStatus::AsleepUntil(_) => false,
                }
            }
        }
    }

    pub fn poll(&self, now: Instant) -> Result<T> {
        eprintln!("# Future::poll()");
        match self {
            Future::Const(v) => v.clone(),
            Future::TaskResult { r#async, tid } => {
                r#async.service(now);
                r#async.task_status(*tid).into()
            }
        }
    }
}

impl<'a, T> Drop for Future<'a, T>
where
    T: Clone,
{
    fn drop(&mut self) {
        match self {
            Future::Const(_) => (),
            Future::TaskResult { r#async, tid } => {
                r#async.drop_task(*tid);
            }
        }
    }
}
