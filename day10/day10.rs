use std::path::Path;
use std::fs;

use std::ops::{Add, AddAssign};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Orientation {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize,
    orientation: Orientation
}


impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            orientation: self.orientation
        }
    }
}


impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            orientation: self.orientation
        };
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct Starter {
    position: Point,
    other_path: Vec<Point>,
    count: u8,
}

fn read_input(file_path: &Path) -> Vec<Vec<u8>> {
    let contents = fs::read_to_string(file_path).expect("File not exist");
    return contents.lines().map(|s| {
        let line = s.to_string();
        
        let mut line_int: Vec<u8> = Vec::new();
        for i in 0..line.len() {
            let c: String = line.chars().nth(i).unwrap().to_string();
            line_int.push(c.parse::<u8>().unwrap());
        }
        line_int
    }).collect::<Vec<_>>();
}

impl Starter {
    fn new(position: Point) -> Self {
        Self { position: position, other_path: vec![], count: 0}
    }

    fn set_map(&self, map: &Vec<String>) {

    }

    fn translate(&mut self) {
        match self.position.orientation {
            Orientation::UP    => self.position += Point {x: -1, y: 0, orientation : self.position.orientation},
            Orientation::RIGHT => self.position += Point {x: 0, y: 1, orientation : self.position.orientation},
            Orientation::DOWN  =>  self.position += Point {x: 1, y: 0, orientation : self.position.orientation},
            Orientation::LEFT  => self.position += Point {x:0, y: -1, orientation : self.position.orientation}
        }
    }

    fn scan_position(&mut self, map: &Vec<Vec<u8>>,  x: isize, y:isize, orientation: Orientation, next_position: u8){
        if map[x as usize][y as usize] ==  next_position {
            self.other_path.push(Point { 
                x: x,
                y: y,
                orientation: orientation
            });
        }
    }

    fn scan(&mut self, map: &Vec<Vec<u8>>) {
        let next_position = self.count + 1;

        if self.position.x > 0 {
            self.scan_position(map, self.position.x - 1, self.position.y, Orientation::UP, next_position);
        }

        if self.position.x < (map.len() - 1) as isize {
            self.scan_position(map, self.position.x - 1, self.position.y, Orientation::DOWN, next_position);
        }

        if self.position.y > 0 {
            self.scan_position(map, self.position.x, self.position.y - 1, Orientation::LEFT, next_position);
        }

        if self.position.y < (map[0].len() - 1) as isize {
            self.scan_position(map, self.position.x, self.position.y + 1, Orientation::RIGHT, next_position);
        }

    }

    fn advandced(&mut self, map: &Vec<Vec<u8>>) {
        self.scan(map);
    }

}

fn search_starter(map: &Vec<String>) -> Vec<Starter>  {
    let mut starter_point: Vec<Starter> = Vec::new();
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            let c = map[x].chars().nth(y).unwrap();
            if c=='0' {
                starter_point.push(Starter::new(Point { x: x as isize, y: y as isize, orientation: Orientation::DOWN }));
            }
        } 
    }
    starter_point
}

fn main() {
    let file_path = Path::new("day10/input.txt");
    println!("In file {}", file_path.display());
    let map: Vec<Vec<u8>> = read_input(file_path);
    let starters: Vec<Starter> = search_starter(&map);

    println!("{:?}", starters);

    for starter in starters {

        starter.advandced(&map);
    }
}