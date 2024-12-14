use std::{fs, path::Path, result, usize};

// Convertie en tableau de binaire
fn operator_position(size: usize,mut n: usize, modulo: usize) -> Vec<usize> {
    let bits: Vec<usize> = (0..size).map(|_| {
        let r = n % modulo; 
        n = n / modulo; 
        r
    }).collect();
    bits
} 

fn verif_equation(result:usize, calc: Vec<&str>, n_factor: usize) -> bool {
    let mut calc_usize: Vec<usize> = Vec::new();
    let mut is_verif= false;
    calc_usize.resize(calc.len(), 0);
    for i in 0..calc.len() {
        calc_usize[i] = calc[i].parse().unwrap();
    }

    let len: usize = calc.len();
    let factor: usize = n_factor.pow((len) as u32);
    
    for i in 0..factor  {
        let position = operator_position(calc.len(), i, n_factor);
        let mut total = calc_usize[0];
        
        // Additionne ou multiplie chaque possibiliter
        for i in 1..len {

            match position[i-1] {
                0 => total += calc_usize[i],
                1 => {total *= calc_usize[i]},
                2 => {total = format!("{}{}", total, calc_usize[i]).parse::<usize>().unwrap()  }
                _ => todo!()
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
    let mut total_part_two: usize = 0;
    println!("Teste binaire{:?}", operator_position(4, 2, 2));
    for line in input {
        let fragment_line: Vec<&str> = line.split(':').collect();
        let result = fragment_line[0].parse().unwrap();
        let calc: Vec<&str> = fragment_line[1].split_whitespace().collect();

        if verif_equation(result, calc.clone(), 2) {
            total += result;
        }

        if verif_equation(result, calc, 3){
            total_part_two += result;
        }
    }
    println!("part 1 -> {total}");
    println!("part 1 -> {total_part_two}");
}