use crate::alphabet_character::AlphabetCharacter;

#[derive(Debug, PartialEq)]
pub enum RotorId {
    I,
    II,
    III,
    IV,
    V,
}

impl RotorId {
    /// The method in which the internal wiring connects the right side of the rotor to the left
    /// side.
    ///
    /// The letters are listed as connected to alphabet order.
    /// If the first letter of a rotor is E, this means that the A is wired to the E.
    /// This does not mean that E is wired to A; such looped wiring is only the case with the
    /// reflectors.
    pub fn wiring(&self) -> &str {
        match self {
            RotorId::I => "EKMFLGDQVZNTOWYHXUSPAIBRCJ",
            RotorId::II => "AJDKSIRUXBLHWTMCQGZNPYFVOE",
            RotorId::III => "BDFHJLCPRTXVZNYEIWGAKMUSQO",
            RotorId::IV => "ESOVPZJAYQUIRHXLNFTGKDCMWB",
            RotorId::V => "VZBRGITYUPSDNHLXAWMJQOFECK",
        }
    }

    /// The index on the rotor where the ratchet-and-pawl mechanism is actioned, advancing the
    /// next rotor in the set, once that character has been reached after looping around.
    pub fn turnover_index(&self) -> u32 {
        match self {
            RotorId::I => 23,   // R
            RotorId::II => 21,  // F
            RotorId::III => 17, // W
            RotorId::IV => 20,  // K
            RotorId::V => 16,   // A
        }
    }

    pub fn find_index_for_char(&self, alphabetic_character: AlphabetCharacter) -> Result<u32, String> {
        match self
            .wiring()
            .chars()
            .position(|char| char == alphabetic_character.character)
        {
            None => Err(format!(
                "Failed to find the index of alphabetic character: {}",
                alphabetic_character
            )),
            Some(index) => Ok(index as u32),
        }
    }
}

pub struct Rotor {
    pub rotor_id: RotorId,
    pub shift_value: u32,
}

impl Rotor {
    pub fn pre_reflector_encrypt(&self, character: AlphabetCharacter) -> Result<AlphabetCharacter, String> {        
        // take the character, increment by shift index, look up mapping for THAT character
        let character_alphabet_index = (character.as_u32() + self.shift_value) - 'A' as u32;

        match self.rotor_id.wiring().chars().nth(character_alphabet_index as usize) {
            Some(encrypted_character) => AlphabetCharacter::new(encrypted_character),
            None => Err(String::from("Failed to encrypt the character."))
        }
    }

    /// Find the index of the character, locate that in the standard A-Z alphabet.
    pub fn post_reflector_encrypt(&self, character: AlphabetCharacter) -> Result<AlphabetCharacter, String> {
        let character_alphabet_index = self.rotor_id.find_index_for_char(character).unwrap();

        let character = std::char::from_u32('A' as u32 + character_alphabet_index).unwrap();

        return AlphabetCharacter::new(character);
    }

    pub fn increment_shift_value(&mut self)  {
        const LAST_ALPHA_INDEX: u32 = 25;
        
        self.shift_value = (self.shift_value + 1) % LAST_ALPHA_INDEX;
    }

    pub fn new(rotor_id: RotorId, initial_character: Option<AlphabetCharacter>) -> Result<Self, String> {
        let shift_value: u32 = match initial_character {
            None => 0,
            Some(character) => {
                match rotor_id.find_index_for_char(character) {
                    Ok(index) => index,
                    Err(_reason) => 0
                }
            }
        };
        
        Ok(Self {
            rotor_id,
            shift_value
        })
    }
}

#[cfg(test)]
mod test {
    use crate::alphabet_character::AlphabetCharacter;
use crate::enigma::rotor::{Rotor, RotorId};

    // #[test]
    // fn new_returns_an_error_given_nonalphabetic_inital_value() {
    //     let rotor = Rotor::new(RotorId::I);
    // 
    //     assert!(rotor.is_err(), "rotor was successfully created.");
    // }

    #[test]
    fn new_creates_rotor_given_valid_rotor_and_no_initial_character() {
        let rotor = Rotor::new(RotorId::II, None).unwrap();

        assert_eq!(RotorId::II, rotor.rotor_id);
        assert_eq!(0, rotor.shift_value);
    }

    #[test]
    fn increment_increments_the_current_value_given_mid_char() {
        let mut rotor = Rotor::new(RotorId::II, None).unwrap();

        rotor.increment_shift_value();

        assert_eq!(1, rotor.shift_value);
    }

    #[test]
    fn increment_increments_the_current_value_given_end_char() {
        let mut rotor = Rotor::new(RotorId::II, None).unwrap();

        for _ in 0..25 { rotor.increment_shift_value(); }

        assert_eq!(0, rotor.shift_value);
    }

    #[test]
    fn pre_reflector_encrypt_character_wraps_around_to_select_correct_character_given_zero_shift_value() {
        let rotor = Rotor::new(RotorId::I, None).unwrap();

        let result = rotor.pre_reflector_encrypt(AlphabetCharacter::new('A').unwrap()).unwrap();

        assert_eq!('E', result.character);
    }
    
    #[test]
    fn pre_reflector_encrypt_character_selects_correct_character_given_positive_shift_value() {
        let mut rotor = Rotor::new(RotorId::I, None).unwrap();

        rotor.increment_shift_value();
        
        let result = rotor.pre_reflector_encrypt(AlphabetCharacter::new('A').unwrap()).unwrap();
        
        assert_eq!('K', result.character);
    }

    #[test]
    fn pre_reflector_encrypt_character_selects_correct_character_given_positive_shift_value_2() {
        let rotor = Rotor::new(RotorId::II, None).unwrap();

        let result = rotor.pre_reflector_encrypt(AlphabetCharacter::new('K').unwrap()).unwrap();

        assert_eq!('B', result.character);
    }

    #[test]
    fn post_reflector_encrypt_character() {
        let rotor = Rotor::new(RotorId::III, None).unwrap();

        let result = rotor.post_reflector_encrypt(AlphabetCharacter::new('H').unwrap()).unwrap();

        assert_eq!('D', result.character);
    }
}
