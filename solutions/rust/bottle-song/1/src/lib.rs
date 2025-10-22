pub fn recite(start_bottles: u32, take_down: u32) -> String {
    if start_bottles == 0 || take_down == 0 {
        return String::new();
    }

    fn num_word_lower(n: u32) -> &'static str {
        match n {
            10 => "ten",
            9 => "nine",
            8 => "eight",
            7 => "seven",
            6 => "six",
            5 => "five",
            4 => "four",
            3 => "three",
            2 => "two",
            1 => "one",
            0 => "no",
            _ => "many",
        }
    }

    fn capitalize(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    let mut song = String::new();

    for i in 0..take_down {
        let current = start_bottles - i;
        let current_word = capitalize(num_word_lower(current));
        let current_bottle = if current == 1 { "bottle" } else { "bottles" };

        song.push_str(&format!("{} green {} hanging on the wall,\n", current_word, current_bottle));
        song.push_str(&format!("{} green {} hanging on the wall,\n", current_word, current_bottle));
        song.push_str("And if one green bottle should accidentally fall,\n");

        let next = current.saturating_sub(1);
        let next_word = num_word_lower(next);
        let next_bottle = if next == 1 { "bottle" } else { "bottles" };
        song.push_str(&format!("There'll be {} green {} hanging on the wall.", next_word, next_bottle));

        if i + 1 < take_down {
            song.push_str("\n\n");
        }
    }

    song
}
