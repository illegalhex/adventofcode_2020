use itertools::Itertools;

pub fn find_2020_pair(vals_in: (usize, Vec<i64>) ) -> i64 {
    let vals = vals_in.1;
    let mut value = 0;
    let val_pairs = vals.into_iter().combinations(vals_in.0);
    for i in val_pairs.into_iter() {
        if i.iter().sum::<i64>() == 2020 {
            value = i.iter().product();
        }
    }
    value
}

pub fn valid_password(pass_type: &str, passwords: Vec<(usize, usize, &str, &str)>) -> usize{
    let mut num_valid_passwords = 0;
    match pass_type {
        "old" => for i in passwords.into_iter() {
                if i.3.matches(i.2).count() >= i.0 && i.3.matches(i.2).count() <= i.1 {
                    num_valid_passwords += 1;
                }
            },
        "new" => {num_valid_passwords =0 },
        &_ => {num_valid_passwords =0 }
    }

    num_valid_passwords 
}