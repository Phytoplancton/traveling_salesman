
use crate::macros::log;
use crate::body::data::Data;
use wasm_bindgen::prelude::*;
use rev_slice::SliceExt;

#[wasm_bindgen]
pub struct TwoOpt {
    cnctn_idx_1: usize,
    cnctn_idx_2: usize,
    pnt_cnt: u16,
    done_steps: i32,
    max_steps: i32,
}

///self manipulation
impl TwoOpt {
    pub fn new(data: &Data) -> TwoOpt {
        let pnt_cnt = data.pnt_cnt();
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

///data manipulation
#[wasm_bindgen]
impl TwoOpt {
    pub fn mut_step(&mut self, data: &mut Data) -> bool {
        while !self.step(data) {
            if !self.next() {
                return false;
            }
        }
        self.next()
    }

    pub fn complete(&mut self, data: &mut Data) {
        while self.step(data) {};
    }
}
impl TwoOpt {
    /// return if mutated connections
    fn step(&mut self, data: &mut Data) -> bool {
        let a = self.cnctn_idx_1;
        let b = a + 1;
        let x = self.cnctn_idx_2;
        let y = data.index(x + 1);

        let current_length = 
            data.distcs()[a] + data.distcs()[x];
        let new_length = 
            data.distc_idx(a, x) 
            + data.distc_idx(b, y);
        if new_length < current_length {
            let mut new_cnctns = Vec::new();
            new_cnctns.extend(&data.connections()[0..=a]);
            new_cnctns.extend(data.connections()[b..=x].rev());
            new_cnctns.extend(&data.connections()[y..]);

            data.set_connections(new_cnctns);

            self.reset();
            
            return true;
        }

        false
    }
}

