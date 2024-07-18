/*
    Appellation: tape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tape {
    tape: Vec<char>,
    head: usize,
}

impl Tape {
    pub fn new(input: &str) -> Tape {
        Tape {
            tape: input.chars().collect(),
            head: 0,
        }
    }

    pub fn read(&self) -> char {
        *self.tape.get(self.head).unwrap_or(&' ')
    }

    pub fn write(&mut self, symbol: char) {
        if self.head < self.tape.len() {
            self.tape[self.head] = symbol;
        } else {
            self.tape.push(symbol);
        }
    }

    pub fn move_head(&mut self, direction: char) {
        match direction {
            'L' => {
                if self.head > 0 {
                    self.head -= 1;
                }
            }
            'R' => self.head += 1,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn print_tape(&self) {
        println!(
            "{}",
            self.tape
                .iter()
                .enumerate()
                .map(|(i, &c)| if i == self.head { format!("[{}]", c) } else { c.to_string() })
                .collect::<String>()
        );
    }
}