use core::fmt;
use std::ops::{Add, AddAssign};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Point,
    velocity: Point
}

#[derive(Debug, Clone)]
struct Wordl {
    width: u32,
    heigth: u32,
    robot: Vec<Robot>
}

impl Wordl {
    fn new_world(width: u32, heigth: u32) -> Self {
        Self {width, heigth, robot: Vec::new()}
    }

    fn append_robot(&mut self, robots: &Vec<Robot>) {
        self.robot.append(&mut robots.clone());
    }

    fn quandrants(&self) -> usize {
        let exclude_y = self.heigth / 2;
        let exclude_x = self.width / 2;

        let mut quadrant_0 = 0;
        let mut quadrant_1 = 0;
        let mut quadrant_2 = 0;
        let mut quadrant_3 = 0;


        for y in 0..self.heigth {
            for x in 0..self.width {
                let mut n_robot = 0;
                for robot in &self.robot {
                    if robot.position.x == x as i64 && robot.position.y == y as i64 {
                        if x < exclude_x && y < exclude_y {
                            quadrant_0 += 1;
                        }

                        if x > exclude_x && y < exclude_y {
                            quadrant_1 += 1
                        }

                        if x < exclude_x && y > exclude_y {
                            quadrant_2 += 1;
                        }

                        if x > exclude_x && y > exclude_y {
                            quadrant_3 += 1
                        }

                        n_robot += 1;
                    }   
                }
            }
        }

        quadrant_0 * quadrant_1 * quadrant_2 * quadrant_3
    }

    fn update(&mut self) {
        for robot in &mut self.robot {
            let mut new_position = robot.position + robot.velocity;
            if new_position.x < 0 {
                new_position.x = self.width as i64 + new_position.x;
            } else if new_position.x >= self.width as i64 {
                new_position.x = 0 + (new_position.x - self.width as i64);
            } 

            if new_position.y < 0 {
                new_position.y = self.heigth as i64 + new_position.y;
            } else if new_position.y >= self.heigth as i64 {
                new_position.y = 0 + (new_position.y - self.heigth as i64);
            }
            robot.position = new_position;
        }
    }
}

impl fmt::Display for Wordl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display: String = String::new();

        for y in 0..self.heigth {
            for x in 0..self.width {
                let mut n_robot = 0;
                for robot in &self.robot {
                    if robot.position.x == x as i64 && robot.position.y == y as i64 {
                        n_robot += 1;
                    }
                    
                }
                if n_robot != 0 {
                    display += &format!("{}", n_robot);
                } else {
                    display += ".";
                }
            }
            display += "\n";
        }

        write!(f, "({}x{})\n{}", self.width, self.heigth, display)
    }
}

fn read_input(file_path: &Path) -> Vec<Robot> {
    let contents = fs::read_to_string(file_path).expect("File not exist");

    let robots: Vec<Robot> = contents.lines().map(|s| { 
        let s: String = s.to_string(); 
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        let position: Vec<i64> = parts[0].trim_matches(['p', '=']).split(',').map(|s| s.parse::<i64>().unwrap() ).collect();
        let velocity: Vec<i64> = parts[1].trim_matches(['v', '=']).split(',').map(|s| s.parse::<i64>().unwrap() ).collect();

        Robot {
            position: Point {x: position[0], y: position[1]},
            velocity: Point { x: velocity[0], y: velocity[1] }
        }

    }).collect::<Vec<_>>();
    return robots;
}

fn main() {
    let file_path = Path::new("day14/input.txt");
    println!("In file {}", file_path.display());
    let list_robot = read_input(file_path);

    let mut world_robot = Wordl::new_world(101, 103);
    world_robot.append_robot(&list_robot);
    /* 
    let mut world_robot_test = Wordl::new_world(11, 7);
    let r: Vec<Robot> = vec![Robot {
        position: Point { x: 2, y: 4 },
        velocity: Point { x: 2, y: -3 }
    }];

    world_robot_test.append_robot(&r);

    for i in 0..5 {
        println!("Stape -> {i}");
        world_robot_test.update();
        println!("{world_robot_test}");
    }
    */

    for _ in 0..100 {
        world_robot.update();
    }

    println!("{world_robot}");
    println!("{}", world_robot.quandrants());
}