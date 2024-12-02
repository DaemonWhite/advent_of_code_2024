use std::fs;
use std::path::Path;
use std::vec;


fn is_safe(report: &Vec<i32>) -> bool {

    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..report.len() - 1 {
        let diff = (report[i] - report[i + 1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        if report[i] < report[i + 1] {
            decreasing = false;
        }
        if report[i] > report[i + 1] {
            increasing = false;
        }
    }

    increasing || decreasing
}

fn read_liste(file_path: &Path) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(file_path).expect("File not exist");
    let lines = contents.lines().map(|s| s.split_whitespace()).collect::<Vec<_>>();

    let mut liste: Vec<Vec<i32>> = vec![];
    let mut index: usize = 0;

    for line in lines {
        liste.push(vec![]);
        for elem in line {
            liste[index].push(elem.parse().unwrap());
        }
        index += 1
    }
    //println!("{:?}",liste);
    liste
}

fn main() {

    let file_path = Path::new("day02/_input.txt");
    println!("In file {}", file_path.display());
    let listes = read_liste(file_path);

    println!("{}", listes.iter().filter(|&liste| is_safe(liste)).count());

}
