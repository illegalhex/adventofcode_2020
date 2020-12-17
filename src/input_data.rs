

pub mod print{
    use std::collections::HashMap;
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

    pub fn print_day4(print_vec: Vec<String>) -> Vec<HashMap<String, String>>{
        // This will take in the unmodified starting text, and return a vector of hash maps
        // that match the defined data structure
        let mut return_vec: Vec<_>  = Vec::new();
        let mut massive_string: String = "".to_string();
        for i in print_vec.into_iter() {if i == ""{massive_string.push_str(", ");continue;}massive_string.push_str(format!("{} ",&i).as_str());}
        let first_level_vec: Vec<&str>  = massive_string.split(|x| x == ',').collect();
        for i in first_level_vec.into_iter(){
            let temp_vec = i.split_whitespace().collect::<Vec<&str>>();
            let mut temp_hash: HashMap<String, String> = HashMap::new();
            for i in temp_vec.into_iter(){
                let vec_i: Vec<&str> = i.split(|x: char| x.to_string() == ":").collect();
                temp_hash.insert(vec_i[0].to_string(),vec_i[1].to_string());
            }
            return_vec.push(temp_hash);
        }
        return_vec
    }
    
    pub fn print_day6(print_vec: Vec<String>) -> Vec<String>{
        let return_vec = print_vec.into_iter().map(|s| if s == "" {s.replace("",",")}else{s}).collect::<Vec<String>>()
                        .concat().split(',').map(String::from).collect::<Vec<String>>();
        return return_vec
        }
}   