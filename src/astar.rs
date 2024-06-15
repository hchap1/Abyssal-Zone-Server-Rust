#[derive(Clone)]
pub struct Position {
    x: usize,
    y: usize
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
    pub fn pretty_print(&self) -> String {
        format!("[{},{}]", self.x, self.y)
    }
    fn get_adjacent(&self, max: usize) -> Vec<Position> {
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
        adjacent
    }
}

#[derive(Clone)]
struct Node {
    parent: Option<usize>,
    position: Position,
    g: usize,
    h: usize,
    f: usize
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Node {
    fn set_g_and_h(&mut self, g: usize, h: usize) {
        self.g = g;
        self.h = h;
        self.f = g + h;
    }
}

fn walkable(tilemap: &Vec<Vec<usize>>, old_position: &Position, new_position: &Position) -> bool {
    let tile_id: usize = tilemap[new_position.y][new_position.x];
    let _dy: i32 = new_position.y as i32 - old_position.y as i32;
    vec![2, 5, 3, 7].contains(&tile_id)
}

pub fn astar(tilemap: &Vec<Vec<usize>>, start: Position, end: Position) -> Option<Vec<Position>> {
    let start_node = Node { parent: None, position: start, g: 0, h: 0, f: 0 };
    let end_node = Node { parent: None, position: end, g: 0, h: 0, f: 0 };
    let mut open_list: Vec<Node> = vec![start_node];
    let mut closed_list: Vec<Node> = vec![];
    let mut running: bool = true;
    while running {
        // Get the current node
        let mut current_node: Node = open_list[0].clone();
        println!("CURRENT_NODE: {}", current_node.position.pretty_print());
        let mut current_index = 0;
        // Find the best node from here
        for (index, item) in open_list.iter().enumerate() {
            if item.f < current_node.f {
                current_node = item.clone();
                current_index = index;
            }
        }

        // Now that we have the best node, move it to the closed list.
        open_list.remove(current_index);
        closed_list.push(current_node);

        let current_node: usize = closed_list.len() - 1;

        // If we have found the goal.
        if closed_list[current_node].position == end_node.position {
            let mut path: Vec<Position> = vec![];
            let mut current_path_node: Option<&Node> = Some(&closed_list[current_node]);
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

        // If we haven't finished, find the children of the current node (based on adjacents).
        let mut children: Vec<Node> = vec![];
        for adjacent in closed_list.last().unwrap().position.get_adjacent(tilemap.len()) {
            if walkable(tilemap, &closed_list[current_node].position, &adjacent) {
                let new_node: Node = Node { parent: Some(current_node), position: adjacent, g: 0, h: 0, f: 0 };
                children.push(new_node);
            }
        }

        // Loop through children
        for mut child in children {
            // If child is NOT closed.
            if !closed_list.contains(&child) {
                let h = ((child.position.x as i32 - end_node.position.x as i32).pow(2) + (child.position.y as i32 - end_node.position.y as i32).pow(2)) as usize;
                child.set_g_and_h(closed_list.last().unwrap().g + 1, h);
                // If there is already a 'better' child on the open list
                let mut found_better: bool = false;
                for open_node in open_list.iter() {
                    if child == *open_node && child.g > open_node.g {
                        found_better = true;
                    }
                }
                if !found_better {
                    open_list.push(child);
                }
            }
        }
        if open_list.len() <= 0 {
            running = false;
        }
    }
    None
}