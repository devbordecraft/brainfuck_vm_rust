extern crate clap;

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

    let mut fuck_state = BrainfuckState {
        current_vector_memory_state: vec![0 ;1],
        current_index: 0,
    };

    fuck_state.compute(&fuck);
    println!("Executed ! ");

    println!("State of the VM memory : ");
    fuck_state.print_brainfuck_memory();
}

struct BrainfuckState {
    current_vector_memory_state: Vec<u8>,
    current_index: i32,
}

impl BrainfuckState {
    fn compute(&mut self, fuck_to_compute: &String) {
        let mut cpt:usize = 0;
        while cpt < fuck_to_compute.len(){
            let ch = fuck_to_compute[cpt..].chars().next().unwrap();
            //println!("Current char is {} and current index is {}", ch, self.current_index);
            //self.print_brainfuck_memory();
            match ch.to_string().as_str(){
                "+" => {
                    // Incremente la donnée de l'index courant
                    self.current_vector_memory_state[self.current_index as usize] += 1;
                }
                "-" => {
                    // Decremente la donnée de l'index courant
                    self.current_vector_memory_state[self.current_index as usize] -= 1;
                }
                ">" => {
                    // Incremente l'index courant
                    self.current_vector_memory_state.push(0);
                    self.current_index += 1;
                }
                "<" => {
                    // Decremente l'index courant
                    self.current_index -= 1;
                }
                "." => {
                    // Affiche la donnée de l'index courant
                    print!("{}", self.current_vector_memory_state[self.current_index as usize] as char);
                }
                "[" => {
                    if self.current_vector_memory_state[self.current_index as usize] == 0 {
                        // saute à l'instruction après le ] correspondant si l'octet pointé est à 0.
                        //println!("Cpt avant {:?}", fuck_to_compute.find("]"));
                        cpt += fuck_to_compute[cpt..fuck_to_compute.len()].find("]").unwrap();
                        continue;
                    }
                }
                "]" => {
                    if self.current_vector_memory_state[self.current_index as usize] != 0 {
                        //retourne à l'instruction après le [ si l'octet pointé est différent de 0.
                        //println!("JE veux reculer de {} en arriere ", cpt - fuck_to_compute[0..cpt].rfind("[").unwrap());
                        cpt -= cpt - fuck_to_compute[0..cpt].rfind("[").unwrap();

                        continue;
                    }
                }
                _ => {
                    println!("Symbol cannot be handled");
                }
            }
            cpt += 1;
        }
    }
    fn print_brainfuck_memory(&self) {
        println!("{:?}", self.current_vector_memory_state)
    }
}

fn read_fuck_file(path: &str) -> String {
    let contents = fs::read_to_string(&path).expect("Probleme de lecture du fichier");
    // Return the content
    contents
}