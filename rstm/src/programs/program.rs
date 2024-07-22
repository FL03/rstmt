/*
    Appellation: program <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Instruction;
use crate::state::State;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Program<Q = String> {
    pub(crate) initial_state: State<Q>,
    pub(crate) instructions: Vec<Instruction<Q>>,
}

impl<Q> Program<Q> {
    pub fn new(initial_state: State<Q>) -> Self {
        Self {
            initial_state,
            instructions: Vec::new(),
        }
    }

    pub fn with_default_state() -> Self
    where
        Q: Default,
    {
        Self::new(State::default())
    }

    pub fn as_slice(&self) -> &[Instruction<Q>] {
        &self.instructions
    }

    pub fn as_mut_slice(&mut self) -> &mut [Instruction<Q>] {
        &mut self.instructions
    }

    pub fn initial_state(&self) -> &State<Q> {
        &self.initial_state
    }

    pub fn instructions(&self) -> &[Instruction<Q>] {
        &self.instructions
    }

    pub fn push(&mut self, instruction: Instruction<Q>) {
        self.instructions.push(instruction);
    }
}

impl<T> Extend<Instruction<T>> for Program<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Instruction<T>>,
    {
        self.instructions.extend(iter);
    }
}

impl<T> IntoIterator for Program<T> {
    type Item = Instruction<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.instructions.into_iter()
    }
}

impl<T, I> core::ops::Index<I> for Program<T>
where
    I: core::slice::SliceIndex<[Instruction<T>]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.instructions[index]
    }
}

impl<T, I> core::ops::IndexMut<I> for Program<T>
where
    I: core::slice::SliceIndex<[Instruction<T>]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.instructions[index]
    }
}
