/*
    Appellation: triad <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstmt_core as rstmt;
extern crate rstmt_neo as neo;

use neo::triad::Triad;
use rstmt::{Fifth, IntervalOps, Note, Third};

#[test]
fn test_major() {
    let root = Note::from_pitch(0);
    let triad = Triad::major(root);
    // validate the chord factors
    assert_eq!(triad.root(), root);
    assert_eq!(triad.third(), root.add_major_third());
    assert_eq!(triad.fifth(), root.add_perfect_fifth());
    // validate the chord intervals
    assert_eq!(triad.root_to_third(), Ok(Third::Major));
    assert_eq!(triad.third_to_fifth(), Ok(Third::Minor));
    assert_eq!(triad.root_to_fifth(), Ok(Fifth::Perfect));
}

#[test]
fn test_minor() {
    let root = Note::from_pitch(0);
    let triad = Triad::minor(root);
    // validate the chord factors
    assert_eq!(triad.root(), root);
    assert_eq!(triad.third(), root.add_minor_third());
    assert_eq!(triad.fifth(), root.add_perfect_fifth());
    // validate the chord intervals
    assert_eq!(triad.root_to_third(), Ok(Third::Minor));
    assert_eq!(triad.third_to_fifth(), Ok(Third::Major));
    assert_eq!(triad.root_to_fifth(), Ok(Fifth::Perfect));
}

#[test]
fn test_augmented() {
    let root = Note::from_pitch(0);
    let triad = Triad::augmented(root);
    // validate the chord factors
    assert_eq!(triad.root(), root);
    assert_eq!(triad.third(), root.add_major_third());
    assert_eq!(triad.fifth(), root.add_augmented_fifth());
    // validate the chord intervals
    assert_eq!(triad.root_to_third(), Ok(Third::Major));
    assert_eq!(triad.third_to_fifth(), Ok(Third::Major));
    assert_eq!(triad.root_to_fifth(), Ok(Fifth::Augmented));
}

#[test]
fn test_diminished() {
    let root = Note::from_pitch(0);
    let triad = Triad::diminished(root);
    // validate the chord factors
    assert_eq!(triad.root(), root);
    assert_eq!(triad.third(), root.add_minor_third());
    assert_eq!(triad.fifth(), root.add_diminished_fifth());
    // validate the chord intervals
    assert_eq!(triad.root_to_third(), Ok(Third::Minor));
    assert_eq!(triad.third_to_fifth(), Ok(Third::Minor));
    assert_eq!(triad.root_to_fifth(), Ok(Fifth::Diminished));
}
