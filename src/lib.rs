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
    if check_passport_fields(passport_clone) == false {return false;} 
    // if 2020 >= passport.get("byr").unwrap()parse::<i32>(); 
    let mut byr = passport.get("byr").unwrap().parse::<isize>();
    let mut iyr = passport.get("iyr").unwrap().parse::<isize>();
    let mut eyr = passport.get("eyr").unwrap().parse::<isize>();
    let mut hgt = passport.get("hgt").unwrap();
    let mut hcl = passport.get("hcl").unwrap();
    let mut ecl = passport.get("ecl").unwrap();
    let mut pid = passport.get("pid").unwrap().parse::<isize>();
    // let 
    match byr {
        Ok(byr_r) => byr = Ok(byr_r),
        Err(_) => return false
    }
    match iyr {
        Ok(iyr_r) => iyr = Ok(iyr_r),
        Err(_) => return false
    }
    match eyr {
        Ok(eyr_r) => eyr = Ok(eyr_r),
        Err(_) => return false
    }
    // match hgt {
    //     Ok(hgt_r) => hgt = Ok(hgt_r),
    //     Err(_) => return false
    // }
    match pid {
        Ok(pid_r) => pid = Ok(pid_r),
        Err(_) => return false
    }
    println!("{:?}", byr );
    println!("{:?}", iyr );
    println!("{:?}", eyr );
    println!("{:?}", hgt );

    println!("{:?}", hcl );
    println!("{:?}", ecl );
    println!("{:?}", pid );
    return true;
}

