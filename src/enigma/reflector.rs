#[derive(Debug, PartialEq)]
pub enum Reflector {
    A,
    B,
    C
}

impl Reflector {
    /// The method in which the internal wiring connects the incoming signal to the outgoing signal.
    ///
    /// The letters are listed as connected to alphabet order.
    /// If the first letter of a reflector is E, this means that the A is wired to the E.
    /// This does not mean that E is wired to A; that's actually a vulnerability of the Enigma machine.
    pub fn wiring(&self) -> &str {
        match self {
            Reflector::A => "EJMZALYXVBWFCRQUONTSPIKHGD",
            Reflector::B => "YRUHQSLDPXNGOKMIEBFZCWVJAT",
            Reflector::C => "FVPJIAOYEDRZXWGCTKUQSBNMHL"
        }
    }

    pub fn reflect_character(&self, character: char) -> Result<char, &str> {
        // take the character, increment by shift index, look up mapping for THAT character
        let character_alphabet_index = character as u32 - 'A' as u32;


        match self.wiring().chars().nth(character_alphabet_index as usize) {
            None => Err("Failed to find reflector character."),
            Some(reflected_character) => Ok(reflected_character)

        }
    }
}

#[cfg(test)]
mod tests {
    use crate::enigma::reflector::Reflector;

    #[test]
    fn wiring_returns_correct_value() {
        assert_eq!("EJMZALYXVBWFCRQUONTSPIKHGD", Reflector::A.wiring())
    }

    #[test]
    fn reflect_character_returns_err_given_non_alphabetic() {
        assert!(Reflector::B.reflect_character('[').is_err())
    }

    #[test]
    fn reflect_character_returns_correct_character() {
        assert_eq!('R', Reflector::B.reflect_character('B').unwrap())
    }
}