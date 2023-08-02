use std::fmt::{Display};
use std::ops::Range;

#[derive(PartialEq)]
pub struct AlphabetCharacter {
    pub character: char,
}

impl Display for AlphabetCharacter {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(format!("{}", self.character))
    }
}

impl AlphabetCharacter {
    pub fn as_u32(&self) -> u32 {
        self.character as u32
    }

    pub fn new(character: char) -> Result<Self, String> {
        const A_TO_Z_CHARS: Range<u32> = 65..90;

        if !character.is_alphabetic() {
            return Err(String::from("Character provided is not alphabetic."));
        }

        let uppercase_character = character.to_ascii_uppercase();
        let char_numeric_value = uppercase_character as u32;

        if !A_TO_Z_CHARS.contains(&char_numeric_value) {
            return Err(String::from("Character provided is not between A..Z."));
        }

        Ok(Self { character })
    }
}

#[cfg(test)]
mod tests {
    use crate::alphabet_character::AlphabetCharacter;
    #[test]
    fn it_creates_an_instance_given_valid_character() {
        assert!(AlphabetCharacter::new('A').is_ok())
    }

    #[test]
    fn it_returns_an_error_given_invalid_character() {
        assert!(AlphabetCharacter::new('0').is_err())
    }
}
