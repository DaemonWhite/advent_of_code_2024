use std::{fs, isize};
use std::path::Path;
use std::vec;

use regex::Regex;

fn mul(x:isize, y:isize) -> isize {
    return x * y;
}

fn search_line_mul(line: &str) -> Vec<isize> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    println!("{line}");

    let ok: Vec<isize> = re.find_iter(line).map(|m| {
        let origial_str: &str = m.as_str();
        let mut new_str: String = String::new();
        for i in 4..origial_str.len() - 1 {
            new_str.push( origial_str.chars().nth(i).unwrap());
        }
        let split_str = new_str.as_str().split(",");
        let mut vec_calc: Vec<isize> = vec![];
        for elem in split_str {
            vec_calc.push(elem.parse().unwrap());
        }
        mul(vec_calc[0], vec_calc[1])
    }).collect();

    ok
}

fn search_line_mul_secure(line: &str) -> Vec<Vec<isize>> {
    let mut line_copy  = line.to_string().replace("\n", "");
    line_copy.insert_str(0, "do()");
    line_copy.insert_str(line_copy.len()-1, "don't()");

    let re = Regex::new(r"do\(\)(.*?)don't\(\)").unwrap();

    let ok: Vec<Vec<isize>> = re.find_iter(line_copy.as_str()).map(|m| {
        search_line_mul(m.as_str())
    }).collect();

    ok
}

fn read_mul(file_path: &Path) -> Vec<Vec<isize>> {
    let contents = fs::read_to_string(file_path).expect("File not exist");
    return contents.lines().map(|s| search_line_mul(s)).collect::<Vec<_>>();
}

fn read_mul_secure(file_path: &Path) -> Vec<Vec<isize>> {
    let contents = fs::read_to_string(file_path).expect("File not exist");
    return search_line_mul_secure(contents.as_str())
}

fn main() {

    let file_path = Path::new("day03/input.txt");
    println!("In file {}", file_path.display());
    let vec_muls = read_mul(file_path);
    let vec_muls_secures = read_mul_secure(file_path);
    let mut some_mul: isize = 0; 
    let mut some_mul_secure: isize = 0;

    // simple mutiplicateurs
    for vec_mul in vec_muls {
        for mul in vec_mul {
            some_mul += mul; 
        }
    }

    for vec_mul in vec_muls_secures {
        for mul in vec_mul  {
            some_mul_secure += mul;            
        }
    }

    println!("mul: {some_mul}");
    println!("mul secure: {some_mul_secure}");
}
