
// day1
#[test]
fn find_2020_pair(){
    assert_eq!(adventofcode_2020::find_2020_pair((2, vec![1721,
        979,
        366,
        299,
        675,
        1456])), 514579)
}
#[test]
fn find_2020_triplets(){
    assert_eq!(adventofcode_2020::find_2020_pair((3, vec![1721,
        979,
        366,
        299,
        675,
        1456])), 241861950)
}

#[test]
fn find_num_valid_passwords_old(){
    assert_eq!(adventofcode_2020::valid_password("old", vec![
        (1, 3, "a", "abcde"),
        (1, 3, "b", "cdefg"),
        (2, 9, "c", "ccccccccc" )
    ]), 2)
}

#[test]
fn find_num_valid_passwords_new(){
    assert_eq!(adventofcode_2020::valid_password("new", vec![
        (1, 3, "a", "abcde"),
        (1, 3, "b", "cdefg"),
        (2, 9, "c", "ccccccccc" )
    ]), 1)
}