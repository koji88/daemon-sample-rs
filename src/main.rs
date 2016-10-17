extern crate nix;
#[macro_use] extern crate chan;
extern crate chan_signal;

use nix::unistd::*;
use chan_signal::{Signal,notify};

fn main() {
    match fork().expect("fork faild") {
        ForkResult::Parent{child} => {
            println!("Start Daemon Process pid={}",child);
        }
        ForkResult::Child =>{
            let signal = notify(&[Signal::INT, Signal::TERM]);
            run(signal);
        }
    }
}

fn run(signal: chan::Receiver<Signal>) {
    println!("Child process is started.");
    loop {
        chan_select! {
            default => {
                println!("child: sleep 3sec...");
                sleep(3);
            },
            signal.recv() => break,
        }
    }
    println!("Child process is terminated.");
}

