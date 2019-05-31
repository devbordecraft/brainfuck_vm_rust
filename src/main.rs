extern crate clap;

mod brainfuck_struct;

use std::fs;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Brainfuck VM")
        .version("0.1.0")
        .author("Valentin SAVREUX <valentin.savreux@protonmail.com>")
        .about("Interpreteur du langage Brainfuck")
        .arg(Arg::with_name("fuckfile")
            .help("Sets the brainfuck file to execute")
            .required(true)
            .index(1))
        .get_matches();

    println!("Using brainfuck file: {}", matches.value_of("fuckfile").unwrap());

    let path = matches.value_of("fuckfile").unwrap();

    let fuck = read_fuck_file(&path);

    println!("Execution of : {}", fuck);

    let mut fuck_state = brainfuck_struct::BrainfuckState {
        current_vector_memory_state: vec![0 ;1],
        current_index: 0,
    };

    fuck_state.compute(&fuck);
    println!("Executed ! ");

    println!("State of the VM memory : ");
    fuck_state.print_brainfuck_memory();
}

fn read_fuck_file(path: &str) -> String {
    let contents = fs::read_to_string(&path).expect("Probleme de lecture du fichier");
    // Return the content
    contents
}
