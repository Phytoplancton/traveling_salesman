
use std::vec;

use wasm_bindgen::prelude::*;

pub struct Heatshrink {
    distcs_to_border: Option<Vec<i32>>,
    pnt_cnt: u16,
}

impl Heatshrink {
    pub fn new(pnt_cnt: u16) -> Heatshrink {
        Heatshrink {
            distcs_to_border: None,
            pnt_cnt
        }
    }
    
    pub fn fill_distcs_to_border(&mut self, 
    points: &Vec<[i32; 2]>, 
    width: i32, height: i32) {
        let mut distcs_to_border = Vec::new();
        for pnt in points.iter() {
            let distc = *vec![
                pnt[0], width - pnt[0], 
                pnt[1], height - pnt[1]
            ].iter().min().unwrap();
            distcs_to_border.push(distc);
        }

        self.distcs_to_border = Some(distcs_to_border);
    }

}