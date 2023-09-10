// use colored::*;

pub fn call_arg_help(){
    let fist_arg = std::env::args().nth(1).unwrap();
    if "-h" == fist_arg || "--help" == fist_arg {
        show_help();
    }
}

fn show_help(){
    println!("arg list\n -h");
}