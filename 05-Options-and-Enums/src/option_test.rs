// option is a enum that is used as a wrapper on a data type to handle better
// the handles are i) None for null  ii) Some to enter some value of that type specified in Option<generic>
pub fn test_option_type() -> Option<u8> {
    // if use this None we wont be able to unwrap it as null is null
    // let opt1 : Option<u8> = None; // none is like null
    let opt1: Option<u8> = Some(222);
    return opt1;
}

pub fn test_option_string() -> Option<String> {
    #[allow(unused_assignments)]
    let mut opt1: Option<String> = None;
    opt1 = Some(String::from("ffddd"));
    return opt1;
}

pub fn test_option_charactertype(x: u8) -> Option<CharacterType> {
    #[allow(unused_assignments)]
    let mut opt1: Option<CharacterType> = None;
    match x {
        0 => opt1 = Some(CharacterType::Archer),
        ..4 => opt1 = Some(CharacterType::Shooter),
        ..=8 => opt1 = Some(CharacterType::Warrior),
        _ => {
            opt1 = Some(CharacterType::Null);
            std::println!("This is not in range");
        }
    }
    // return opt1;
    opt1 // this means that the value opt1 of type Option<CharacterType> is returned
}

pub enum CharacterType {
    Warrior,
    Archer,
    Shooter,
    Null,
}

// that is a std lib trait which we have modified to work with our custom data type
impl ToString for CharacterType {
    fn to_string(&self) -> String {
        match self {
            CharacterType::Archer => "Archer".to_string(),
            CharacterType::Null => "Null".to_string(),
            CharacterType::Shooter => "Shooter".to_string(),
            CharacterType::Warrior => String::from("Warrior"),
        }
    }
}
