use std::time::Instant;
use std::process::Command;

use log::{info};

pub fn start_benchmark(mut cmd: Command) {
    let now = Instant::now();
    let mut child = cmd.spawn().expect("Couldn't spawn child!");

    info!("Child values: {:?}", child);

    match child.try_wait() {
        Ok(Some(status)) => println!("Exited with: {status}"),
        Ok(None) => {
            let _ = child.wait();
            let exec_duration = now.elapsed();
            println!("{:?}", exec_duration);
        }
        Err(e) => println!("Error attempting to wait: {e}"),
    }    
}