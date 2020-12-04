// Regression testing for refactoring covering the original answers


#[test]
fn find_2020_pair(){
assert_eq!(adventofcode_2020::find_2020_elements(2, adventofcode_2020::input_data::day_one::print(adventofcode_2020::readlines("text/day1.txt" ))).unwrap(), 713184)
}

#[test]
fn find_2020_triplets(){
    assert_eq!(adventofcode_2020::find_2020_elements(3, adventofcode_2020::input_data::day_one::print(adventofcode_2020::readlines("text/day1.txt" ))).unwrap(), 261244452)
}

#[test]
fn find_num_valid_passwords_old(){
assert_eq!(adventofcode_2020::valid_password("old", adventofcode_2020::input_data::day_two::print(adventofcode_2020::readlines("text/day2.txt" ))), 620)
}

#[test]
fn find_num_valid_passwords_new(){
    assert_eq!(adventofcode_2020::valid_password("new", adventofcode_2020::input_data::day_two::print(adventofcode_2020::readlines("text/day2.txt" ))), 727)
}

#[test]
fn find_number_of_trees(){
    assert_eq!(adventofcode_2020::count_of_trees(3, 1, adventofcode_2020::readlines("text/day3.txt" )), 156)
}
#[test]
fn find_number_of_trees_mul(){
    let first = adventofcode_2020::count_of_trees(1, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let second = adventofcode_2020::count_of_trees(3, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let third = adventofcode_2020::count_of_trees(5, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let forth = adventofcode_2020::count_of_trees(7, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let fifth = adventofcode_2020::count_of_trees(1, 2, adventofcode_2020::readlines("text/day3.txt" ));
    assert_eq!(first * second * third * forth * fifth, 3521829480);
}