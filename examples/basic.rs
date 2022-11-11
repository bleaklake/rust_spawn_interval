use std::{thread, time::Duration};

fn on_tick() {
    println!("tick!");
}

fn main() {
    let cancel = spawn_interval::spawn_interval(&on_tick, Duration::from_millis(500));

    // Waiting before cancelling this instance of spawn_interval.
    thread::sleep(Duration::from_secs(3));

    cancel();

    println!("This instance of spawn_interval has been succesfully stopped");

    // Sleeping for a long time for the sake of this example.
    thread::sleep(Duration::from_millis(u64::MAX));
}
