use colored::*;

pub fn zero_arg(){
    if std::env::args().len() == 1 {
        println!("{}", "no arg given".red());
        std::process::exit(0);
    }
}