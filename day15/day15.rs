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


#[derive(Debug, Clone, Copy)]
struct Boxe {
    position_forward: Point,
    position_extended: u64
}

impl Boxe {
    fn translate(&mut self, p: Point) {
        self.position_forward += p;
    }

    fn contains(&self, p: Point) -> bool {
        if self.position_forward == p {
            return true;
        }

        for i in 1..(self.position_extended + 1) {
            if (self.position_forward + Point {x: i as i64, y: 0}) == p {
                return true;
            }
        }
        
        false
    }
}

struct World {
    robot: Point,
    boxes: Vec<Boxe>,
    collider: Vec<Point>,
    width: u32,
    heigth: u32,
    extended: bool
}

impl World {
    fn new(map: &Vec<Vec<char>>, extended: bool) -> Self {
        let mut robot: Point = Point { x: 0, y: 0 };
        let mut boxes: Vec<Boxe> = Vec::new();
        let mut collider: Vec<Point> = Vec::new();
        let mut offset = 1;
        if extended {
            offset = 2;
        }

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if '@' == map[y][x] {
                    robot = Point {x: x as i64 * offset , y: y as i64};
                } else if 'O' == map[y][x] {
                    boxes.push(Boxe { position_forward: Point {x: x as i64 * offset, y: y as i64 }, position_extended:  if extended {1} else {0}} );
                } else if '#' == map[y][x]{
                    collider.push(Point { x: x as i64 * offset, y: y as i64});
                    if extended {collider.push(Point { x: x as i64 * offset + 1, y: y as i64}); }
                }
            }
        }

        Self {robot, boxes, collider, width: map[0].len() as u32 * offset as u32 + 1, heigth: map.len() as u32, extended}
    }

    fn sum_coordonaite_box(&self) -> usize {
        let mut sum: usize = 0;
        if !self.extended {
            for boxe in &self.boxes {
                sum += 100 * boxe.position_forward.y as usize + boxe.position_forward.x as usize;
            }
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
            if self.collider.contains( nex_position ) {
                is_movable = false;
            } else if self.boxes.iter().find(|f| { f.contains(*nex_position) } ).is_some() {
                let position = self.boxes.iter_mut().position( |f| f.contains(*nex_position) ).unwrap();
                
                print!("{position}->");

                /*
                .......
                ..OOOO.
                ...OO..
                ....@..
                ....^..
                Le problème et que seul les objet toucher par le point d'impacte sont toucher
                ce qui donne 
                ....OO.
                ..OOO.. Il faut donc trouver un moyent de faire avancer les boites sans qu'il y'est de conflit
                ....@..
                .......
                ....^..
                 */

                if !move_point.contains(&position) {
                    move_point.push(
                        position
                    );
                } 
                

            } else {
                is_move = true;
                is_movable = false;
            }

            offset += Point {x:1, y:1};
        }

        if is_move {
            *position += move_posision;
            for i in move_point {
                self.boxes[i].translate(move_posision);
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
            map[boxe.position_forward.y as usize][boxe.position_forward.x as usize] = 'O';
            for i in 1..(boxe.position_extended + 1) {
                map[boxe.position_forward.y as usize][boxe.position_forward.x as usize + i as usize ] = 'O';
            }
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

#[derive(Clone, Copy)]
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

/*

Doit êtres fait avec une boxe pour syncroniser la boite cortement

 */


fn main() {
    let file_path = Path::new("day15/_input3.txt");
    println!("In file {}", file_path.display());
    let (map, move_robot) = read_input(file_path);
    println!("{:?}", map);
    let mut world = World::new(&map, false);
    let mut second_world = World::new(&map, true);

    println!("{second_world}");

    for mv in move_robot {
        world.move_robot(mv);
        second_world.move_robot(mv);
        println!("{second_world}");
    }

    

    println!("{world} \n part 1 -> {} ", world.sum_coordonaite_box());
    println!("{second_world} \n part 2 -> {}", second_world.sum_coordonaite_box());
}