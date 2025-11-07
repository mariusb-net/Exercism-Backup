#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}
pub struct BowlingGame {
    rolls: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { rolls: Vec::with_capacity(21) }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // Basic bounds
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.is_complete() {
            return Err(Error::GameComplete);
        }

        // Determine state entering 10th frame
        let mut idx = 0usize;
        // process first 9 frames
        for _frame in 0..9 {
            if idx >= self.rolls.len() {
                // we're at start of a non-10th frame
                // if this is the first roll of the frame, any pins 0..=10 ok
                // second roll rules enforced below when adding the second roll
                self.rolls.push(pins);
                return Ok(());
            }
            let r = self.rolls[idx];
            if r == 10 {
                idx += 1; // strike frame uses one roll
            } else {
                idx += 1;
                if idx < self.rolls.len() {
                    idx += 1; // second roll present
                } else {
                    // second roll missing -> we are in the middle of a frame
                    // validate sum
                    if r + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                    self.rolls.push(pins);
                    return Ok(());
                }
            }
        }

        // Now idx is the start index of 10th frame in rolls
        let tenth = &self.rolls[idx..];

        match tenth.len() {
            0 => {
                // first roll of 10th
                self.rolls.push(pins);
                Ok(())
            }
            1 => {
                let r1 = tenth[0];
                if r1 == 10 {
                    // bonus rolls allowed; second can be 0..=10
                    self.rolls.push(pins);
                    Ok(())
                } else {
                    // second roll must satisfy r1 + pins <= 10
                    if r1 + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                    self.rolls.push(pins);
                    Ok(())
                }
            }
            2 => {
                let r1 = tenth[0];
                let r2 = tenth[1];
                if r1 == 10 {
                    // first was strike -> always allowed a third roll
                    // but if second was not strike, then second + third must be <= 10
                    if r2 != 10 && r2 + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                    self.rolls.push(pins);
                    Ok(())
                } else if r1 + r2 == 10 {
                    // spare -> one bonus roll, can be 0..=10
                    self.rolls.push(pins);
                    Ok(())
                } else {
                    // frame already complete (open frame) - no third roll
                    Err(Error::GameComplete)
                }
            }
            _ => {
                // 3 or more rolls in 10th -> game complete
                Err(Error::GameComplete)
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None;
        }

        let mut total: u32 = 0;
        let mut i = 0usize;
        for _frame in 0..10 {
            if self.rolls[i] == 10 {
                // strike
                let a = self.rolls.get(i + 1).copied().unwrap_or(0) as u32;
                let b = self.rolls.get(i + 2).copied().unwrap_or(0) as u32;
                total += 10 + a + b;
                i += 1;
            } else {
                let r1 = self.rolls.get(i).copied().unwrap_or(0) as u32;
                let r2 = self.rolls.get(i + 1).copied().unwrap_or(0) as u32;
                if r1 + r2 == 10 {
                    // spare
                    let a = self.rolls.get(i + 2).copied().unwrap_or(0) as u32;
                    total += 10 + a;
                } else {
                    total += r1 + r2;
                }
                i += 2;
            }
        }

        Some(total as u16)
    }

    fn is_complete(&self) -> bool {
        let mut idx = 0usize;
        // first 9 frames
        for _ in 0..9 {
            if idx >= self.rolls.len() {
                return false;
            }
            let r = self.rolls[idx];
            if r == 10 {
                idx += 1;
            } else {
                idx += 1;
                if idx >= self.rolls.len() {
                    return false;
                }
                idx += 1;
            }
        }

        // now check 10th frame
        let remaining = self.rolls.len().saturating_sub(idx);
        if remaining == 0 {
            return false;
        }
        let r1 = self.rolls[idx];
        if r1 == 10 {
            // need two more rolls
            return remaining >= 3;
        }
        if remaining == 1 {
            return false;
        }
        let r2 = self.rolls[idx + 1];
        if r1 + r2 == 10 {
            // spare -> need one bonus
            return remaining >= 3;
        }
        // open frame -> only two rolls
        true
    }
}
