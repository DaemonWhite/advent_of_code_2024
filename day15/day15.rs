use core::fmt;
use std::fs;
use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn UP() -> Self {
        Point {
            x: 0,
            y: -1
        }
    }
    fn DOWN() -> Self {
        Point {
            x: 0,
            y: 1
        }
    }
    fn LEFT() -> Self {
        Point {
            x: -1,
            y: 0
        }
    }
    fn RIGHT() -> Self {
        Point {
            x: 1,
            y: 0
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Mul for Point {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl MulAssign for Point {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

struct World {
    robot: Point,
    boxes: Vec<Point>,
    collider: Vec<Point>,
    width: u32,
    heigth: u32
}

impl World {
    fn new(map: &Vec<Vec<char>>) -> Self {
        let mut robot: Point = Point { x: 0, y: 0 };
        let mut boxes: Vec<Point> = Vec::new();
        let mut collider: Vec<Point> = Vec::new();
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if '@' == map[y][x] {
                    robot = Point {x: x as i64, y: y as i64};
                } else if 'O' == map[y][x] {
                    boxes.push(Point {x: x as i64, y: y as i64} );
                } else if '#' == map[y][x]{
                    collider.push(Point { x: x as i64, y: y as i64 });
                }
            }
        }

        Self {robot, boxes, collider, width: map[0].len() as u32, heigth: map.len() as u32}
    }

    fn sum_coordonaite_box(&self) -> usize {
        let mut sum: usize = 0;
        for boxe in &self.boxes {
            sum += 100 * boxe.y as usize + boxe.x as usize;
        }
        sum
    }

    fn translate(&mut self, move_posision: Point, position: &mut Point) {
        let mut is_move= false;
        let mut is_movable= true; 
        let mut move_point: Vec<usize> = Vec::new();

        let mut offset = Point {x: 1, y: 1};

        while is_movable {
            let nex_position = &(*position + move_posision * offset);
            println!("{:?}", nex_position);
            if self.collider.contains( nex_position ) {
                is_movable = false;
            } else if self.boxes.contains(nex_position) {
                move_point.push(
                    self.boxes.iter().position( |f| nex_position == f ).unwrap()
                );

            } else {
                is_move = true;
                is_movable = false;
            }
            offset += Point {x:1, y:1};
        }
        offset = Point {x: 1, y: 1};

        if is_move {
            println!("a");
            *position += move_posision;
            for i in move_point {
                self.boxes[i] = *position + move_posision * offset;
                println!("{:?}", self.boxes[i]);
                offset += Point {x:1, y:1};
            }
        }
    }

    fn move_robot(&mut self, instruct: Move) {
        let mut robot = self.robot.clone();
        match instruct {
            Move::UP => { self.translate(Point::UP(), &mut robot) },
            Move::DOWN => { self.translate(Point::DOWN(), &mut robot) },
            Move::LEFT => { self.translate(Point::LEFT(), &mut robot) },
            Move::RIGHT => { self.translate(Point::RIGHT(), &mut robot) }
        }
        self.robot = robot;
    }

}

impl fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map_string: String = String::with_capacity((self.width * self.heigth + self.heigth) as usize);

        let mut map: Vec<Vec<char>> = Vec::with_capacity(self.heigth as usize);

        for _ in 0..self.heigth {
            let mut row: Vec<char> = Vec::new();
            row.resize(self.width as usize, '.');
            map.push(row);
        }

        map[self.robot.y as usize][self.robot.x as usize] = '@';

        for boxe in self.boxes.clone() {
            map[boxe.y as usize][boxe.x as usize] = 'O'
        }

        for collider in self.collider.clone() {
            map[collider.y as usize][collider.x as usize] = '#';
        }

        for y in 0..self.heigth {
            for x in 0..self.width {
                map_string.push( map[y as usize][x as usize] );
            }
            map_string.push('\n');
        }

        write!(f, "{}x{}\n{}", self.width, self.heigth, map_string)
    }
}

enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn read_input(file_path: &Path) -> (Vec<Vec<char>>, Vec<Move> ) {
    let mut list_move: Vec<Move> = Vec ::new();
    let mut map: Vec<Vec<char>> = Vec::new();
    let contents = fs::read_to_string(file_path).expect("File not exist");
    let mut switch_map = true;
    for line in contents.lines() {
        if line.len() == 0 {
            switch_map = false;
        }
        if switch_map {
            let mut line_char: Vec<char> = Vec::new();
            for c in line.chars() {
                line_char.push(c);
            }
            map.push(line_char);
        } else {
            for c in line.chars() {
                match c {
                    '^' => list_move.push(Move::UP),
                    'v' => list_move.push(Move::DOWN),
                    '>' => list_move.push(Move::RIGHT),
                    '<' => list_move.push(Move::LEFT),
                    _ => todo!() 
                }
            }
        }
    }
    return (map, list_move);
}

fn main() {
    let file_path = Path::new("day15/input.txt");
    println!("In file {}", file_path.display());
    let (map, move_robot) = read_input(file_path);
    println!("{:?}", map);
    let mut world = World::new(&map);


    for mv in move_robot {
        world.move_robot(mv);
    }

    

    println!("{world} part 1 -> {} ", world.sum_coordonaite_box());
}