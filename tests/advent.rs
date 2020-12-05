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
    let input: Vec<String> = vec![
                    "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
                    "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
                    "".to_string(),
                    "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
                    "hcl:#cfa07d byr:1929".to_string(),
                    "".to_string(),
                    "hcl:#ae17e1 iyr:2013".to_string(),
                    "eyr:2024".to_string(),
                    "ecl:brn pid:760753108 byr:1931".to_string(),
                    "hgt:179cm".to_string(),
                    "".to_string(),
                    "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
                    "iyr:2011 ecl:brn hgt:59in".to_string()
                    ];
    return adventofcode_2020::input_data::print::print_day4(input)

}

fn return_good_passports() -> Vec<HashMap<String, String>> {
    let input: Vec<String> = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980".to_string(),
            "hcl:#623a2f".to_string(),
            "".to_string(),
            "eyr:2029 ecl:blu cid:129 byr:1989".to_string(),
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm".to_string(),
            "".to_string(),
            "hcl:#888785".to_string(),
            "hgt:164cm byr:2001 iyr:2015 cid:88".to_string(),
            "pid:545766238 ecl:hzl".to_string(),
            "eyr:2022".to_string(),
            "".to_string(),
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string()
        ];
    return adventofcode_2020::input_data::print::print_day4(input)
}

fn return_bad_passports() -> Vec<HashMap<String, String>> {
    let input: Vec<String> = vec![
            "eyr:1972 cid:100".to_string(), 
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".to_string(), 
            "".to_string(), 
            "iyr:2019".to_string(), 
            "hcl:#602927 eyr:1967 hgt:170cm".to_string(), 
            "ecl:grn pid:012533040 byr:1946".to_string(), 
            "".to_string(), 
            "hcl:dab227 iyr:2012".to_string(), 
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".to_string(), 
            "".to_string(), 
            "hgt:59cm ecl:zzz".to_string(), 
            "eyr:2038 hcl:74454a iyr:2023".to_string(), 
            "pid:3556412378 byr:2007".to_string(), 
            "".to_string()
        ];
    return adventofcode_2020::input_data::print::print_day4(input)
}


#[test]
fn find_number_valid_passports(){
    assert_eq!(adventofcode_2020::number_of_passports(false, return_passports()), 2)
}


#[test]
fn find_number_good_passports(){
    assert_eq!(adventofcode_2020::number_of_passports(true, return_good_passports()), 4)
}

// #[test]
// fn find_number_bad_passports(){
//     assert_eq!(adventofcode_2020::number_of_passports(true, return_bad_passports()), 0)
// }

fn test_password_contents(contents: HashMap<String, String>)-> bool{
    return adventofcode_2020::test_passport_contents(contents);
}

#[test]
fn test_password_contents_good_one(){
    assert_eq!(
        adventofcode_2020::test_passport_contents(return_good_passports().get(0).unwrap().clone()), true
    );
}
#[test]
fn test_password_contents_good_two(){
    assert_eq!(
        adventofcode_2020::test_passport_contents(return_good_passports().get(1).unwrap().clone()), true
    );
}
#[test]
fn test_password_contents_good_three(){
    assert_eq!(
        adventofcode_2020::test_passport_contents(return_good_passports().get(2).unwrap().clone()), true
    );
}
#[test]
fn test_password_contents_good_four(){
    assert_eq!(
        adventofcode_2020::test_passport_contents(return_good_passports().get(3).unwrap().clone()), true
    );
}

// #[test]
// fn test_password_contents_bad_one(){
//     assert_eq!(
//         adventofcode_2020::test_passport_contents(return_bad_passports().get(0).unwrap().clone()), false
//     );
// }
// #[test]
// fn test_password_contents_bad_two(){
//     assert_eq!(
//         adventofcode_2020::test_passport_contents(return_bad_passports().get(1).unwrap().clone()), false
//     );
// }
// #[test]
// fn test_password_contents_bad_three(){
//     assert_eq!(
//         adventofcode_2020::test_passport_contents(return_bad_passports().get(2).unwrap().clone()), false
//     );
// }
// #[test]
// fn test_password_contents_bad_four(){
//     assert_eq!(
//         adventofcode_2020::test_passport_contents(return_bad_passports().get(3).unwrap().clone()), false
//     );
// }
