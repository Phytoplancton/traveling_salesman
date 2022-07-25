
use crate::body::data::Data;
use crate::macros::log;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct NextNeightbour {
    /// "connnection index"
    c_idx: usize,
    pnt_cnt: u16,
    done_steps: i32,
    max_steps: i32,
}

///self manipulation 
impl NextNeightbour {
    pub fn new(data: &Data) -> NextNeightbour{
        let pnt_cnt = data.pnt_cnt();
        NextNeightbour {
            c_idx: 0,
            pnt_cnt,
            done_steps: 0,
            max_steps: (pnt_cnt) as i32, 
        }
    }
    
    pub fn next(&mut self) -> bool {
        self.c_idx += 1;
        if self.c_idx >= self.pnt_cnt.into() {
            self.c_idx = 0;
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

///data manipulation
#[wasm_bindgen]
impl NextNeightbour {
    pub fn mut_step (&mut self, data: &mut Data) -> bool {
        while !self.step(data) {
            if !self.next() {
                return false;
            }
        }
        self.next()
    }

    pub fn complete(&mut self, data: &mut Data)  {
        while self.mut_step(data) {}
    }
}
impl NextNeightbour {

    /// return if muted connections
    fn step(&mut self, data: &mut Data) -> bool {

        let mut len_diffs = Vec::new();

        for i in 0..data.pnt_cnt() as usize {
            if i == self.c_idx 
            || data.index(i + 1) == self.c_idx {
                len_diffs.push(0);
                continue;
            };
            let c_idx_2 = 
                data.index(self.c_idx as i32 - 1);
            let crnt_len             
                = data.distcs()[self.c_idx] 
                + data.distcs()[c_idx_2]
                + data.distcs()[i];
            let new_len = 
                data.distc_idx(
                        self.c_idx as i32 - 1, 
                        self.c_idx as i32 + 1
                    )
                + data.distc_idx(self.c_idx, i)
                
                + data.distc_idx(self.c_idx, data.index(i + 1));
            len_diffs.push(new_len - crnt_len);

        }

        let min_len_diff = 
            len_diffs
                .iter()
                .min()
                .unwrap();
        if min_len_diff < &0 {
            let min_idx = len_diffs
                .iter()
                .position(|p| p == min_len_diff)
                .unwrap();
            data.swap_connections(self.c_idx, min_idx);

            self.reset();
            return true;
        }
        false

    }

}