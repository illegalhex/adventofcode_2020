mod input_data;
use std::time::Instant;

fn day1(){
    let now1 = Instant::now();
    println!("The product of the two items who's sum is 2020 is    :   {}", adventofcode_2020::find_2020_elements(2, input_data::print::print_day1(adventofcode_2020::readlines("text/day1.txt" ))).unwrap());
    let now2 = Instant::now();
    println!("The product of the three items who's sum is 2020 is  :   {}", adventofcode_2020::find_2020_elements(3, input_data::print::print_day1(adventofcode_2020::readlines("text/day1.txt" ))).unwrap());
    println!("Day 1 part 1 took: {} ms to run", now1.elapsed().as_millis());
    println!("Day 1 part 2 took: {} ms to run\n", now2.elapsed().as_millis());
}

fn day2(){
    let now1 = Instant::now();
    println!("The number of valid passwords with the old method is : {}", adventofcode_2020::valid_password("old", input_data::print::print_day2(adventofcode_2020::readlines("text/day2.txt" ))));
    let now2 = Instant::now();
    println!("The number of valid passwords with the old method is : {}", adventofcode_2020::valid_password("new", input_data::print::print_day2(adventofcode_2020::readlines("text/day2.txt" ))));
    println!("Day 2 part 2 took: {} ms to run", now1.elapsed().as_millis());
    println!("Day 2 part 2 took: {} ms to run\n", now2.elapsed().as_millis());
}
fn day3(){
    let now1 = Instant::now();
    println!("There are {} trees during the simple slide", adventofcode_2020::count_of_trees(3, 1, adventofcode_2020::readlines("text/day3.txt" )));
    let now2 = Instant::now();
    let first = adventofcode_2020::count_of_trees(1, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let second = adventofcode_2020::count_of_trees(3, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let third = adventofcode_2020::count_of_trees(5, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let forth = adventofcode_2020::count_of_trees(7, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let fifth = adventofcode_2020::count_of_trees(1, 2, adventofcode_2020::readlines("text/day3.txt" ));
    let ski_product = first * second * third * forth * fifth;
    println!("There are {} trees. It is the product of: {}, {}, {}, {}  {}",ski_product, first , second , third , forth , fifth );
    println!("Day 3 part 1 took: {} ms to run", now1.elapsed().as_millis());
    println!("Day 3 part 2 took: {} ms to run\n", now2.elapsed().as_millis());
}

fn day4(){
    let now1 = Instant::now();
    println!("There are {} valid passports if cid is ignored", adventofcode_2020::number_of_passports(false, input_data::print::print_day4(adventofcode_2020::readlines("text/day4.txt" ))));
    let now2 = Instant::now();
    println!("There are {} valid passports if cid is ignored and data is validated", adventofcode_2020::number_of_passports(true, input_data::print::print_day4(adventofcode_2020::readlines("text/day4.txt" ))));
    println!("Day 4 part one took: {} ms to run", now1.elapsed().as_millis());
    println!("Day 4 part two took: {} ms to run\n", now2.elapsed().as_millis());
}

fn day5(){
    let now1 = Instant::now();
    println!("The largest valid boarding pass is: {}", 
    adventofcode_2020::find_largest_boarding_pass_id(adventofcode_2020::readlines("text/day5.txt" )));
    let now2 = Instant::now();
    println!("My boarding pass is: {}",
    adventofcode_2020::find_my_boarding_pass(adventofcode_2020::readlines("text/day5.txt" )));
    println!("Day 5 part one took: {} ms to run", now1.elapsed().as_millis());
    println!("Day 5 part two took: {} ms to run\n", now2.elapsed().as_millis());
}

fn day6(){
    let now1 = Instant::now();
    println!("The number of yes on all custom cards is: {}", 
    adventofcode_2020::find_number_of_yeses(0, input_data::print::print_day6(adventofcode_2020::readlines("text/day6.txt" ))));
    println!("Day 6 part one took: {} ms to run", now1.elapsed().as_millis());
}

fn main() {
    day1();
    day2();
    day3();
    day4();
    day5();
    day6()
}
