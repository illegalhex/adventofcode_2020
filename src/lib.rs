use itertools::Itertools;

pub mod input_data;

pub fn find_2020_elements(size: usize, vals_in:Vec<usize>) -> usize {
    for i in vals_in.into_iter().combinations(size).into_iter() {
        if i.iter().sum::<usize>() == 2020 {
            return i.iter().product()
        }
    }
    0
}

pub fn valid_password(pass_type: &str, passwords: Vec<(usize, usize, &str, &str)>) -> usize{
    let mut num_valid_passwords = 0;
    match pass_type {
        "old" => for i in passwords.into_iter() {
                if i.3.matches(i.2).count() >= i.0 && i.3.matches(i.2).count() <= i.1 {
                    num_valid_passwords += 1;
                }
            },
        "new" => for i in passwords.into_iter(){
                let password_candidate = i.3.chars().collect::<Vec<char>>().into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
                if password_candidate[i.0-1] == i.2 && password_candidate[i.1-1] != i.2 || password_candidate[i.0-1] != i.2 && password_candidate[i.1-1] == i.2 {
                    num_valid_passwords += 1;
                }
        },
        &_ => {}
    }

    num_valid_passwords 
}