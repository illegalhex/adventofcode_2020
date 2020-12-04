use std::collections::HashMap;

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
    assert_eq!(adventofcode_2020::count_of_trees(3, 1, return_tree_map()), 7)
}


#[test]
fn find_multislopes_trees_first(){

    assert_eq!(adventofcode_2020::count_of_trees(1, 1, return_tree_map()), 2);
}
#[test]
fn find_multislopes_trees_second(){

    assert_eq!(adventofcode_2020::count_of_trees(3, 1, return_tree_map()), 7);
}
#[test]
fn find_multislopes_trees_third(){

    assert_eq!(adventofcode_2020::count_of_trees(5, 1, return_tree_map()), 3);
}
#[test]
fn find_multislopes_trees_forth(){

    assert_eq!(adventofcode_2020::count_of_trees(7, 1, return_tree_map()), 4);
}
#[test]
fn find_multislopes_trees_fifth(){
    assert_eq!(adventofcode_2020::count_of_trees(1, 2, return_tree_map()), 2);
}
#[test]
fn find_multislopes_trees_product(){
    let first = adventofcode_2020::count_of_trees(1, 1, return_tree_map());
    let second = adventofcode_2020::count_of_trees(3, 1, return_tree_map());
    let third = adventofcode_2020::count_of_trees(5, 1, return_tree_map());
    let forth = adventofcode_2020::count_of_trees(7, 1, return_tree_map());
    let fifth = adventofcode_2020::count_of_trees(1, 2, return_tree_map());

    assert_eq!(first * second * third * forth * fifth, 336);
}

fn return_passports() -> Vec<HashMap<String, String>> {
    let mut one =  HashMap::new();
    let mut two =  HashMap::new();
    let mut three =  HashMap::new();
    let mut four =  HashMap::new();

    one.insert("ecl".to_string(),"gry".to_string());
    one.insert("pid".to_string(),"860033327".to_string());
    one.insert("eyr".to_string(),"2020".to_string());
    one.insert("hcl".to_string(),"#fffffd".to_string());
    one.insert("byr".to_string(),"1937".to_string());
    one.insert("iyr".to_string(),"2017".to_string());
    one.insert("cid".to_string(),"147".to_string());
    one.insert("hgt".to_string(),"183cm".to_string());

    two.insert("iyr".to_string(),"2013".to_string());
    two.insert("ecl".to_string(),"amb".to_string());
    two.insert("cid".to_string(),"350".to_string());
    two.insert("eyr".to_string(),"2023".to_string());
    two.insert("pid".to_string(),"028048884".to_string());
    two.insert("hcl".to_string(),"#cfa07d".to_string());
    two.insert("byr".to_string(),"1929".to_string());

    three.insert("hcl".to_string(),"#ae17e1".to_string());
    three.insert("iyr".to_string(),"2013".to_string());
    three.insert("eyr".to_string(),"2024".to_string());
    three.insert("ecl".to_string(),"brn".to_string());
    three.insert("pid".to_string(),"760753108".to_string());
    three.insert("byr".to_string(),"1931".to_string());
    three.insert("hgt".to_string(),"179cm".to_string());

    four.insert("hcl".to_string(),"#cfa07d".to_string());
    four.insert("eyr".to_string(),"2025".to_string());
    four.insert("pid".to_string(),"166559648".to_string());
    four.insert("iyr".to_string(),"2011".to_string());
    four.insert("ecl".to_string(),"brn".to_string());
    four.insert("hgt".to_string(),"59in".to_string());

    return vec![one, two, three, four]
}

#[test]
fn find_number_valid_passports(){
    assert_eq!(adventofcode_2020::number_of_passports(return_passports()), 2)
}