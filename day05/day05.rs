use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn generate_order(position_order: &Vec<String>) -> HashMap<usize, Vec<usize>>{
    let mut poid: Vec<usize> = Vec::new();

    let mut order: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in position_order {
        let before_and_after: Vec<usize> = line.split('|').map(|s| s.parse().unwrap() ).collect();
        println!("{:?}", before_and_after);
        order.entry(before_and_after[0]).or_default().push(before_and_after[1]);
        order.entry(before_and_after[1]).or_default();
    }

    println!("{:?}", order);
    
    order
}

fn midle_point(order_queue: &HashMap<usize, Vec<usize>>, queue: &Vec<usize>) -> usize {
    let mut multiplicator = 1;
    for i in 0..queue.len() - 1 {
        if !order_queue.get(&queue[i]).unwrap().contains(&queue[i+1]) {
            multiplicator = 0;
            break;
        }
    }
    queue[queue.len()/2] * multiplicator
}

fn read_input(file_path: &Path) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("File not exist");
    return contents.lines().map(|s| s.to_string()).collect::<Vec<_>>();
}

fn main() {
    let file_path = Path::new("day05/input.txt");
    println!("In file {}", file_path.display());
    let map = read_input(file_path);

    let mut order_queue: Vec<String> = Vec::new();
    let mut queues: Vec<Vec<usize>> = Vec::new();
    let mut no_switch = true;

    let mut score_queue = 0;

    for  line in map {
        if line.len() == 0 {
            no_switch = false;
        } else {
            if no_switch {
                order_queue.push(line);
            } else {
                let v: Vec<usize> = line.split(',').map(|s| s.parse::<usize>().unwrap() ).collect();
                queues.push(v);
            }
        }   
    }

    let order_queue = generate_order(&order_queue);

    println!("{:?}", order_queue);

    println!("--------------------");
    for queue in queues {
        println!("{:?}", queue);
        score_queue += midle_point(&order_queue, &queue);
    }
    println!("{:?}", score_queue);
}