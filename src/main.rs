use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

type Unit = u64;
type Point = (Unit, Unit);
type TreeSet = HashSet<Point>;

fn main() {
    let forest = read_forest();
    let a1 = tree_count(&forest, 3, 1);
    let a2 = task2(&forest);
    println!("Answer 1 = {}", a1);
    println!("Answer 2 = {}", a2);
}

struct Forest {
    width: Unit,
    height: Unit,
    trees: TreeSet,
}

impl Forest {
    fn new(width: Unit, height: Unit, trees: TreeSet) -> Forest {
        println!("New forest: {} {}", width, height);
        Forest { width, height, trees }
    }
    fn is_tree(&self, point: Point) -> bool {
        let p = (point.0 % self.width, point.1);
        let t = self.trees.contains(&p);
        if t {
            println!("Tree at ({},{}) -> ({},{}) : {} width:{}", point.0, point.1, p.0, p.1, t, self.width);
        }
        t
    }
}

fn task2(forest: &Forest) -> u64 {
    let t11 = tree_count(forest, 1, 1);
    let t13 = tree_count(forest, 3, 1);
    let t15 = tree_count(forest, 5, 1);
    let t17 = tree_count(forest, 7, 1);
    let t21 = tree_count(forest, 1, 2);
    t11 * t13 * t15 * t17 * t21
}

fn tree_count(forest: &Forest, dx: Unit, dy: Unit) -> u64 {
    let mut count: u64 = 0;
    for i in 1..forest.height {
        if forest.is_tree((i * dx, i * dy)) {
            count = count + 1;
        }
    }
    count
}

fn read_forest() -> Forest {
    let mut trees: TreeSet = TreeSet::new();
    let mut width: Unit = 0;
    let mut height: Unit = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for lineresult in lines {
            if let Ok(line) = lineresult {
                if width < 1 {
                    width = line.len() as u64;
                }
                for i in 0..line.len() {
                    let chars: Vec<char> = line.chars().collect();
                    if chars[i] == '#' {
                        let tree = (i as u64, height);
                        println!("Adding tree: ({},{})", tree.0, tree.1);
                        trees.insert(tree);
                    }
                }
                height = height + 1;
            }
        }
    }
    Forest::new(width, height, trees)
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}