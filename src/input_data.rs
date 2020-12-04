pub mod print{

    pub fn print_day1(print_vec: Vec<String>) -> Vec<usize> {
        let usize_print_vec = print_vec.into_iter().map(|letter| { letter.parse::<usize>().unwrap() }).collect();
        return usize_print_vec
    }

    pub fn print_day2(print_vec: Vec<String>) -> Vec<(usize, usize, String, String)> {
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
}   