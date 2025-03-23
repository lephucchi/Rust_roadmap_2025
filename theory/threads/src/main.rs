use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn msg_hello() -> &'static str {
    "Hello, "
}

fn msg_name() -> &'static str {
    "My name is Chi, "
}

fn msg_inf() -> &'static str {
    "Cmaj7 "
}

fn main() {
    let hello: JoinHandle<&'static str> = thread::spawn(move || {
        msg_hello()
    });

    let name: JoinHandle<&'static str> = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        msg_name()
    });

    let inf: JoinHandle<&'static str> = thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        msg_inf()
    });

    let hello = hello.join().expect("hello err");
    let name = name.join().expect("name err");
    let inf = inf.join().expect("inf err");

    println!("{}{}{}", hello, name, inf);
}