/*
    Appellation: tm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Registry;
use crate::prelude::{FsmError, Tape};
use crate::state::{State, Transition};

pub struct Context<T = String> {
    pub(crate) initial_state: State<T>,
    pub(crate) states: Vec<State<T>>,
    pub(crate) transitions: Vec<Transition<T>>,
}

///
pub struct TuringMachine {
    pub(crate) state: State,
    pub(crate) states: Vec<State>,
    pub(crate) tape: Tape,
    pub(crate) transitions: Registry,
}

impl TuringMachine {
    pub fn new(
        tape: Tape,
        start_state: State,
        states: Vec<State>,
        transforms: Vec<Transition>,
    ) -> Self {
        let mut transitions = Registry::new();
        for trs in transforms {
            transitions.insert(
                (trs.current_state.clone(), trs.read_symbol),
                (trs.next_state, trs.write_symbol, trs.direction),
            );
        }

        TuringMachine {
            tape,
            states,
            transitions,
            state: start_state,
        }
    }

    pub fn step(&mut self) -> Result<(), FsmError> {
        let cursor = self.state.clone();
        let current_symbol = *self.tape.read()?;
        if let Some(&(ref next_state, write_symbol, direction)) =
            self.transitions.get(&(cursor.clone(), current_symbol))
        {
            self.tape.write(write_symbol);
            self.tape.step(direction);
            self.state = next_state.clone();
            return Ok(());
        }
        Err(FsmError::state_not_found(cursor, current_symbol))
    }

    pub fn run(&mut self) {
        loop {
            self.tape.print_tape();
            self.step().expect("Error stepping through machine");
            if *self.state == "HALT" {
                break;
            }
        }
    }
}
