//Key realization here is to start working from the bottom.
//Since any two neighboring sibling nodes share the same parent one having greater value from the
//bottom is strictly better than the other. We can assess each node by comparing it's children's
//values and taking the larger one
//You could do it in many less lines of code by accessing/mutating the initial vector directly
//instead of constructing nodes and a VecMap but I ended up learning more Rust this way

use std::collections::VecMap;
use std::iter::range;
use std::cmp::max;

struct Node {
    depth: usize,
    position: usize,
    val: i32,
    children_max_sum: i32,
}

impl Node {
    // TODO:: figure a way to DRY this up.. macros?
    fn get_child_coords(&self, left: bool) -> (usize, usize) {
        let depth = self.depth+1;
        let position = if left {self.position} else {self.position + 1};
        (depth, position)
    }

    fn total(&self) -> i32 {
        self.children_max_sum + self.val
    }
}

// Bijection from N x N -> N so that I can map my 2d coordinated onto my VecMap which contains the
// nodes
fn cantor_pairing(coords: (usize, usize)) -> usize {
  let (x, y) = coords;
  (((x + y) * (x+y+1)) / 2 + y) as usize
}

fn create_node(depth:usize, position:usize, val: i32) -> Node {
    Node {
        depth:depth,
        position:position,
        val:val,
        children_max_sum: 0
    }
}

fn get_child_from_coords(coords: (usize, usize), path_map: &VecMap<Node>) -> &Node {
    path_map.get(&cantor_pairing(coords)).unwrap()
}

fn main() {
let input: Vec<Vec<i32>> = "75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23".split('\n').map(|line| line.split(' '). map(|num| num.parse::<i32>().unwrap()).collect()).collect();

    let mut current_depth: usize = input.len() - 1;
    let max_depth: usize = input.len()  - 1;
    let mut path_map = VecMap::new();
    loop {
        for i in range(0, current_depth + 1) {
            let mut new_node = create_node(current_depth, i, input[current_depth][i]);
            let children_max_sum = if current_depth < max_depth {
                let left_child = get_child_from_coords(new_node.get_child_coords(true), &path_map);
                let right_child = get_child_from_coords(new_node.get_child_coords(false), &path_map);
                max(left_child.total(), right_child.total())
            } else {
                0
            };
            new_node.children_max_sum = children_max_sum;
            if current_depth == 0 {
                println!("Max path sum is {}", new_node.total());
                return;
            }
            path_map.insert(cantor_pairing((current_depth, i)), new_node);
        }
        current_depth -= 1;
    }
}
