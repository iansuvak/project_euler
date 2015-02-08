//Key realization here is to start working from the bottom.
//Since any two neighboring sibling nodes share the same parent one having greater value from the
//bottom is strictly better than the other

struct Node {
    depth: usize,
    position: usize,
    val: i32,
    children_max_sum: i32,
}

impl Node {
    fn getParent(&self, left: bool, input: &Vec<Vec<i32>>) -> Option<Node> {
        println!("depth:{}, pos:{}, left:{}", self.depth, self.position, left);
        if self.depth == 0 || (left && self.position == 0) || (!left && self.position == self.depth) {
          None
        } else {
            let depth = self.depth-1;
            let position = if left {self.position - 1} else {self.position};
            Some(Node {
                depth: depth,
                position: position,
                val:input[depth][position],
                children_max_sum: self.children_max_sum + self.val})
        }

    }

    fn max(&self) -> i32 {
        self.children_max_sum + self.val
    }

    //fn is_sibling_of(&self, candidate: &Node) -> bool {
        //if self.depth == candidate.depth && ((self.position as isize - candidate.position as isize).abs() == 1) { true } else { false}
    //}
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            depth:self.depth,
            position:self.position,
            val:self.val,
            children_max_sum:self.children_max_sum,
        }
    }
}

struct LiveNodes {
    list: Vec<Node>
}

// very inefficient since I am reallocating memory on each action, pruning and generation but this
// is just a first pass
impl LiveNodes {
    fn prune(self) -> LiveNodes {
        // Taking advantage of the known ordering here to only do one loop of the nodes
        let mut current_node = self.list[0].clone();

        let mut live_nodes: Vec<Node> = Vec::new();
        for i in self.list {
            let Node {depth:depth, position:position, val:val, ..} = current_node;
            if (i.max() > current_node.max() || i.position== 0 || i.position == i.depth) {
                live_nodes.push(i.clone());
            }
            current_node = i;
        }
        LiveNodes { list: live_nodes}
    }

    fn maybeAddParent(&mut self, node: Option<Node>) {
        match node {
            None => (),
            Some(x) => self.list.push(x),
        }
    }

    fn generateParents(self, data: &Vec<Vec<i32>>) -> LiveNodes {
        let mut live_nodes =  LiveNodes{list : Vec::new()};
        for i in self.list {
            live_nodes.maybeAddParent(i.getParent(true, data));
            live_nodes.maybeAddParent(i.getParent(false, data));
        }
        live_nodes
    }
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

    let max_depth: usize = input.len()  - 1;
    let mut live_nodes = LiveNodes { list: Vec::new()};
    for i in range(0, max_depth) {
        live_nodes.list.push(Node {
            depth:max_depth,
            position: i,
            val: input[max_depth][i],
            children_max_sum: 0,
        })

    }
    while live_nodes.list.len() > 1 {
        live_nodes = live_nodes.prune();
        live_nodes = live_nodes.generateParents(&input);
    }
    let Node {depth: x,position: y,val: z, ..} = live_nodes.list[0];
    println!("({},{}) = {}", x,y,z);
}
