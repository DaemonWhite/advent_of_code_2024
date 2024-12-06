use std::fs;
use std::ops::AddAssign;
use std::path::Path;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

enum POSITION {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

struct Guard {
    position: Point,
    next_postion: Point,
    collider: char,
    limit_start_wordl: Point,
    limit_end_world: Point,
    orientation: POSITION
}

impl Guard {
    fn new(position: Point, collider: char, limit_start_wordl: Point, limit_end_world: Point) -> Self {
        Self {position, next_postion: Point {x: -1, y: 0} ,collider,limit_start_wordl, limit_end_world,orientation: POSITION::UP}
    }

    fn rotate(&mut self) {
        match self.orientation {
            POSITION::UP    => self.orientation = POSITION::RIGHT,
            POSITION::RIGHT => self.orientation = POSITION::DOWN,
            POSITION::DOWN  =>  self.orientation = POSITION::LEFT,
            POSITION::LEFT  => self.orientation = POSITION::UP
        }
    }

    fn get_next_position(&mut self) -> Point{
        return self.next_postion;
    }

    fn translate(&mut self) {
        match self.orientation {
            POSITION::UP    => self.position += Point {x: -1, y: 0},
            POSITION::RIGHT => self.position += Point {x: 0, y: 1},
            POSITION::DOWN  =>  self.position += Point {x: 1, y: 0},
            POSITION::LEFT  => self.position += Point {x:0, y: -1}
        }
    }

    fn verif_limit(&self) -> bool {
        match self.orientation {
            POSITION::UP    => self.position.x <= self.limit_start_wordl.x,
            POSITION::RIGHT => self.position.y >= self.limit_end_world.y,
            POSITION::DOWN  =>  self.position.x >= self.limit_end_world.x,
            POSITION::LEFT  => self.position.y <= self.limit_start_wordl.y
        }
    }

    fn move_guard(&mut self, present: char) {

        if present == self.collider {
            self.rotate();
        }

        self.translate();
    }
}

struct GuardMap {
    totla_move: usize,
    map: Vec<String>,
    guard: Guard,
}

impl GuardMap {
    fn new(map: Vec<String>) -> Self {
        let mut guard = Guard::new(
            Point {x:0,y:0},
            '#',
            Point {x:0,y:0},
            Point {x:0,y:0}
        );
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i].chars().nth(j).unwrap() == '^' {
                    guard = Guard::new(
                        Point {x: i as isize,y: j as isize}, 
                        '#', 
                        Point {x: 0,y: 0}, 
                        Point{x: (map.len()-1) as isize , y: (map[i].len() -1) as isize});
                    break
                }
            }
        }
        Self {totla_move: 0, map: map, guard: guard}
    }
    
    fn play(&mut self) {
        let desination = false;
        let mut count = 0;
        while !self.guard.verif_limit() {
            let i = self.guard.get_next_position();
            println!("{:?}", i);
            self.guard.move_guard(self.map[i.x as usize].chars().nth(i.y as usize).unwrap() );
            count += 1
        }
        println!("{count}")
    }
   
}

fn read_input(file_path: &Path) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("File not exist");
    return contents.lines().map(|s| s.to_string()).collect::<Vec<_>>();
}

fn main() {
    let file_path = Path::new("day06/_input.txt");
    println!("In file {}", file_path.display());
    let map = read_input(file_path);
    let mut guard_map = GuardMap::new(map);
    guard_map.play()
}