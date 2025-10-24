use my_lib_1::study_1::test_study;

#[test]
fn test_add() {
    let a1 = test_study::unit_1::add(3, 2);
    assert_eq!(a1, 5);
}
