mod input_data;

fn day1(){
    println!("The product of the two items who's sum is 2020 is    :   {}", adventofcode_2020::find_2020_elements(2, input_data::print::print_day1(adventofcode_2020::readlines("text/day1.txt" ))).unwrap());
    println!("The product of the three items who's sum is 2020 is  :   {}", adventofcode_2020::find_2020_elements(3, input_data::print::print_day1(adventofcode_2020::readlines("text/day1.txt" ))).unwrap());
}

fn day2(){
    println!("The number of valid passwords with the old method is : {}", adventofcode_2020::valid_password("old", input_data::print::print_day2(adventofcode_2020::readlines("text/day2.txt" ))));
    println!("The number of valid passwords with the old method is : {}", adventofcode_2020::valid_password("new", input_data::print::print_day2(adventofcode_2020::readlines("text/day2.txt" ))));
}
fn day3(){
    println!("There are {} trees during the simple slide", adventofcode_2020::count_of_trees(3, 1, adventofcode_2020::readlines("text/day3.txt" )));
    let first = adventofcode_2020::count_of_trees(1, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let second = adventofcode_2020::count_of_trees(3, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let third = adventofcode_2020::count_of_trees(5, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let forth = adventofcode_2020::count_of_trees(7, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let fifth = adventofcode_2020::count_of_trees(1, 2, adventofcode_2020::readlines("text/day3.txt" ));
    let ski_product = first * second * third * forth * fifth;
    println!("There are {} trees. It is the product of: {}, {}, {}, {}  {}",ski_product, first , second , third , forth , fifth )
}

fn main() {
    day1();
    day2();
    day3();
}
