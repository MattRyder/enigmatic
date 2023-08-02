use crate::alphabet_character::AlphabetCharacter;
use crate::enigma::reflector::Reflector;
use crate::enigma::rotor::Rotor;

pub struct Rotors {
    pub left_rotor: Rotor,
    pub middle_rotor: Rotor,
    pub right_rotor: Rotor,
    pub reflector: Reflector,
}

impl Rotors {
    pub fn new(
        reflector: Reflector,
        left_rotor: Rotor,
        middle_rotor: Rotor,
        right_rotor: Rotor,
    ) -> Result<Self, ()> {
        let rotors_unique = (&left_rotor.rotor_id != &middle_rotor.rotor_id)
            && (&middle_rotor.rotor_id != &right_rotor.rotor_id)
            && (&left_rotor.rotor_id != &right_rotor.rotor_id);

        if rotors_unique {
            Ok(Self {
                reflector,
                left_rotor,
                middle_rotor,
                right_rotor,
            })
        } else {
            Err(())
        }
    }

    fn pre_reflector(&mut self, character: AlphabetCharacter) -> AlphabetCharacter {
        self.right_rotor.increment_shift_value();

        let right_encrypt = self.right_rotor.pre_reflector_encrypt(character).unwrap();
        
        println!("Right rotor encrypted {} to {}", character, right_encrypt);
        
        // if self.right_rotor.shift_value == self.right_rotor.rotor_id.turnover_index() {
        //     self.middle_rotor.increment_shift_value();
        // }

        let shifted_char = std::char::from_u32((right_encrypt.as_u32()) - self.right_rotor.shift_value).unwrap();

        let middle_encrypt = self.middle_rotor.pre_reflector_encrypt(shifted_char).unwrap();

        println!("Middle rotor encrypted {} to {}", shifted_char, middle_encrypt);

        if self.middle_rotor.shift_value == self.middle_rotor.rotor_id.turnover_index() {
            self.left_rotor.increment_shift_value();
        }

        let mid_shifted_char = std::char::from_u32(middle_encrypt.as_u32() - self.middle_rotor.shift_value).unwrap();

        let left_encrypt = self.left_rotor.pre_reflector_encrypt(mid_shifted_char).unwrap();

        println!("Left rotor encrypted {} to {}", mid_shifted_char, left_encrypt);

        left_encrypt
    }

    fn post_reflector(&mut self, reflected_char: AlphabetCharacter) -> AlphabetCharacter {
        let left = self.left_rotor.post_reflector_encrypt(reflected_char).unwrap();
        println!("Coming back... Left rotor encrypted {} to {}", reflected_char, left);

        let middle = self.middle_rotor.post_reflector_encrypt(left).unwrap();
        println!("Coming back... Middle rotor encrypted {} to {}", left, middle);

        let right = self.right_rotor.post_reflector_encrypt(left).unwrap();
        let right_shifted = std::char::from_u32(right.as_u32() - self.right_rotor.shift_value).unwrap();
        println!("Coming back... Right rotor encrypted {} to {}", left, right_shifted);

        right
    }

    pub fn encrypt_char(&mut self, character: AlphabetCharacter) -> AlphabetCharacter {
        let pre_reflector_encrypt_char = self.pre_reflector(character);

        let reflected_char = self.reflector.reflect_character(pre_reflector_encrypt_char).unwrap();

        println!("Reflected {} to {}", character, reflected_char);

        let post_reflector_encrypt_char = self.post_reflector(reflected_char);

        post_reflector_encrypt_char
    }
}

#[cfg(test)]
mod tests {
    use crate::alphabet_character::AlphabetCharacter;
use crate::enigma::reflector::Reflector;
    use crate::enigma::rotor::{Rotor, RotorId};
    use crate::enigma::rotors::Rotors;

    #[test]
    fn new_returns_err_given_duplicate_rotors() {
        let left_rotor = Rotor::new(RotorId::I, None).unwrap();
        let mid_rotor = Rotor::new(RotorId::II, None).unwrap();
        let right_rotor = Rotor::new(RotorId::II, None).unwrap();

        let result = Rotors::new(Reflector::A, left_rotor, mid_rotor, right_rotor);

        assert!(result.is_err());
    }

    #[test]
    fn new_returns_rotors_given_valid_configuration() {
        let left_rotor = Rotor::new(RotorId::I, None).unwrap();
        let mid_rotor = Rotor::new(RotorId::II, None).unwrap();
        let right_rotor = Rotor::new(RotorId::III, None).unwrap();

        let result = Rotors::new(Reflector::A, left_rotor, mid_rotor, right_rotor).unwrap();
        
        assert_eq!(Reflector::A, result.reflector);
    }

    #[test]
    fn pre_reflector_encrypt_returns_correct_value_given_valid_char() {
        let left_rotor = Rotor::new(RotorId::III, None).unwrap();
        let mid_rotor = Rotor::new(RotorId::II, None).unwrap();
        let right_rotor = Rotor::new(RotorId::I, None).unwrap();

        let mut rotors = Rotors::new(Reflector::B, left_rotor, mid_rotor, right_rotor).unwrap();
        
        let result = rotors.pre_reflector(AlphabetCharacter::new('A').unwrap());
        
        assert_eq!('D', result.character);
    }

    #[test]
    fn post_reflector_encrypt_returns_correct_value_given_valid_char() {
        let left_rotor = Rotor::new(RotorId::III, None).unwrap();
        let mid_rotor = Rotor::new(RotorId::II, None).unwrap();
        let right_rotor = Rotor::new(RotorId::I, None).unwrap();

        let mut rotors = Rotors::new(Reflector::B, left_rotor, mid_rotor, right_rotor).unwrap();
        
        let result = rotors.post_reflector(AlphabetCharacter::new('H').unwrap());
        
        assert_eq!('G', result.character);
    }

    #[test]
    fn reflect_returns_correct_value_given_valid_char() {
        let left_rotor = Rotor::new(RotorId::III, None).unwrap();
        let mid_rotor = Rotor::new(RotorId::II, None).unwrap();
        let right_rotor = Rotor::new(RotorId::I, None).unwrap();

        let mut rotors = Rotors::new(Reflector::B, left_rotor, mid_rotor, right_rotor).unwrap();
        
        let result = rotors.pre_reflector(AlphabetCharacter::new('A').unwrap());
        
        assert_eq!('D', result.character);
    }
    
    #[test]
    fn encrypt_character_encrypts_to_the_correct_value() {
        let left_rotor = Rotor::new(RotorId::III, None).unwrap();
        let mid_rotor = Rotor::new(RotorId::II, None).unwrap();
        let right_rotor = Rotor::new(RotorId::I, None).unwrap();

        let mut rotors = Rotors::new(Reflector::B, left_rotor, mid_rotor, right_rotor).unwrap();
        
        let result = rotors.encrypt_char(AlphabetCharacter::new('A').unwrap());
        
        assert_eq!('G', result.character);
    }
}
