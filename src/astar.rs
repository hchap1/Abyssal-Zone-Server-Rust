use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(PartialEq)]
pub enum Ai {
    Spider,
    Ground
}

#[derive(Clone, Eq, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    pub fn pretty_print(&self) -> String {
        format!("[{},{}]", self.x, self.y)
    }

    fn get_adjacent(&self, max: usize, ai:& Ai) -> Vec<Position> {
        let mut adjacent: Vec<Position> = vec![];
        if self.x > 0 {
            adjacent.push(Position { x: self.x - 1, y: self.y });
        }
        if self.y > 0 {
            adjacent.push(Position { x: self.x, y: self.y - 1 });
        }
        if self.x < max - 1 {
            adjacent.push(Position { x: self.x + 1, y: self.y });
        }
        if self.y < max - 1 {
            adjacent.push(Position { x: self.x, y: self.y + 1 });
        }
        if *ai == Ai::Ground {
            if self.x > 0 && self.y > 0{
                adjacent.push(Position { x: self.x - 1, y: self.y - 1 });
            }
            if self.y > 0 && self.x < max - 1 {
                adjacent.push(Position { x: self.x + 1, y: self.y - 1 });
            }
            if self.x < max - 1 && self.y < max - 1 {
                adjacent.push(Position { x: self.x + 1, y: self.y + 1 });
            }
            if self.x > 0 && self.y < max - 1 {
                adjacent.push(Position { x: self.x - 1, y: self.y + 1 });
            }
        }
        adjacent
    }
}

#[derive(Clone, Eq)]
struct Node {
    parent: Option<usize>,
    position: Position,
    g: usize,
    h: usize,
    f: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f) // Inverted for min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn set_g_and_h(&mut self, g: usize, h: usize) {
        self.g = g;
        self.h = h;
        self.f = g + h;
    }
}

fn walkable(tilemap: &Vec<Vec<usize>>, old_position: &Position, new_position: &Position, ai: &Ai) -> bool {
    let dy: isize = new_position.y as isize - old_position.y as isize;
    if *ai == Ai::Spider { 
        return vec![2, 5, 3, 7, 6].contains(&tilemap[new_position.y][new_position.x]); 
    }
    if *ai == Ai::Ground {
        if new_position.y > 1 {
            if vec![2, 3, 5, 7].contains(&tilemap[new_position.y][new_position.x]) && vec![1, 4].contains(&tilemap[new_position.y - 1][new_position.x]) { return true; }
            if dy > 0 && tilemap[new_position.y][new_position.x] == 6 { return true; }
        }
        if dy < 0 {
            return tilemap[new_position.y][new_position.x] == 6; 
        }
    }
    false
}

pub fn astar(tilemap: &Vec<Vec<usize>>, start: Position, end: Position, ai_type: &Ai) -> Option<Vec<Position>> {
    let start_node = Node { parent: None, position: start.clone(), g: 0, h: 0, f: 0 };
    let end_node = Node { parent: None, position: end.clone(), g: 0, h: 0, f: 0 };

    let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
    let mut closed_list: Vec<Node> = vec![];
    let mut running: bool = true;

    open_list.push(start_node);

    while running {
        if open_list.is_empty() {
            running = false;
            break;
        }

        let current_node = open_list.pop().unwrap();
        let current_index = closed_list.len();
        closed_list.push(current_node.clone());

        // If we have found the goal.
        if current_node.position == end_node.position {
            let mut path: Vec<Position> = vec![];
            let mut current_path_node: Option<&Node> = Some(&closed_list[current_index]);
            while let Some(current) = current_path_node {
                path.push(current.position.clone());
                match current.parent {
                    Some(index) => {
                        current_path_node = Some(&closed_list[index]);
                    }
                    None => {
                        current_path_node = None;
                    }
                }
            }
            path.reverse();
            return Some(path);
        }

        // Find the children of the current node (based on adjacents).
        let mut children: Vec<Node> = vec![];
        for adjacent in current_node.position.get_adjacent(tilemap.len(), ai_type) {
            if walkable(tilemap, &current_node.position, &adjacent, ai_type) {
                let mut new_node = Node {
                    parent: Some(current_index),
                    position: adjacent.clone(),
                    g: 0,
                    h: 0,
                    f: 0,
                };
                new_node.set_g_and_h(current_node.g + 1, ((adjacent.x as isize - end_node.position.x as isize).pow(2) + (adjacent.y as isize - end_node.position.y as isize).pow(2)) as usize);
                children.push(new_node);
            }
        }

        // Loop through children
        for child in children {
            // If child is not in closed list
            if !closed_list.contains(&child) {
                // If child is already in open list with a higher g cost, skip adding it
                if let Some(open_node) = open_list.iter().find(|&n| n == &child && n.g < child.g) {
                    continue;
                }
                open_list.push(child);
            }
        }
    }
    None
}
