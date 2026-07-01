use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Instant, SystemTime};
use crate::memory::Value;

mod memory;

fn main() {
    let mem = Arc::new(Mutex::new(memory::init_memory()));

    memory::run_command("SET x 1", &mut mem.lock().unwrap());
    memory::run_command("SET y 2", &mut mem.lock().unwrap());
    memory::run_command("GET x", &mut mem.lock().unwrap());
    memory::run_command("GET y", &mut mem.lock().unwrap());
    memory::run_command("DEL x", &mut mem.lock().unwrap());
    memory::run_command("GET x", &mut mem.lock().unwrap());

    sleep(std::time::Duration::from_secs(5));


    memory::run_command("GET y", &mut mem.lock().unwrap());

    thread::spawn(move || {
        loop {
            sleep(std::time::Duration::from_secs(10));
            evict_expired_keys(&mut mem.lock().unwrap());
        }
    });

}


fn evict_expired_keys(mem: &mut HashMap<String, (Value, Instant)>) {
    let mut now = Instant::now();
    mem.retain(|_, (_, expire_time)| expire_time > &mut now);
}