use std::collections::VecDeque;
use crate::bsp::Segment;
use crate::bsp::BSPTree;

pub mod bsp;

fn main() {
    let mut arr : VecDeque<Segment> = VecDeque::new();
    let a = Segment { x1: -50.0, y1: 0.0, x2: 50.0, y2: 0.0 };
    let b = Segment { x1: -50.0, y1: 20.0, x2: -50.0, y2: -20.0 };
    let c = Segment { x1: 40.0, y1: -30.0, x2: 80.0, y2: 30.0 };
    let d = Segment { x1: -80.0, y1: 30.0, x2: -40.0, y2: -40.0 };
    arr.push_back(a);
    arr.push_back(b);
//    arr.push_back(c);
    arr.push_back(d);
    
//    println!("{:?}",arr);
    let tree = BSPTree::new(&mut arr);

    println!("Generated BSP tree:");
    tree.print_leaves(&tree.root);
}
