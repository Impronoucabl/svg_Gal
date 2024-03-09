use crate::gall_node::GallNode;
use crate::gall_pair::GallLinePair;

fn setup(node: &GallNode) -> (f64,f64) {
    (0.0,0.0)//todo!()
}

fn base_loop<'a>(spare_list:Vec<&'a GallNode>, pair_list:&mut Vec<GallLinePair<'a>>) -> Vec<&'a GallNode> {
    let mut new_spare = Vec::new();
    let mut iter_loop = spare_list.into_iter();
    let mut node1 = iter_loop.next().expect("empty node list");
    let mut conds = setup(node1);
    let mut clean = false;
    while let Some(node2) = iter_loop.next() {
        if true {
            pair_list.push(GallLinePair::new(node1,node2));
            if let Some(node1) = iter_loop.next() {
                conds = setup(node1)
            } else {
                clean = true;
                break;
            }
        } else {
            new_spare.push(node1);
            node1 = node2;
        }
    }
    if !clean {
        new_spare.push(node1);
    }
    new_spare
}

pub fn generate_pairs(node_vec:Vec<&GallNode>) -> Vec<GallLinePair> {
    let mut pair_list = Vec::new();
    let mut spare_list = Vec::new();
    spare_list = base_loop(node_vec, &mut pair_list);
    pair_list
}