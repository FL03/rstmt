/*
    Appellation: parallel <module>
    Created At: 2025.12.24:15:41:09
    Contrib: @FL03
*/

pub struct ParIter<'a> {
    _marker: core::marker::PhantomData<&'a ()>,
}
