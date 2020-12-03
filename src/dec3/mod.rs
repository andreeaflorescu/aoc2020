use std::io::{self, BufRead, Read};

struct Slope {
    right: usize,
    down: usize,
}

#[derive(Clone)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Map {
    // u8 so it easy to count; 1 = tree, 0 = no tree.
    map: Vec<Vec<u8>>,
    current_pos: Location,
}

impl Map {
    fn new<F: Read>(reader: F, initial_location: Location) -> Map {
        let reader = io::BufReader::new(reader);
        let mut map = Vec::<Vec<u8>>::new();
        for line in reader.lines() {
            let result = line
                .unwrap()
                .as_bytes()
                .to_vec()
                .into_iter()
                .map(|val| if val as char == '.' { 0 } else { 1 })
                .collect();
            map.push(result);
        }
        Map {
            map,
            current_pos: initial_location,
        }
    }

    fn width(&self) -> usize {
        self.map.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn move_right(&mut self, step: usize) {
        self.current_pos.x = (self.current_pos.x + step) % self.width();
    }

    fn move_down(&mut self, step: usize) -> bool {
        let next = self.current_pos.y + step;
        if next < self.height() {
            self.current_pos.y = next;
            true
        } else {
            false
        }
    }

    fn current_elem(&self) -> u8 {
        self.map[self.current_pos.y][self.current_pos.x]
    }
}

struct Toboggan {
    map: Map,
    slope: Slope,
}

impl Toboggan {
    fn new(map: Map, slope: Slope) -> Self {
        Toboggan { map, slope }
    }

    fn next(&mut self) -> bool {
        self.map.move_right(self.slope.right);
        self.map.move_down(self.slope.down)
    }

    fn slide(&mut self) -> usize {
        let mut nr_trees: usize = 0;

        while self.next() {
            nr_trees += self.map.current_elem() as usize;
        }

        nr_trees
    }
}

pub fn solve_puzzle<F: Read>(reader: F) {
    let initial_location = Location { x: 0, y: 0 };
    let map = Map::new(reader, initial_location);
    let mut slopes: Vec<Slope> = Vec::new();
    slopes.push(Slope { right: 1, down: 1 });
    slopes.push(Slope { right: 3, down: 1 });
    slopes.push(Slope { right: 5, down: 1 });
    slopes.push(Slope { right: 7, down: 1 });
    slopes.push(Slope { right: 1, down: 2 });

    let mut nr_trees: Vec<usize> = Vec::new();
    for slope in slopes.drain(..) {
        let mut toboggan = Toboggan::new(map.clone(), slope);
        nr_trees.push(toboggan.slide());
    }

    println!("nr trees for slope nr 2={}", nr_trees[1]);
    println!(
        "product of nr trees on all slopes={}",
        nr_trees.iter().fold(1, |acc, v| acc * v)
    );
}
