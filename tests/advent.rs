
// day1
#[test]
fn find_2020_pair(){
    assert_eq!(adventofcode_2020::find_2020_elements(2, vec![1721,
        979,
        366,
        299,
        675,
        1456]).unwrap(), 514579)
}
#[test]
fn find_2020_triplets(){
    assert_eq!(adventofcode_2020::find_2020_elements(3, vec![1721,
        979,
        366,
        299,
        675,
        1456]).unwrap(), 241861950)
}

#[test]
fn find_num_valid_passwords_old(){
    assert_eq!(adventofcode_2020::valid_password("old", vec![
        (1, 3, "a".to_string(), "abcde".to_string()),
        (1, 3, "b".to_string(), "cdefg".to_string()),
        (2, 9, "c".to_string(), "ccccccccc".to_string() )
    ]), 2)
}

#[test]
fn find_num_valid_passwords_new(){
    assert_eq!(adventofcode_2020::valid_password("new", vec![
        (1, 3, "a".to_string(), "abcde".to_string()),
        (1, 3, "b".to_string(), "cdefg".to_string()),
        (2, 9, "c".to_string(), "ccccccccc".to_string() )
    ]), 1)
}