use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

// spawn_interval
pub fn spawn_interval<F>(cb: &'static F, wait_time: Duration) -> Box<dyn Fn() + Send + 'static>
where
    F: Fn() -> () + Sync,
{
    let mut start = Instant::now();

    let has_stopped = Arc::new(AtomicBool::new(false));
    let has_stopped_cloned = has_stopped.clone();

    thread::spawn(move || loop {
        let runtime = start.elapsed();

        if let Some(remaining) = wait_time.checked_sub(runtime) {
            thread::sleep(remaining);
        }

        start = Instant::now();

        if has_stopped_cloned.load(Ordering::Relaxed) {
            return;
        }

        thread::spawn(cb);
    });

    Box::new(move || {
        has_stopped.store(true, Ordering::Relaxed);
    })
}
