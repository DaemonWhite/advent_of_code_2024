use std::fs;
use std::path::Path;
use std::vec;

struct SearchXmax {
    word: Vec<char>,
    rword: Vec<char>,
    count: usize,
    input: Vec<String>
}

impl SearchXmax {
    fn new(word: Vec<char>) -> Self {
        let mut reverse_word: Vec<char> = vec![];
        let len = word.len()-1;
        for i in 0..len+1 {
            reverse_word.push(word[len-i]);
        } 
        println!("{:?}", reverse_word);

        Self {word: word, rword: reverse_word, count: 0, input: vec![]}
    }

    fn set_input(&mut self, input: Vec<String>) {
        self.input = input;
    }

    fn get_count(&self) -> usize {
        return self.count;
    }

    fn read_line(&mut self, start_col: usize, start_line: usize) {
        let mut total_word = 0;
        let mut total_rword = 0;

        let mut index_word = 0;
        let mut index_rword = 0;

        for i in 0..self.word.len()  {
            let letter_input = self.input[start_line].chars().nth(start_col + i).unwrap();
            if letter_input == self.word[index_word] {
                index_word += 1;
                total_word += 1;
            } 

            if letter_input == self.rword[index_rword] {
                index_rword += 1;
                total_rword += 1;
            }

            if total_rword == 0 && total_word == 0 {
                break;
            }
        }

        if total_word == 4 {
            self.count += 1
        }
        if total_rword == 4 {
            self.count += 1
        }
    }

    fn read_col(&mut self, start_col: usize, start_line: usize) {
        let mut total_word = 0;
        let mut total_rword = 0;

        let mut index_word = 0;
        let mut index_rword = 0;

        for i in 0..self.word.len()  {
            let letter_input = self.input[start_line + i].chars().nth(start_col).unwrap();
            if letter_input == self.word[index_word] {
                index_word += 1;
                total_word += 1;
            } 

            if letter_input == self.rword[index_rword] {
                index_rword += 1;
                total_rword += 1;
            }

            if total_rword == 0 && total_word == 0 {
                break;
            }
        }

        if total_word == 4 {
            self.count += 1
        }
        if total_rword == 4 {
            self.count += 1
        }

    }

    fn read_diag(&mut self, pos_offset: isize, start_col: usize, start_line: usize) {
        let mut count_word = 0;
        let mut count_rword = 0;
        let mut offset_j: isize = 0;

        for i in 0..self.word.len() {
            let offset = start_line + i;
            let letter_input = self.input[offset].chars().nth((start_col as isize + offset_j) as usize).unwrap();

            if letter_input == self.word[count_word] {
                    // println!("{} {count_word} -> {i} {start_line}, {offset}", self.word[count_word]);
                    count_word += 1;
            }

            if letter_input == self.rword[count_rword] {
                // println!("{} {count_word} -> {i} {start_line}, {offset}", self.word[count_word]);
                count_rword += 1;
            }

            if count_rword == 0 && count_word == 0 {
                break;
            }
            
            offset_j += 1 * pos_offset;
        }
        if count_word == 4 {
            self.count += 1
        }
        if count_rword == 4 {
            self.count += 1
        }

    }

    fn search(&mut self) {
        let len_word = self.word.len();
        for index_line in 0..self.input.len() {
            for index_character in 0..self.input[index_line].len() {

                let limit_end_colone: bool = self.input[index_line].len() - (len_word-1) > index_character;
                let limit_start_colone: bool = index_character >= (len_word-1);
                let limit_end_line: bool = index_line < self.input.len() - (len_word-1);

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

struct SearchXmas {
    word: Vec<char>,
    rword: Vec<char>,
    count: usize,
    input: Vec<String>
}

impl SearchXmas {
    fn new(word: Vec<char>) -> Self {
        let mut reverse_word: Vec<char> = vec![];
        let len = word.len()-1;
        for i in 0..len+1 {
            reverse_word.push(word[len-i]);
        } 
        println!("{:?}", reverse_word);

        Self {word: word, rword: reverse_word, count: 0, input: vec![]}
    }

    fn set_input(&mut self, input: Vec<String>) {
        self.input = input;
    }

    fn get_count(&self) -> usize {
        self.count
    }

    fn read_diag(&mut self, pos_offset: isize, start_col: usize, start_line: usize) -> bool {
        let mut count_word = 0;
        let mut count_rword = 0;
        let mut offset_j: isize = 0;

        let mut is_detected = false;
        let mut is_reverse_detected = false;

        for i in 0..self.word.len() {
            let offset = start_line + i;
            let letter_input = self.input[offset].chars().nth((start_col as isize + offset_j) as usize).unwrap();

            if letter_input == self.word[count_word] {
                    // println!("{} {count_word} -> {i} {start_line}, {offset}", self.word[count_word]);
                    count_word += 1;
            }

            if letter_input == self.rword[count_rword] {
                // println!("{} {count_word} -> {i} {start_line}, {offset}", self.word[count_word]);
                count_rword += 1;
            }

            if count_rword == 0 && count_word == 0 {
                break;
            }
            
            offset_j += 1 * pos_offset;
        }
        println!("{pos_offset} -> {count_word}x{count_rword}");
        if count_word == self.word.len() {
            is_detected = true;
        }
        if count_rword == self.word.len() {
            is_reverse_detected = true;
        }

        is_detected || is_reverse_detected
    }



    fn search(&mut self) {
        for index_line in 0..self.input.len()-2 {
            for index_col in 0..self.input[index_line].len() - 2 {
                if self.read_diag(1, index_col, index_line) 
                && self.read_diag(-1, index_col + 2, index_line)  {
                    self.count += 1
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
    let mut input_search_xmas = SearchXmax::new(vec!['X','M','A','S']) ;
    input_search_xmas.set_input(read_input(file_path));
    input_search_xmas.search();

    let mut input_search_cross_sam = SearchXmas::new(vec!['M','A','S']);
    input_search_cross_sam.set_input(read_input(file_path));
    input_search_cross_sam.search();

    println!("samx: {}", input_search_xmas.get_count());
    println!("X-mas: {}", input_search_cross_sam.get_count());
}
//1982
//2255