

pub fn find_2020_pair(vals: Vec<i32> ) -> Vec<i32> {
    let vals_len = vals.len();
    for i in vals.into_iter() {
        for n in 1..vals_len{
            println!("in{}, out{}", n, i)
        }
        // println!("{}", i)
    }
    vec![1,2]
}