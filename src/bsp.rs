use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
pub struct Segment {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

impl Segment {
    pub fn new(x1 : f64,y1 : f64,x2 : f64,y2 : f64) -> Segment {
        return Segment {
            x1,
            y1,
            x2,
            y2
        };
    }

    fn print(&self) {
        println!("({}, {}), ({}, {})", self.x1,self.y1,self.x2,self.y2);
    }
}

pub struct BSPNode {
    seg: Segment,
    left: Option<Box<BSPNode>>,
    right: Option<Box<BSPNode>>,
}

impl BSPNode {
    pub fn new(seg: Segment) -> Self {
        return BSPNode {
            seg,
            left: None,
            right: None,
        };
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

pub struct BSPTree {
    pub root: Option<Box<BSPNode>>,
}

fn compute_coeffs(s : Segment) -> (f64,f64) {
    let diff_x : f64 = s.x2 - s.x1;
    let mut coeff : f64 = 0.0;
    let mut b : f64 = 0.0;
    if diff_x != 0.0 {
        coeff = (s.y2 - s.y1) / diff_x;
        b = s.y1 - coeff*s.x1;
    } else {
        coeff = std::f64::INFINITY;
    }
    
    return (coeff,b);
}

impl BSPTree {
    pub fn new(segs: &mut VecDeque<Segment>) -> Self {
        let mut tree = BSPTree {
            root: None,
        };
        tree.root = tree.create(segs);
        return tree;
    }

    fn create(&self, arr: &mut VecDeque<Segment>) -> Option<Box<BSPNode>> {
        let mut queue : VecDeque<Segment> = arr.clone();
        if let Some(s) = queue.pop_front() {
            println!("{:?}",s);
            println!("{:?}",queue);
            let mut left_arr : VecDeque<Segment> = VecDeque::new(); 
            let mut right_arr : VecDeque<Segment> = VecDeque::new();
            
            let (coeff_s, b_s) = compute_coeffs(s);
            
            for seg in queue.iter() {
                let (mut coeff, b) = compute_coeffs(*seg);
                let mut split_x : f64 = 0.0;
                
                if coeff==0.0 {
                    split_x=seg.x1;
                } else if coeff == std::f64::INFINITY {
                    split_x=s.x1;
                    coeff=0.0;
                } else if coeff - coeff_s != 0.0 {
                    split_x = (b_s - b) / (coeff - coeff_s);
                }
                
                let split_y : f64 = coeff * split_x + b;

                println!("split : {:.2} , {:.2}",split_x,split_y);
                println!("coeff : {:.2} b : {:.2}",coeff,b);
                println!("x : {} x1 : {} x2 : {}",split_x,seg.x1,seg.x2);

                if seg.x1 >= split_x {
                    if coeff>=0.0 {
                        if seg.x2 <= split_x {
                            println!("---A");
                            left_arr.push_back(Segment::new(seg.x1,seg.y1,split_x,split_y));
                            right_arr.push_back(Segment::new(split_x,split_y,seg.x2,seg.y2));
                        } else {
                           println!("---B");
                            left_arr.push_back(*seg);
                        }
                    } else {
                        if seg.x2 <= split_x {
                            println!("---C");
                            right_arr.push_back(Segment::new(seg.x1,seg.y1,split_x,split_y));
                            left_arr.push_back(Segment::new(split_x,split_y,seg.x2,seg.y2));
                        } else {
                            println!("---D");
                            right_arr.push_back(*seg);
                        }
                    }
                } else {
                    if coeff>=0.0 {
                        if seg.x2 > split_x {
                            println!("---E");
                            right_arr.push_back(Segment::new(seg.x1,seg.y1,split_x,split_y));
                            left_arr.push_back(Segment::new(split_x,split_y,seg.x2,seg.y2));
                        } else {
                            println!("---F");
                            right_arr.push_back(*seg);
                        }
                    } else {
                        if seg.x2 > split_x {
                            println!("---G");
                            left_arr.push_back(Segment::new(seg.x1,seg.y1,split_x,split_y));
                            right_arr.push_back(Segment::new(split_x,split_y,seg.x2,seg.y2));
                        } else {
                            println!("---H");
                            left_arr.push_back(*seg);
                        }
                    }
                }
            }
            
            let mut node : Box<BSPNode> = Box::new(BSPNode::new(s));
            println!("left : {:?}",left_arr);
            println!("right : {:?}",right_arr);
            node.right = self.create(&mut right_arr);
            node.left = self.create(&mut left_arr);
            
            return Some(node);
        }
        return None;
}

    pub fn print_leaves(&self, node: &Option<Box<BSPNode>>) {
        if let Some(n) = node {
            if n.is_leaf() {
                n.seg.print();
            } else {
                self.print_leaves(&n.left);
                n.seg.print();
                self.print_leaves(&n.right);
            }
        }
    }
}

