use std::collections::{BTreeMap, HashSet};

pub struct School {
    grades: BTreeMap<u32, Vec<String>>,
    all_students: HashSet<String>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: BTreeMap::new(),
            all_students: HashSet::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        // Check if student is already in the roster (any grade)
        if self.all_students.contains(student) {
            return;
        }

        // Add student to the all_students set
        self.all_students.insert(student.to_string());

        // Add student to the grade
        self.grades
            .entry(grade)
         //   .or_insert_with(Vec::new)
            .or_default()
            .push(student.to_string());

        // Sort students in the grade
        self.grades.get_mut(&grade).unwrap().sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades
            .get(&grade)
            .map(|students| students.clone())
            .unwrap_or_default()
    }
}
