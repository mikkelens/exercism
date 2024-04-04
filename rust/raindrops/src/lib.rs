struct Sounds {
    num: u32,
    s: Option<String>,
}
impl Sounds {
    fn new(num: u32) -> Self {
        Sounds { num, s: None }
    }

    fn add(mut self, comparison: fn(u32) -> bool, addition: &str) -> Self {
        if comparison(self.num) {
            self.s.get_or_insert(String::new()).push_str(addition);
        }
        self
    }

    fn finalize(self) -> String {
        match self.s {
            None => self.num.to_string(),
            Some(inner) => inner,
        }
    }
}

pub fn raindrops(n: u32) -> String {
    Sounds::new(n)
        .add(|num| num % 3 == 0, "Pling")
        .add(|num| num % 5 == 0, "Plang")
        .add(|num| num % 7 == 0, "Plong")
        .finalize()
}
