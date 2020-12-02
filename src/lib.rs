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

pub fn valid_password(passwords: Vec<(usize, usize, &str, &str)>) -> usize{
    2
}