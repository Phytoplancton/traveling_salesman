
use crate::data::Data;
use crate::macros::log;
use wasm_bindgen::prelude::*;


pub struct NextNeightbour {
    cnctn_idx: usize,
    pnt_cnt: u16,
    done_steps: i32,
    max_steps: i32,
}

impl NextNeightbour {
    pub fn new(pnt_cnt: u16) -> NextNeightbour{
        NextNeightbour {
            cnctn_idx: 0,
            pnt_cnt,
            done_steps: 0,
            max_steps: (pnt_cnt) as i32, 
        }
    }
    
    pub fn next(&mut self) -> bool {
        self.cnctn_idx += 1;
        if self.cnctn_idx >= self.pnt_cnt.into() {
            self.cnctn_idx = 0;
        }
        self.done_steps += 1;
        if self.done_steps >= self.max_steps { 
            log!("next neighbor finished");
            return false;
        }
        true
    }

    pub fn reset(&mut self) -> &NextNeightbour {
        self.done_steps = 0;
        
        self
    }
}

#[wasm_bindgen]
impl Data {
    pub fn nxt_ngbr_mut_step(&mut self) -> bool {
        while !self.nxt_ngbr(self.next_neightbour.cnctn_idx) {
            if !self.next_neightbour.next() {
                return false;
            }
        }
        self.update_distcs();
        self.next_neightbour.next()
    }

    pub fn complete_nxt_ngbr(&mut self) {
        while self.nxt_ngbr_mut_step() {}
    }
}

impl Data {

    /// return if muted connections
    fn nxt_ngbr(&mut self, cnctn_idx: usize) -> bool {

        let mut len_diffs = Vec::new();

        for i in 0..self.point_cnt as usize{
            if i == cnctn_idx || self.index(i + 1) == cnctn_idx {
            len_diffs.push(0);
                continue;
            }
            let cnctn_idx_2 = self.index(cnctn_idx - 1);
            let crnt_len             
                = self.get_distcs()[cnctn_idx] 
                + self.get_distcs()[cnctn_idx_2]
                + self.get_distcs()[i];
            let new_len = 
                self.distc_idx(
                        self.index(cnctn_idx as i32 - 1), 
                        self.index(cnctn_idx + 1)
                    )
                + self.distc_idx(cnctn_idx, i)
                
                + self.distc_idx(cnctn_idx, self.index(i + 1));
            len_diffs.push(new_len - crnt_len);

        }

        let min_len_diff = len_diffs.iter().min().unwrap();
        if min_len_diff < &0 {
            let min_idx = len_diffs
                .iter()
                .position(|p| p == min_len_diff)
                .unwrap();
            let elem = self.connections.remove(cnctn_idx);
            let new_index = if min_idx < cnctn_idx
                {min_idx + 1} else {min_idx};
            self.connections.insert(new_index, elem);

            self.next_neightbour.reset();
            return true;
        }
        false

    }

}