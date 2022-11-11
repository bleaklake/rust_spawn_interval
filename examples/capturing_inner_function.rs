use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    // The data shared with the given callback has to be thread-safe.
    // https://doc.rust-lang.org/nomicon/send-and-sync.html
    let counter = Arc::new(Mutex::new(10));
    let cancel_option: Arc<Mutex<Option<Box<dyn Fn() + Send + 'static>>>> =
        Arc::new(Mutex::new(None));
    let cancel_option_clone = cancel_option.clone();

    let on_tick = move || {
        let mut c = counter.lock().unwrap();
        if *c == 0 {
            cancel_option_clone.lock().unwrap().as_ref().unwrap()();
            println!("We have a liftoff!");
            println!("This instance of spawn_interval has been succesfully stopped");
        } else {
            println!("{}", c);
            *c -= 1;
        }
    };

    // Leaking this inner function to make its lifetime as 'static.
    // https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak
    let static_on_tick = Box::leak(Box::new(on_tick));

    let cancel = spawn_interval::spawn_interval(static_on_tick, Duration::from_secs(1));

    *cancel_option.lock().unwrap() = Some(cancel);

    // Sleeping for a long time for the sake of this example.
    thread::sleep(Duration::from_millis(u64::MAX));
}
