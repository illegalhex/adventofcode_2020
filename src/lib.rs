#![feature(iterator_fold_self)]
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};

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
    if boarding_pass.len() != 10 {return Err("invalid boarding pass".to_string())};
    let boarding_id =  &boarding_pass[0..10].chars().map(|c| if (c == 'F') || (c == 'L') {'0'} else {'1'}).collect::<String>();
    return Ok((isize::from_str_radix(&boarding_id[0..7], 2).unwrap() * 8) + isize::from_str_radix(&boarding_id[7..10], 2).unwrap())
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
    let mut candiates: Vec<isize> = Vec::new();
    for i in list.into_iter(){
        if !list_copy.contains(&(i+1)){
            candiates.push(i)
        }
        if !list_copy.contains(&(i-1)){
            candiates.push(i)
        }
    }
    for i in candiates.into_iter().permutations(2).into_iter() {
        if i[1]-i[0] ==2 {
            mine = i[0] + 1;
        }
    }

    return mine
}

pub fn find_number_of_yeses(card_type: usize, question_cards: Vec<String>) -> usize {
    let mut count = 0;
    if card_type == 0 {
        let qc_split_on_nl : Vec<String> = question_cards.into_iter().map(|s| if s == "" {s.replace("",",")}else{s}).collect();
        let qc_split_on_comma: Vec<String> = qc_split_on_nl.concat().split(',').map(String::from).collect();
        let question_cards_vecs = qc_split_on_comma.iter().map(|s| s.chars().collect::<Vec<char>>()).
                                    collect::<Vec<Vec<char>>>();

        for vector in &question_cards_vecs {
            let mut temp_set = HashSet::new();
            for element in vector {
                temp_set.insert(element);
            }
            count += temp_set.len()
        }
        return count
    }
    else if card_type == 1  {
        let qc_split_on_nl : Vec<String> = question_cards.into_iter().map(|s| if s == "" {s.replace("","\n")}else{format!("{},",s)}).collect();
        let qc_text_join: Vec<String> = qc_split_on_nl.concat().split('\n').map(String::from).collect();
        let mut question_cards_cleaned: Vec<String> = Vec::new();
        for mut group in qc_text_join.into_iter() {
            group.pop();
            question_cards_cleaned.push(group);
        }
        for group in question_cards_cleaned.into_iter() {
            let mut temp_vec: Vec<HashSet<char>> = Vec::new();
            let pass_group = group.split(",").map(String::from).collect::<Vec<String>>();
            for pass in &pass_group{
                let mut temp_hash = HashSet::new();
                let temp_vec_char: Vec<char> = pass.chars().collect::<Vec<char>>();
                for i in temp_vec_char {
                    temp_hash.insert(i);
                }
                temp_vec.push(temp_hash);
            }
            let hash_inter = temp_vec.into_iter().fold1(set_intersect).unwrap();
            count += hash_inter.len();
        }
        return count
    }
    else {
            return 0
        }
}

fn set_intersect(a:HashSet<char>, b:HashSet<char>) -> HashSet<char> {
    let intersection = a.intersection(&b);
    let mut temp_hash_set = HashSet::new();
    intersection.for_each(|c| {temp_hash_set.insert(c.clone());});
    temp_hash_set
}


pub fn find_number_gold_bags(a: Vec<String>) -> isize{
    return 1
}