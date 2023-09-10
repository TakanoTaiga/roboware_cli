use roboware_cli::{help, check, env};

fn main() {
    check::zero_arg();
    help::call_arg_help();
    env::call_env();
}

