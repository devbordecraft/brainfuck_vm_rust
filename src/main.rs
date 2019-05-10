extern crate clap;
use std::env;
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

    println!("{}", fuck);
}

fn read_fuck_file(path: &str) -> String{
    let contents = fs::read_to_string(&path).expect("Probleme de lecture du fichier");
    // Return the content
    contents
}