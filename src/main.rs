mod enigma;
mod alphabet_character;

struct EngimaMachine {
    rotors: enigma::rotors::Rotors
}

impl EngimaMachine {
    pub fn encrypt(&mut self, plaintext: &str) -> String {
        plaintext.chars().map(|c| self.rotors.encrypt_char(c)).collect()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        
    }
}