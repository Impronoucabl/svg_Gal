use std::f64::consts::PI;

use crate::gall_loc::Location;
use crate::gall_node::GallNode;
use crate::gall_ord::PolarOrdinate;
use crate::gall_pair::GallLinePair;

fn align_nodes(node1:&mut GallNode, node2: &mut GallNode) {
    let ang1 = node1.cent_ang2cent_ang(node2);
    node1.mut_ang(ang1);
    node2.mut_ang(ang1-PI);
}

fn unique_pair_test(node1:&mut GallNode, node2: &mut GallNode, pair_vec: &mut Vec<GallLinePair>) -> bool {
    let center_ref1 = node1.get_center();
    let center_ref2 = node2.get_center();
    for pair in pair_vec {
        let cent1 = pair.node1.get_center();
        let cent2 = pair.node2.get_center();
        if center_ref1 == cent1 && center_ref2 == cent2 ||
        center_ref2 == cent1 && center_ref1 == cent2 {
            return false
        }
    }
    true
}

fn base_loop<'a>(spare_list:Vec<&'a mut GallNode>, pair_list:&mut Vec<GallLinePair<'a>>) -> Vec<&'a mut GallNode> {
    let mut new_spare = Vec::new();
    let mut iter_loop = spare_list.into_iter();
    let node0 = iter_loop.next().expect("empty node list");
    new_spare.push(node0);
    while let Some(node2) = iter_loop.next() {
        let node1 = new_spare.pop().expect("Spare node empty?!?");
        if node1.node_test(node2) && node2.node_test(node1) && 
        unique_pair_test(node1, node2, pair_list) {
            align_nodes(node1, node2);
            pair_list.push(GallLinePair::new(node1,node2));
            if let Some(buffer) = iter_loop.next() {
                new_spare.push(buffer);
            } else {break;}
        } else {
            new_spare.push(node1);
            new_spare.push(node2);
        }
    }
    new_spare
}

pub fn generate_pairs(node_vec:Vec<&mut GallNode>) -> Vec<GallLinePair> {
    let mut pair_list = Vec::new();
    let mut spare_list = Vec::new();
    spare_list = base_loop(node_vec, &mut pair_list);
    if spare_list.len() > 0 {
        spare_list = base_loop(spare_list, &mut pair_list);
    }
    pair_list
}