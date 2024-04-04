use std::collections::{BTreeMap, BTreeSet};

type Grade = u32;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not
// the point of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    grade_to_names: BTreeMap<Grade, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grade_to_names: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: Grade, student: &str) {
        self.grade_to_names
            .entry(grade)
            .or_insert_with(|| BTreeSet::from([student.to_string()]))
            .insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grade_to_names.keys().cloned().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a
    // `Vec<String>` internally to lend out. By returning an owned vector of owned
    // `String`s instead, the internal structure can be completely arbitrary. The
    // tradeoff is that some data must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grade_to_names
            .get(&grade)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_else(|| vec![])
    }
}
