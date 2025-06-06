/*
    Appellation: triad <test>
    Contrib: @FL03
*/
use rstmt_nrt::Triads;

#[test]
fn test_class_creation() {
    let class = Triads::try_from_arr([0, 4, 7]).unwrap();
    assert!(class.is_major());
    let class = Triads::try_from_arr([0, 3, 7]).unwrap();
    assert!(class.is_minor());
    let class = Triads::try_from_arr([0, 4, 8]).unwrap();
    assert!(class.is_augmented());
    let class = Triads::try_from_arr([0, 3, 6]).unwrap();
    assert!(class.is_diminished());

    let class = Triads::try_from_arr([0, 7, 4]).unwrap();
    assert!(class.is_major());
    let class = Triads::try_from_arr([0, 7, 3]).unwrap();
    assert!(class.is_minor());
    let class = Triads::try_from_arr([8, 0, 4]).unwrap();
    assert!(class.is_augmented());
    let class = Triads::try_from_arr([6, 0, 3]).unwrap();
    assert!(class.is_diminished());
}
