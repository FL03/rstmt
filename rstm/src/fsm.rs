/*
    Appellation: fsm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/



use std::collections::HashMap;

pub use self::{state::State, tape::Tape, transition::Transition};

pub(crate) mod state;
pub(crate) mod tape;
pub(crate) mod transition;


pub struct TuringMachine {
    pub(crate) tape: Tape,
    pub(crate) states: Vec<State>,
    pub(crate) transitions: HashMap<(State, char), (State, char, char)>,
    pub(crate) current_state: State,
}

impl TuringMachine {
    pub fn new(tape: Tape, start_state: State, states: Vec<State>, transitions: Vec<Transition>) -> TuringMachine {
        let mut transition_map = HashMap::new();
        for transition in transitions {
            transition_map.insert(
                (transition.current_state.clone(), transition.read_symbol),
                (
                    transition.next_state,
                    transition.write_symbol,
                    transition.direction,
                ),
            );
        }

        TuringMachine {
            tape,
            states,
            transitions: transition_map,
            current_state: start_state,
        }
    }

    pub fn step(&mut self) {
        let current_symbol = self.tape.read();
        if let Some(&(ref next_state, write_symbol, direction)) = self.transitions.get(&(self.current_state.clone(), current_symbol)) {
            self.tape.write(write_symbol);
            self.tape.move_head(direction);
            self.current_state = next_state.clone();
        } else {
            println!("No transition found for state {:?} and symbol '{}'", self.current_state, current_symbol);
        }
    }

    pub fn run(&mut self) {
        loop {
            self.tape.print_tape();
            self.step();
            if self.current_state.0 == "HALT" {
                break;
            }
        }
    }
}