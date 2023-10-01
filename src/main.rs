mod args;
mod cmd;
mod benchmark;

fn main() {
    env_logger::init();

    let args = args::parse_args();

    let cmd = cmd::create_command(args.exec_path, args.exec_args);

    benchmark::start_benchmark(cmd);
}