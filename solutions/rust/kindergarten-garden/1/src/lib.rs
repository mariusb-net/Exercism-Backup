pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    // Ordered list of students (alphabetical)
    let students = [
        "Alice",
        "Bob",
        "Charlie",
        "David",
        "Eve",
        "Fred",
        "Ginny",
        "Harriet",
        "Ileana",
        "Joseph",
        "Kincaid",
        "Larry",
    ];

    // Determine student's index (0-based). If not found, return empty vector.
    let idx = match students.iter().position(|&s| s == student) {
        Some(i) => i,
        None => return Vec::new(),
    };

    // Expect two rows; split by lines and trim possible whitespace.
    let rows: Vec<&str> = diagram.lines().collect();
    if rows.len() < 2 {
        return Vec::new();
    }
    let r1 = rows[0].trim();
    let r2 = rows[1].trim();

    let start = idx * 2;
    let mut stplants: Vec<&'static str> = Vec::with_capacity(4);

    for row in [r1, r2].iter() {
        // take the two characters for this student from the row
        for ch in row.chars().skip(start).take(2) {
            let plant = match ch {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                'V' => "violets",
                _ => "",
            };
            stplants.push(plant);
        }
    }

    stplants
}
