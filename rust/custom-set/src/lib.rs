#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    values: Vec<T>,
}

impl<T> CustomSet<T>
where
    T: Clone + Copy + PartialEq + Ord,
{
    pub fn new(input: &[T]) -> Self {
        let mut input = input.to_vec();
        input.sort_unstable();
        Self { values: input }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.values.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.values.contains(&element) {
            self.values.push(element);
            self.values.sort_unstable();
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.values.iter().all(|a| other.values.contains(a))
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        for o in other.values.iter() {
            if self.values.contains(o) {
                return false;
            }
        }
        true
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            values: self
                .values
                .iter()
                .filter(|&a| other.values.contains(a))
                .copied()
                .collect(),
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self {
            values: self
                .values
                .iter()
                .filter(|a| !other.values.contains(a))
                .copied()
                .collect(),
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut rel = Self::new(&self.values);
        other.values.iter().cloned().for_each(|e| rel.add(e));

        rel
    }
}
