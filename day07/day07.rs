use std::{fs, path::Path, usize};

// Convertie en tableau de binaire
fn operator_position(size: usize,mut n: usize) -> Vec<bool> {
    let bits: Vec<bool> = (0..size).map(|_| {
        let r = (n % 2) == 1; 
        n = n / 2; 
        r
    }).collect();
    bits
} 

fn verif_equation(result:usize, calc: Vec<&str>) -> bool {
    let mut calc_usize: Vec<usize> = Vec::new();
    let mut is_verif= false;
    calc_usize.resize(calc.len(), 0);
    for i in 0..calc.len() {
        calc_usize[i] = calc[i].parse().unwrap();
    }

    let len: usize = calc.len();
    let binary: usize = 2;
    //println!("{len}");
    for i in 0..binary.pow((len) as u32 )  {
        let position = operator_position(calc.len(), i);
        let mut total = calc_usize[0];
        
        // Additionne ou multiplie chaque possibiliter
        for i in 1..len {
            if position[i-1] {
                total += calc_usize[i];
            } else {
                total *= calc_usize[i];
            }
        }
        //println!("{:?}", calc_usize);

        //println!("{total} -> {result}");
        if result == total {
            is_verif = true;
            break;
        }
    }

    return is_verif;
}

fn read_input(file_path: &Path) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("File not exist");
    return contents.lines().map(|s| s.to_string()).collect::<Vec<_>>();
}

fn main() {
    let file_path =  Path::new("day07/input.txt");
    println!("In file {}", file_path.display());
    let input = read_input(file_path);
    let mut total: usize = 0;
    println!("Teste binaire{:?}", operator_position(4, 2));
    for line in input {
        let fragment_line: Vec<&str> = line.split(':').collect();
        if verif_equation(fragment_line[0].parse().unwrap(), fragment_line[1].split_whitespace().collect()) {
            total += fragment_line[0].parse::<usize>().unwrap();
        }
    }
    println!("part 1 -> {total}");
}