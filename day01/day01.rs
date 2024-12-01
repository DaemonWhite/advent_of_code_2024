use std::collections::btree_map::{Keys, Values};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::vec;

fn calc_distance(a:usize, b:usize) -> usize{
    let mut r: usize = 0;
    if a > b {
        r =  a - b;
    } else {
        r = b - a;
    }
    return r;
}


fn count_element(listes: &Vec<Vec<usize>>) -> Vec<HashMap<usize, usize>>{
    let mut liste_elems: Vec<HashMap<usize, usize>> = vec![HashMap::new(), HashMap::new()];
    for index in 0..listes[0].len() {
        if listes[1].contains(&listes[0][index]) {
            liste_elems[0].entry(listes[0][index]).and_modify(|i| *i += 1).or_insert(1);
            // liste_elems.entry(listes[1][index]).and_modify(|i| *i += 1).or_insert(1);
        }
        if listes[0].contains(&listes[1][index]) {
            liste_elems[1].entry(listes[1][index]).and_modify(|i | *i += 1).or_insert(1);
        }
    }
    liste_elems
}

fn read_liste(file_path: &Path) -> Vec<Vec<usize>> {
    let contents = fs::read_to_string(file_path).expect("File not exist");
    let lines = contents.lines().map(|s| s.split_whitespace()).collect::<Vec<_>>();

    let mut liste: Vec<Vec<usize>> = vec![ vec![], vec![]];

    for line in lines {
        let mut row: Vec<usize> = vec![];
        for elem in line {
            row.push(elem.parse().unwrap());
        }
        liste[0].push(row[0]);
        liste[1].push(row[1]);
    }
    liste
}

fn main() {

    let file_path = Path::new("day01/input.txt");
    println!("In file {}", file_path.display());
    let mut listes = read_liste(file_path);
    let list_items = count_element(&listes);


    listes[0].sort();
    listes[1].sort();

    let mut sum_distance: usize = 0;
    let mut mutiply_distance: usize = 0;

    for index in 0..listes[0].len() {

        sum_distance += calc_distance(listes[0][index], listes[1][index]);
    }

    for (key, value) in &list_items[0]  {
        mutiply_distance += key * list_items[0].get(key).unwrap() * list_items[1].get(key).unwrap();
    }

    println!("Sum distance : {}",sum_distance);
    println!("Mult distance : {}", mutiply_distance);
}
