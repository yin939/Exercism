use std::fmt::{Display, Formatter, Result};

pub struct Roman(u32);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", int_to_roman(self.0))
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman(num)
    }
}

pub fn int_to_roman(mut num: u32) -> String {
    let romans = vec![
    (1000, "M"), (900,  "CM"), (500,  "D"), (400,  "CD"),
    (100,  "C"), (90,   "XC"), (50,   "L"), (40,   "XL"),
    (10,   "X"), (9,    "IX"), (5,    "V"), (4,    "IV"),
    (1,    "I")
  ];
    let mut rel = String::new();
    for (key, value) in romans {
        if num / key != 0 {
            let mut count = num / key;
            while count > 0 {
                rel.push_str(value);
                count -= 1;
            }
            num %= key;
            if num == 0 {
                return rel;
            }
        }
    }
    rel
}
