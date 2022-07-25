
use std::vec;  
use crate::body::data::Data;

use wasm_bindgen::prelude::*;
use crate::macros::min;

#[wasm_bindgen]
pub struct Heatshrink {
    distcs_to_border: Option<Vec<i32>>,
    pnt_cnt: u16,
}

impl Heatshrink {
    pub fn new(data: &Data) -> Heatshrink {
        let pnt_cnt = data.pnt_cnt();
        Heatshrink {
            distcs_to_border: None,
            pnt_cnt
        }
    }
    
    pub fn fill_distcs_to_border(
        &mut self, 
        points: &Vec<[i32; 2]>, 
        width: i32, height: i32
    ) {
        let mut distcs_to_border = Vec::new();
        for pnt in points.iter() {
            let distc = min![
                pnt[0], 
                width - pnt[0], 
                pnt[1], 
                height - pnt[1]
            ];
            distcs_to_border.push(distc);
        }

        self.distcs_to_border = Some(distcs_to_border);
    }

}