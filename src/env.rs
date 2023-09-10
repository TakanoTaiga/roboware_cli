use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

pub fn call_env(){
    let fist_arg = std::env::args().nth(1).unwrap();
    let secound_arg = std::env::args().nth(2).unwrap();
    if "env" != fist_arg {return;}
        
    if "build" == secound_arg {
        env_build();
    }else{
        
    }
}

fn env_build() {
    let mut child = Command::new("colcon")
        .arg("build")
        .arg("--symlink-install")
        .arg("--cmake-args")
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("--cargo-args")
        .arg("--release")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    if let Some(ref mut stdout) = child.stdout {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
    }

    if let Some(ref mut stderr) = child.stderr {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            eprintln!("Error: {}", line.unwrap());
        }
    }

    let status = child.wait().expect("Failed to wait on child");
    if !status.success() {
        eprintln!("Command exited with error: {:?}", status);
    }
}
