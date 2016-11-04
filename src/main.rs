extern crate rustc_serialize;
extern crate docopt;
extern crate nix;
#[macro_use]
extern crate chan;
extern crate chan_signal;
#[macro_use]
extern crate log;
extern crate env_logger;

use docopt::Docopt;
use nix::unistd::*;
use chan_signal::{Signal, notify};
use std::path::Path;
mod utils;

const USAGE: &'static str = "
Daemon Sample.

Usage:
  daemon_sample_rs start [--pidfile=<pidfile>]
  daemon_sample_rs stop
  nc_rec (-h | --help)
  nc_rec --version

Options:
  -h --help              Show this screen.
  --version              Show version.
  --pidfile=<pidfile>    Process ID file [default: /tmp/daemon_sample_rs.pid].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_pidfile: String,
    cmd_start: bool,
    cmd_stop: bool,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    env_logger::init().unwrap();
    let pidfile = Path::new(&args.flag_pidfile);

    if args.cmd_start {
        match fork() {
            Ok(result) => {
                match result {
                    ForkResult::Parent { child } => {
                        info!("Start daemon process pid={}", child);
                        if let Err(e) = utils::write_pid(&pidfile, child) {
                            info!("Err: {}", e);
                        }
                    }
                    ForkResult::Child => {
                        let signal = notify(&[Signal::INT, Signal::TERM]);
                        run(signal);
                    }
                }
            }
            Err(_) => error!("Cannot fork child."),
        }
    } else if args.cmd_stop {
        let pid = utils::read_pid(&pidfile).unwrap();
        info!("Try to kill daemon process pid={}", pid);
        if utils::is_pid_exists(pid) {
            nix::sys::signal::kill(pid, nix::sys::signal::SIGTERM).unwrap();
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
