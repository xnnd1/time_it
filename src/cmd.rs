use std::process::Command;

use log::{info};

pub fn create_command(exec_path: String, exec_args: Option<Vec<String>>) -> Command {
    let mut cmd = Command::new(exec_path);

    match exec_args {
        Some(_) => cmd.args(exec_args.unwrap().to_vec()),
        None => &mut cmd,
    };

    info!("Command values: {:?}", cmd);

    cmd
}