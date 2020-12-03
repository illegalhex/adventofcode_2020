#[allow(dead_code)]
pub fn print(print_vec: Vec<String>) -> Vec<(usize, usize, String, String)> {
    let mut return_vec: Vec<(usize, usize, String, String)>  = Vec::new();
    for i in print_vec.into_iter() {
        let temp_i = i;
        let  vec_i: Vec<&str> = temp_i.split(|x| (x == '-') || (x == ':')|| (x == ' ')) .collect();
        let vec_temp: (usize, usize, String, String) =(
            vec_i[0].parse::<usize>().unwrap(),
            vec_i[1].parse::<usize>().unwrap(),
            vec_i[2].to_string(),
            vec_i[4].to_string()
        );
        return_vec.push(vec_temp)
    }
    return return_vec
}