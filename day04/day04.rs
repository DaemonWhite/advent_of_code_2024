use std::os::unix::raw::off_t;
use std::{fs};
use std::path::Path;
use std::vec;

struct SearchXmax {
    xmas: Vec<char>,
    samx: Vec<char>,
    count: usize,
    input: Vec<String>
}

impl SearchXmax {
    fn new() -> Self {
        Self {xmas: vec!['X', 'M', 'A', 'S'], samx: vec!['S','A','M','X'], count: 0, input: vec![]}
    }

    fn set_input(&mut self, input: Vec<String>) {
        self.input = input
    }

    fn get_count(&self) -> usize {
        return self.count;
    }

    fn read_line(&mut self, start_col: usize, start_line: usize) {
        let mut total_xmas = 0;
        let mut total_smax = 0;

        let mut index_xmas = 0;
        let mut index_smax = 0;

        for i in 0..4  {
            let letter_input = self.input[start_line].chars().nth(start_col + i).unwrap();
            if letter_input == self.xmas[index_xmas] {
                index_xmas += 1;
                total_xmas += 1;
            } 

            if letter_input == self.samx[index_smax] {
                index_smax += 1;
                total_smax += 1;
            }

            if total_smax == 0 && total_xmas == 0 {
                break;
            }
        }

        if total_xmas == 4 {
            self.count += 1
        }
        if total_smax == 4 {
            self.count += 1
        }
    }

    fn read_col(&mut self, start_col: usize, start_line: usize) {
        let mut total_xmas = 0;
        let mut total_smax = 0;

        let mut index_xmas = 0;
        let mut index_smax = 0;

        for i in 0..4  {
            let letter_input = self.input[start_line + i].chars().nth(start_col).unwrap();
            if letter_input == self.xmas[index_xmas] {
                index_xmas += 1;
                total_xmas += 1;
            } 

            if letter_input == self.samx[index_smax] {
                index_smax += 1;
                total_smax += 1;
            }

            if total_smax == 0 && total_xmas == 0 {
                break;
            }
        }

        if total_xmas == 4 {
            self.count += 1
        }
        if total_smax == 4 {
            self.count += 1
        }

    }

    fn read_diag(&mut self, pos_offset: isize, start_col: usize, start_line: usize) {
        let mut count_xmax = 0;
        let mut count_smax = 0;
        let mut offset_j: isize = 0;

        for i in 0..4 {
            println!("{offset_j}");
            let offset = (start_line as isize + (i)) as usize;
            let letter_input = self.input[offset].chars().nth((start_col as isize + offset_j) as usize).unwrap();

            if letter_input == self.xmas[count_xmax] {
                    // println!("{} {count_xmax} -> {i} {start_line}, {offset}", self.xmas[count_xmax]);
                    count_xmax += 1;
            }

            if letter_input == self.samx[count_smax] {
                // println!("{} {count_xmax} -> {i} {start_line}, {offset}", self.xmas[count_xmax]);
                count_smax += 1;
            }

            if count_smax == 0 && count_xmax == 0 {
                break;
            }
            
            offset_j += 1 * pos_offset;
        }
        if count_xmax == 4 {
            self.count += 1
        }
        if count_smax == 4 {
            self.count += 1
        }

    }

    fn search(&mut self) {

        for index_line in 0..self.input.len() {
            for index_character in 0..self.input[index_line].len() {

                let limit_end_colone: bool = self.input[index_line].len() - 3 > index_character;
                let limit_start_colone: bool = index_character > 2;
                let limit_end_line: bool = index_line < self.input.len() - 3;

                if limit_end_colone {
                    self.read_line( index_character, index_line);
                } 

        
                if limit_end_line{
                    self.read_col( index_character, index_line );
                }
                
                 if limit_end_colone && limit_end_line {
                    self.read_diag(1, index_character, index_line );              
                }
                
                 if limit_start_colone && limit_end_line {
                    self.read_diag(-1, index_character, index_line );              
                }
            }
        }

    }
}



fn read_input(file_path: &Path) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("File not exist");
    return contents.lines().map(|s| s.to_string()).collect::<Vec<_>>();
}

fn main() {

    let file_path = Path::new("day04/input.txt");
    println!("In file {}", file_path.display());
    let mut input_search = SearchXmax::new();
    input_search.set_input(read_input(file_path));
    input_search.search();

    println!("samx: {}", input_search.get_count());
}
//1982
//2255