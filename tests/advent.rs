
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


fn return_tree_map() -> Vec<String> {
    return vec![
        "..##.........##.........##.........##.........##.........##.......".to_string(),
        "#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..".to_string(),
        ".#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.".to_string(),
        "..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#".to_string(),
        ".#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.".to_string(),
        "..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....".to_string(),
        ".#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#".to_string(),
        ".#........#.#........#.#........#.#........#.#........#.#........#".to_string(),
        "#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...".to_string(),
        "#...##....##...##....##...##....##...##....##...##....##...##....#".to_string(),
        ".#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#".to_string()
    ];
}

#[test]
fn find_num_trees(){
    assert_eq!(adventofcode_2020::count_of_trees(1, 3, return_tree_map()), 7)
}

#[test]
fn find_num_multislopes_trees(){
    let first = adventofcode_2020::count_of_trees(1, 1, return_tree_map());
    let second = adventofcode_2020::count_of_trees(1, 1, return_tree_map());
    let third = adventofcode_2020::count_of_trees(1, 1, return_tree_map());
    let forth = adventofcode_2020::count_of_trees(1, 1, return_tree_map());
    let fifth = adventofcode_2020::count_of_trees(1, 1, return_tree_map());
    // assert_eq!(adventofcode_2020::count_of_trees(return_tree_map()), 336)
}