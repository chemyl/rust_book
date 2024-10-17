trait Attacker {
    fn choose_style(&self) -> String;
    fn choose_weapon(&self) -> String;
}

#[derive(Debug)]
enum Character {
    Warrior,
    Archer,
    Wizard,
}

impl Attacker for Character {
    fn choose_weapon(&self) -> String {
        match self {
            Character::Archer => "Bowl".to_string(),
            Character::Warrior => "Sword".to_string(),
            Character::Wizard => "Magic".to_string(),
        }
    }

    fn choose_style(&self) -> String {
        match self {
            Character::Archer => "Speed".to_string(),
            Character::Warrior => "Power".to_string(),
            Character::Wizard => "Magic".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn traits_test() {
        let warrior: Character = Character::Warrior;
        let fight_style = warrior.choose_style();
        let fight_weapon = warrior.choose_weapon();
        dbg!(fight_style);
        dbg!(fight_weapon);
    }
}
