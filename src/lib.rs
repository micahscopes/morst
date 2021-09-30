#![feature(type_alias_impl_trait)]

use float_duration::FloatDuration;
use std::error::Error;
use dispose::{Disposable, Dispose};

type Time = FloatDuration;
type Delay = Time;
type Period = Time;
type Offset = Time;


pub trait Sink<A> {
    fn event(&self, time: Time, a:A) -> ();
    fn error(&self, time: Time, error: impl Error) -> ();
    fn end(&self, time: Time) -> ();
}
pub trait Scheduler {
    fn current_time(&self) -> Time;
    fn schedule_task(&self, offset: Offset, delay: Delay, period: Period, task: impl Task) -> ScheduledTask;
    fn relative(&self, offset: Offset) -> Self;
    fn cancel(&self, scheduled_task: ScheduledTask) -> ();
}

pub trait Clock {
    fn now() -> Time;
}

trait Timer<Handle> {
  fn now(&self) -> Time;
  fn set_timer(&self, task_func: impl FnMut(), delay: Delay) -> Option<Handle>;
  fn clear_timer(&self, handle: Handle) -> ();
}

pub type TaskRunner = fn(ScheduledTask) -> ();

pub trait Timeline {
    fn add(&self, task: ScheduledTask) -> ();
    fn remove(&self, task: ScheduledTask) -> bool;
    fn is_empty(&self) -> bool;
    fn next_arrival(&self) -> Time;
    fn run_tasks(&self, time: Time, task_runner: TaskRunner) -> ();
}

pub trait Task: Dispose {
    fn run(&self, time: Time) -> ();
    fn error(&self, time: Time, error: impl Error) -> ();
  }

pub enum ScheduledTask {
    Task
}

pub trait Stream<A: Dispose> {
    fn run(&self, sink: impl Sink<A>, scheduler: impl Scheduler) -> Disposable<A>;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
