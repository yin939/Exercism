use std::ops::Add;

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: Default + PartialOrd + Add<Output = T> + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let a = &sides[0];
        let b = &sides[1];
        let c = &sides[2];
        if *a + *b <= *c || *a + *c <= *b || *b + *c <= *a {
            return None;
        }
        Some(Triangle {
            a: *a,
            b: *b,
            c: *c,
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.c == self.b && self.a == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.a != self.c && self.b != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        // unimplemented!("Determine if the Triangle is isosceles.");
        self.a == self.b || self.a == self.c || self.b == self.c
    }
}
