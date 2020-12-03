// Regression testing for refactoring covering the original answers


#[test]
fn find_2020_pair(){
assert_eq!(adventofcode_2020::find_2020_pair((2, adventofcode_2020::input_data::day_one::print())), 713184)
}

#[test]
fn find_2020_triplets(){
    assert_eq!(adventofcode_2020::find_2020_pair((3, adventofcode_2020::input_data::day_one::print())), 261244452)
}

#[test]
fn find_num_valid_passwords_old(){
assert_eq!(adventofcode_2020::valid_password("old", adventofcode_2020::input_data::day_two::print()), 620)
}

#[test]
fn find_num_valid_passwords_new(){
    assert_eq!(adventofcode_2020::valid_password("new", adventofcode_2020::input_data::day_two::print()), 727)
}