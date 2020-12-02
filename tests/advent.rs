
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
