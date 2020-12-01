

pub fn find_2020_pair(vals: Vec<i32> ) -> Vec<i32> {
    let vals_len = vals.len();
    let mut i = vals_len - 1 ;
    while i > 0 { 
        for n in vals.iter() { 
            println!("in{}, out{}", vals[i], n)
        }
    i -= 1
    }
    vec![1,2]
}