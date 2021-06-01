use std::{cell::RefCell, collections::HashSet};

use rand::prelude::*;

thread_local! {
    static NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}
#[derive(Debug, Default)]
pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        Self(gen_name())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        NAMES.with(|names| names.borrow_mut().remove(&self.0));
        self.0 = gen_name()
    }
}

fn gen_name() -> String {
    let mut rng = thread_rng();
    NAMES
        .with(|names| {
            for _ in 0..100 {
                let name: String = (0..5)
                    .map(|f| match f {
                        0..=1 => rng.gen_range(b'A'..=b'Z') as char,
                        _ => rng.gen_range(b'0'..=b'9') as char,
                    })
                    .collect();

                if names.borrow_mut().insert(name.clone()) {
                    return Some(name);
                }
            }
            None
        })
        .unwrap_or_else(|| panic!("No unique names"))
}
