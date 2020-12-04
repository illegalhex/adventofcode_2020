mod input_data;

fn day1(){
    println!("ex1 answer is:   {}", adventofcode_2020::find_2020_elements(2, input_data::day_one::print(adventofcode_2020::readlines("text/day1.txt" ))).unwrap());
    println!("ex2 answer is:   {}", adventofcode_2020::find_2020_elements(3, input_data::day_one::print(adventofcode_2020::readlines("text/day1.txt" ))).unwrap());
}

fn day2(){
    println!("number of valid passwords is: {}", adventofcode_2020::valid_password("old", input_data::day_two::print(adventofcode_2020::readlines("text/day2.txt" ))));
    println!("number of valid passwords is: {}", adventofcode_2020::valid_password("new", input_data::day_two::print(adventofcode_2020::readlines("text/day2.txt" ))));
}
fn day3(){
    println!("number of trees are: {}", adventofcode_2020::count_of_trees(3, 1, adventofcode_2020::readlines("text/day3.txt" )));
    let first = adventofcode_2020::count_of_trees(1, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let second = adventofcode_2020::count_of_trees(3, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let third = adventofcode_2020::count_of_trees(5, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let forth = adventofcode_2020::count_of_trees(7, 1, adventofcode_2020::readlines("text/day3.txt" ));
    let fifth = adventofcode_2020::count_of_trees(1, 2, adventofcode_2020::readlines("text/day3.txt" ));
    let ski_product = first * second * third * forth * fifth;
    println!("the product of  {}, {}, {}, {}, {} is {}", first , second , third , forth , fifth, ski_product)
}

fn main() {
    day1();
    day2();
    day3();
}
