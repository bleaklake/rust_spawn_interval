use std::{thread, time::Duration};

fn main() {
    let on_tick = || {
        println!("tick!");
    };

    // Leaking this inner function to make its lifetime as 'static.
    // https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak
    let static_on_tick = Box::leak(Box::new(on_tick));

    let cancel = spawn_interval::spawn_interval(static_on_tick, Duration::from_millis(500));

    // Waiting before cancelling this instance of spawn_interval.
    thread::sleep(Duration::from_secs(3));

    cancel();

    println!("This instance of spawn_interval has been succesfully stopped");

    // Sleeping for a long time for the sake of this example.
    thread::sleep(Duration::from_millis(u64::MAX));
}
