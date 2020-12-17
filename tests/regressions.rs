// Regression testing for refactoring covering the original answers


#[test]
fn find_2020_pair(){
assert_eq!(adventofcode_2020::find_2020_elements(2, adventofcode_2020::input_data::print::print_day1(adventofcode_2020::readlines("text/day1.txt" ))).unwrap(), 713184)
}

#[test]
fn find_2020_triplets(){
    assert_eq!(adventofcode_2020::find_2020_elements(3, adventofcode_2020::input_data::print::print_day1(adventofcode_2020::readlines("text/day1.txt" ))).unwrap(), 261244452)
}

#[test]
fn find_num_valid_passwords_old(){
assert_eq!(adventofcode_2020::valid_password("old", adventofcode_2020::input_data::print::print_day2(adventofcode_2020::readlines("text/day2.txt" ))), 620)
}

#[test]
fn find_num_valid_passwords_new(){
    assert_eq!(adventofcode_2020::valid_password("new", adventofcode_2020::input_data::print::print_day2(adventofcode_2020::readlines("text/day2.txt" ))), 727)
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


#[test]
fn find_number_of_valid_passports(){
    assert_eq!( adventofcode_2020::number_of_passports(false, adventofcode_2020::input_data::print::print_day4(adventofcode_2020::readlines("text/day4.txt" ))), 210 );
}
#[test]
fn find_number_of_valid_passports_and_validate(){
    assert_eq!( adventofcode_2020::number_of_passports(true, adventofcode_2020::input_data::print::print_day4(adventofcode_2020::readlines("text/day4.txt" ))), 131 );
}

#[test]
fn find_largest_boarding_pass(){
    assert_eq!( adventofcode_2020::find_largest_boarding_pass_id(adventofcode_2020::readlines("text/day5.txt" )), 935 );
}
#[test]
fn find_my_boarding_pass(){
    assert_eq!( adventofcode_2020::find_my_boarding_pass(adventofcode_2020::readlines("text/day5.txt" )), 743 );
}

#[test]
fn find_number_of_yes_1(){
    assert_eq!( adventofcode_2020::find_number_of_yeses(0, adventofcode_2020::readlines("text/day6.txt" )), 6930 );
}

#[test]
fn find_number_of_yes_1(){
    assert_eq!( adventofcode_2020::find_number_of_yeses(1, adventofcode_2020::readlines("text/day6.txt" )), 3585 );
}