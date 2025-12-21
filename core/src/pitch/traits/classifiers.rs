/*
    Appellation: classifiers <module>
    Created At: 2025.12.21:09:08:08
    Contrib: @FL03
*/
/// The [`PitchCls`] trait establishes an interface to defining pitch classes.
pub trait PitchCls: core::fmt::Debug {
    const IDX: usize;
    type Tag: PitchType;

    private! {}

    fn new() -> Self
    where
        Self: Sized;

    fn index(&self) -> usize;
}
/// [`PitchType`] is a sealed marker trait used to designate various _kinds_ of musical notes,
/// i.e., sharp, flat, natural, etc.
pub trait PitchType {
    private! {}
}
