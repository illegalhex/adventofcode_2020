use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub mod input_data;

pub fn loadfile<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn readlines(filename: &str) -> Vec<String> {
    let mut file_vec = Vec::new();
    if let Ok(lines) = loadfile(filename) {
        for line in lines {
            if let Ok(ele) = line {
                file_vec.push(ele);
            }
        }
    }
    file_vec
}


pub fn find_2020_elements(size: usize, vals_in:Vec<usize>) -> Result<usize, String>  {
    for i in vals_in.into_iter().combinations(size).into_iter() {
        if i.iter().sum::<usize>() == 2020 {
            return Ok(i.iter().product())
        }
    }
    Err("No match was found".to_string())
}

pub fn valid_password(pass_type: &str, passwords: Vec<(usize, usize, String, String)>) -> usize{
    let mut num_valid_passwords = 0;
    match pass_type {
        "old" => for i in passwords.into_iter() {
                if i.3.matches(&i.2).count() >= i.0 && i.3.matches(&i.2).count() <= i.1 {
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

pub fn count_of_trees(right: usize, down: usize, tree_map: Vec<String>)-> usize{
    let mut tree_count = 0;
    let mut right_offset = 0;
    for (count, element) in tree_map.iter().enumerate() {
        if  (count + 1)  % down == 0 && down != 1 { continue }
        if element.chars().nth(right_offset % element.len() ) == Some('#') {tree_count+=1};
        right_offset += right;
    }
    tree_count
}


pub fn number_of_passports(verify_data: bool, passport_vec: Vec<HashMap<String, String>>) -> usize{

    let mut valid_passports: usize = 0;

    for passports in passport_vec.into_iter(){
        if verify_data {
            if test_passport_contents(passports){
                valid_passports += 1;
            }
        } else {
            if check_passport_fields(passports){
                valid_passports += 1;
            }
        }
    }
    valid_passports
}

pub fn check_passport_fields(passport: HashMap<String, String>) -> bool {
    let required_fields: Vec<String> = vec![
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string(),
        ];
    let req_field_clone = required_fields.clone();
    let key_match: Vec<_> = req_field_clone.iter().filter(|k| passport.contains_key(k.as_str())).collect();
    if key_match.len() == req_field_clone.len(){
        return true;
    } else {
        return false;
    }
}

pub fn test_passport_contents(passport: HashMap<String, String>) -> bool {
    let passport_clone = passport.clone();
    if check_passport_fields(passport_clone) == false {return false}; 
    let byr = passport.get("byr").unwrap().parse::<isize>();
    let iyr = passport.get("iyr").unwrap().parse::<isize>();
    let eyr = passport.get("eyr").unwrap().parse::<isize>();
    let hgt_t = passport.get("hgt").unwrap();
    let hcl_t = passport.get("hcl").unwrap();
    let ecl = passport.get("ecl").unwrap();
    let pid = passport.get("pid").unwrap();

    if pid.len() != 9 {return false};

    let  pid = pid.parse::<isize>();
    if !hgt_t.contains("cm") {if  !hgt_t.contains("in") {return false}};
    let hgt_match = hgt_t.chars().collect::<Vec<char>>();
    let hgt: Vec<String> = vec![
        hgt_t.split(|s| (s == 'c') || (s == 'i')).collect::<String>().split(|s| (s == 'm') || (s == 'n')).collect::<String>(),
        format!("{}{}", hgt_match[hgt_match.len()-2], hgt_match[hgt_match.len()-1].to_string() ) 
    ];
    if hgt.len() != 2 {return false};
    if hcl_t.len() != 7 {return false};
    let hcl = hcl_t.trim_start_matches("#");
    if hcl.len() != 6 {return false};
    let hcl = isize::from_str_radix(hcl, 16);

    let byr = match byr {
        Ok(byr_r) => byr_r,
        Err(_) => return false
    };
    let iyr = match iyr {
        Ok(iyr_r) => iyr_r,
        Err(_) => return false
    };
    let eyr  =match eyr {
        Ok(eyr_r) => eyr_r,
        Err(_) => return false
    };
    match hgt[0].parse::<isize>(){
        Ok(_) => {},
        Err(_) => return false,
    };
    match pid {
        Ok(pid_r) => pid_r,
        Err(_) => return false
    };
    match hcl {
        Ok(hcl_r) => hcl_r,
        Err(_) => return false
    };

    if byr < 1920  || byr > 2002  {return false};
    if iyr < 2010  || iyr > 2020  {return false};
    if eyr < 2020  || eyr > 2030   {return false};
    if hgt[1] == "cm"{
        let height = hgt[0].parse::<isize>().unwrap();
        if  height < 150 || height > 193 {return false};
    }
    if hgt[1] == "in"{
        let height = hgt[0].parse::<isize>().unwrap();
        if height < 59 || height > 76  {return false};
    }
    if vec!["amb".to_string(), "blu".to_string(), "brn".to_string(), "gry".to_string(), "grn".to_string(), "hzl".to_string(), "oth".to_string(), ].into_iter().find(|x| x == ecl) == None {return false};
    
    return true;

}

pub fn boarding_pass_id(boarding_pass: String) -> Result<isize, String> {
    if boarding_pass.len() != 10 {return Err("invlaid boarding pass".to_string())};
    // println!("{}", &boarding_pass[0..6]);
    // let row = i8::from_str_radix(
    //     &boarding_pass[0..6].chars().map(|c| if c == 'F' {'0'} else {'1'}
    //     ).collect::<Vec<char>>().into_iter().collect::<String>(), 2);
    // let column = i8::from_str_radix(
    //     &boarding_pass[0..6].chars().map(|c| if c == 'F' {'0'} else {'1'}
    //     ).collect::<Vec<char>>().into_iter().collect::<String>(), 2);
    // println!("{}",&boarding_pass); 
    let boarding_id =  &boarding_pass[0..10].chars().map(|c| if (c == 'F') || (c == 'L') {'0'} else {'1'}).collect::<String>();
    // .collect::<Vec<char>>().into_iter().collect::<String>();
    // let row = i8::from_str_radix(row, 2);
    // println!("{:?} {:?} {:?} {:?} {:?}", &boarding_id, &boarding_id[0..7], isize::from_str_radix(&boarding_id[0..7], 2) ,  &boarding_id[6..10], isize::from_str_radix(&boarding_id[6..10], 2));
    
    // let column  = &boarding_pass[7..9].chars().map(|c| if c == 'L' {'0'} else {'1'}).collect::<Vec<char>>().into_iter().collect::<String>();
    // let 
    // println!("{:?}", column);
    // let mut row: isize = 0;
    // let mut seat: isize = 0;
    // let  boarding_pass_i = &boarding_pass.chars().collect::<Vec<char>>();

    // if boarding_pass_i[0] == 'B' {
    //     row += 64
    // }
    // if boarding_pass_i[1] == 'B' {
    //     row += 32
    // }
    // if boarding_pass_i[2] == 'B' {
    //     row += 16
    // }
    // if boarding_pass_i[3] == 'B' {
    //     row += 8
    // }
    // if boarding_pass_i[4] == 'B' {
    //     row += 4
    // }
    // if boarding_pass_i[5] == 'B' {
    //     row += 2
    // }
    // if boarding_pass_i[6] == 'B' {
    //     row += 1
    // }
    
    // if boarding_pass_i[7] == 'R' {
    //     seat += 4
    // }
    // if boarding_pass_i[8] == 'R' {
    //     seat += 2
    // }
    // if boarding_pass_i[9] == 'R' {
    //     seat += 1
    // }
    // panic!();
    return Ok((isize::from_str_radix(&boarding_id[0..7], 2).unwrap() * 8) + isize::from_str_radix(&boarding_id[6..10], 2).unwrap())
    // return Ok(1)
}

pub fn find_largest_boarding_pass_id(boarding_id: Vec<String>) -> isize{
    let mut largest = 0;
    for i in boarding_id.into_iter(){
        let current = match boarding_pass_id(i){
            Ok(val) => val,
            _ => continue
            
        };
        if current > largest {
            largest = current;
        }
    }
    return largest
}

pub fn find_my_boarding_pass(boarding_id: Vec<String>) -> isize{
    let mut list: Vec<isize> = Vec::new();
    let mut mine = 0;
    for i in boarding_id.into_iter(){
        let current = match boarding_pass_id(i){
            Ok(val) => val,
            _ => continue
            
        };
        list.push(current);
    }
    let list_copy = list.clone();
    println!("{:?}", list_copy);
    let mut candiates: Vec<isize> = Vec::new();
    for i in list.into_iter(){
        if !list_copy.contains(&(i+1)){
            candiates.push(i)
        }
        // if !list_copy.contains(&(i-1)){
        //     candiates.push(i)
        // }
    }
    // println!("{:?}", candiates);
    for i in candiates.into_iter().permutations(2).into_iter() {
        if i[1]-i[0] ==2 {
            mine = i[0] + 1;
        }
    }

    return mine
}