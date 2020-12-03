#[allow(dead_code)]
pub fn print(print_vec: Vec<String>) -> Vec<usize> {
    let usize_print_vec = print_vec.into_iter().map(|letter| { letter.parse::<usize>().unwrap() }).collect();
    return usize_print_vec
}
