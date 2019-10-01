use std::process::Command;

use clap::{App, Arg, SubCommand};

fn main() {
    let cmd = Command::new("ls")
        .arg("-l")
        .arg("-A")
        .arg("-h")
        .output()
        .unwrap();

    let stdout: String = String::from_utf8(cmd.stdout).unwrap();

    println!("{}", stdout);

    let matches = App::new("Command")
        .version("1.0")
        .author("Parker Timmerman")
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .value_name("NUMBER")
                .help("Takes a number")
                .takes_value(true),
        )
        .get_matches();
    
    let number = matches.value_of("number").unwrap();
    println!("Value for number: {}", number);
}
