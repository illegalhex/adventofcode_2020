mod input_data;

fn day1(){
    println!("ex1 answer is:   {}", adventofcode_2020::find_2020_elements(2, input_data::day_one::print(adventofcode_2020::readlines("text/day1.txt" ))).unwrap());
    println!("ex2 answer is:   {}", adventofcode_2020::find_2020_elements(3, input_data::day_one::print(adventofcode_2020::readlines("text/day1.txt" ))).unwrap());
}

fn day2(){
    println!("number of valid passwords is: {}", adventofcode_2020::valid_password("old", input_data::day_two::print(adventofcode_2020::readlines("text/day2.txt" ))));
    println!("number of valid passwords is: {}", adventofcode_2020::valid_password("new", input_data::day_two::print(adventofcode_2020::readlines("text/day2.txt" ))));
}

fn main() {
    day1();
    day2();
}
