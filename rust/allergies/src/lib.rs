use std::collections::HashSet;

pub struct Allergies {
    allergies: HashSet<Allergen>,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut score = score;
        while score > 2_u32.pow(8) {
            score -= 2_u32.pow(8);
        }
        let mut allergies = HashSet::new();
        for i in (0..=7).rev() {
            let temp = 2_u32.pow(i);
            if score.checked_sub(temp).is_some() {
                let allergen = match temp {
                    1 => Allergen::Eggs,
                    2 => Allergen::Peanuts,
                    4 => Allergen::Shellfish,
                    8 => Allergen::Strawberries,
                    16 => Allergen::Tomatoes,
                    32 => Allergen::Chocolate,
                    64 => Allergen::Pollen,
                    128 => Allergen::Cats,
                    _ => Allergen::Eggs,
                };
                allergies.insert(allergen);
                score -= temp;
            }
        }

        Allergies { allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(&allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.iter().cloned().collect::<Vec<Allergen>>()
    }
}
