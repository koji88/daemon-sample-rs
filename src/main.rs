extern crate nix;
#[macro_use] extern crate chan;
extern crate chan_signal;
#[macro_use] extern crate log;
extern crate env_logger;

use nix::unistd::*;
use chan_signal::{Signal,notify};

fn main() {
    env_logger::init().unwrap();
    
    match fork() {
        Ok(result) => match result {
            ForkResult::Parent{child} => {
                info!("Start Daemon Process pid={}",child);
            }
            ForkResult::Child =>{
                let signal = notify(&[Signal::INT, Signal::TERM]);
                run(signal);
            }
        },
        Err(_) => {
            error!("Cannot fork child.")
        }
    }
}

fn run(signal: chan::Receiver<Signal>) {
    info!("Child process is started.");
    loop {
        chan_select! {
            default => {
                info!("child: sleep 3sec...");
                sleep(3);
            },
            signal.recv() => break,
        }
    }
    info!("Child process is terminated.");
}

