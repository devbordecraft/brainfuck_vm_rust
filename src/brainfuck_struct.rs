pub struct BrainfuckState {
    pub current_vector_memory_state: Vec<u8>,
    pub current_index: i32,
}

impl BrainfuckState {
    pub fn compute(&mut self, fuck_to_compute: &String) {
        let mut cpt: usize = 0;
        while cpt < fuck_to_compute.len() {
            let ch = fuck_to_compute[cpt..].chars().next().unwrap();
            match ch.to_string().as_str() {
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
    pub fn print_brainfuck_memory(&self) {
        println!("{:?}", self.current_vector_memory_state)
    }
}