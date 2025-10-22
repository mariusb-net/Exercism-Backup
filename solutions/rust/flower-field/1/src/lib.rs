pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    let grid: Vec<Vec<char>> = garden.iter().map(|r| r.chars().collect()).collect();
    let rows = grid.len();

    fn is_flower(c: char) -> bool {
        c == '*'
    }

    let mut annotated = Vec::with_capacity(rows);

    for i in 0..rows {
        let cols = grid[i].len();
        let mut row = String::with_capacity(cols);
        for j in 0..cols {
            let c = grid[i][j];
            if is_flower(c) {
                row.push(c);
                continue;
            }

            let mut count = 0;
            for di in [-1i32, 0, 1] {
                for dj in [-1i32, 0, 1] {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && (ni as usize) < rows {
                        let ni_us = ni as usize;
                        if nj >= 0 && (nj as usize) < grid[ni_us].len() {
                            if is_flower(grid[ni_us][nj as usize]) {
                                count += 1;
                            }
                        }
                    }
                }
            }

            if count == 0 {
                row.push(' ');
            } else {
                row.push(std::char::from_digit(count, 10).unwrap());
            }
        }
        annotated.push(row);
    }

    annotated
}
