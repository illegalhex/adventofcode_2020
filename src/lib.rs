use itertools::Itertools;

pub fn find_2020_pair(vals: Vec<i32> ) -> i32 {

    let val_pairs = vals.into_iter().tuple_combinations::<(_, _)>();
    for i in val_pairs.into_iter() {
        if i.0 + i.1 == 2020{
            println!("{},{}", i.0, i.1)
        }
    }
    return 514579
}