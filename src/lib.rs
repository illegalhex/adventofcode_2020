use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

pub fn count_of_trees(down: usize, right: usize, tree_map: Vec<String>)-> usize{
    let mut tree_count = 0;
    let mut right_offset = 0;
    for i in tree_map.into_iter() {
        if right_offset > i.len()-1 {
            right_offset = 0;
        }
        let terrain = i .chars().nth(right_offset);
        if terrain == Some('#') {tree_count+=1}
        right_offset += right
    }
    tree_count
}