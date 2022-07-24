
use crate::macros::log;
use crate::body::data::Data;
use wasm_bindgen::prelude::*;
use rev_slice::SliceExt;


pub struct TwoOpt {
    cnctn_idx_1: usize,
    cnctn_idx_2: usize,
    pnt_cnt: u16,
    done_steps: i32,
    max_steps: i32,
}

impl TwoOpt {
    pub fn new(data: &Data) -> TwoOpt {
        let pnt_cnt = data.pnt_cnt;
        let max_steps 
            = (pnt_cnt as i32) * (pnt_cnt as i32 + 1) / 2;
        TwoOpt { 
            cnctn_idx_1: 0,
            cnctn_idx_2: 2,
            pnt_cnt,
            done_steps: 0, 
            max_steps
        }
    }

    pub fn next(&mut self) -> bool{
        self.cnctn_idx_1 += 1;
        if self.cnctn_idx_1 + 1 >= self.cnctn_idx_2 {
            self.cnctn_idx_1 = 0;
            self.cnctn_idx_2 += 1;
            if self.cnctn_idx_2 >= self.pnt_cnt as usize{
                self.cnctn_idx_2 = 2;
            }
        }
        self.done_steps += 1;
        if self.done_steps + 1 >= self.max_steps {
            log!("finished two_opt");
            return false;
        }
        true
    }  

    pub fn reset(&mut self) -> &TwoOpt {
        self.done_steps = 0;
        
        self
    }
}



#[wasm_bindgen]
impl Data {
    pub fn two_opt_mut_step(&mut self) -> bool {
        while !self.two_opt_step() {
            if !self.two_opt.next() {
                return false;
            }
        }
        self.two_opt.next()
    }

    pub fn complete_two_opt(&mut self) {
        while self.two_opt_step() {};
    }
}

impl Data {
    /// return if mutated connections
    fn two_opt_step(&mut self) -> bool {
        let a = self.two_opt.cnctn_idx_1;
        let b = a + 1;
        let x = self.two_opt.cnctn_idx_2;
        let y = self.index(x + 1);

        let current_length = 
            self.get_distcs()[a] + self.get_distcs()[x];
        let new_length = 
            self.distc_idx(a, x) 
            + self.distc_idx(b, y);
        if new_length < current_length {
            let mut new_cnctns = Vec::new();
            new_cnctns.extend(&self.connections[0..=a]);
            new_cnctns.extend(self.connections[b..=x].rev());
            new_cnctns.extend(&self.connections[y..]);

            self.connections = new_cnctns;

            self.reset_distcs();
            self.two_opt.reset();
            
            return true;
        }

        false
    }
}

