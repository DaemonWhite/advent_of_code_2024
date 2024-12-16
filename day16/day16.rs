use std::fs;
use std::path::Path;

struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0
        }
    }

    fn UP() -> Self {
        Self {
            x: 0,
            y: -1
        }
    }

    fn DOWN() -> Self {
        Self {
            x: 0,
            y: 1
        }
    }

    fn LEFT() -> Self {
        Self {
            x: -1,
            y: 0
        }
    }

    fn RIGHT() -> Self {
        Self {
            x: 1,
            y: 0
        }
    }
}

fn move_player(player: &mut Point, score: &mut u64, map: &Vec<Vec<char>>) -> bool {


    true
}


fn read_input(file_path: &Path) -> Vec<Vec<char>> {
    let file = fs::read_to_string(file_path).expect("File not exist").to_string();
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        map.push(Vec::new());
        for c in line.chars() {
            let i = map.len() - 1;
            map[i].push(c);
        }
    }

    map
}

fn detected_player(map: &Vec<Vec<char>>) -> Point {
    let mut player = Point::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                player = Point {x: x as i64, y: y as i64};
            }
        }
    }
    player
}

fn main() {
    let file_path = Path::new("day15/_input3.txt");
    println!("In file {}", file_path.display());
    let map = read_input(file_path);
    let mut player: Point = detected_player(&map);
    let mut score: u64 =0;

    while move_player(&mut player, &mut score, &map) {
        
    }

}