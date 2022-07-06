mod two_opt;
mod cast_to_num;
mod next_neightbour;

use cast_to_num::CastToNum;
use two_opt::TwoOpt;
use next_neightbour::NextNeightbour;

use wasm_bindgen::prelude::*;
use rand::{self, Rng};
use crate::settings;
use crate::macros::log;


#[wasm_bindgen]
pub struct Data {
    pub width: i32,
    pub height: i32,
    pub point_cnt: u16,
    total_distc: Option<i32>,
    points: Vec<[i32; 2]>,
    connections: Vec<u16>,
    sq_distcs: Option<Vec<i32>>,
    distcs: Option<Vec<i32>>,
    two_opt: TwoOpt,
    next_neightbour: NextNeightbour,
}

#[wasm_bindgen]
impl Data {
    pub fn new(width: i32, height: i32, point_cnt: u16) -> Data {
        log!("new data");
        if settings::MARGIN * 2 > width 
            || settings::MARGIN * 2 > height {
            log!("window too small!");
            panic!("window too small!");
        }

        let two_opt = 
        TwoOpt::new(point_cnt);
        let next_neightbour = 
        NextNeightbour::new(point_cnt);
        let mut data = Data {
            width,
            height, 
            point_cnt,
            total_distc: None,
            points: Vec::new(),
            connections: Vec::new(),
            sq_distcs: None,
            distcs: None,
            two_opt,
            next_neightbour,
        }; 
        data.fill_points_rand();
        log!("data created!");
        data
    }

    pub fn get_points(&self) -> *const [i32; 2] {
        self.points.as_ptr()
    }

    pub fn get_connections(&self) -> *const u16 {
        self.connections.as_ptr()
    }
    
    pub fn get_total_distc(&mut self) -> i32 {
        match self.total_distc {
            Some(d) => d,
            None => self.fill_total_distc().total_distc.unwrap()
        }
    }
}

impl Data {
    pub fn update_distcs(&mut self) -> &Data {
        self.fill_sq_distcs();
        self.fill_distcs();
        self.fill_total_distc();

        self
    }

    pub fn get_distcs(&mut self) -> &Vec<i32> {
        if let None = self.distcs {
            self.fill_distcs();            
        }
        &self.distcs.as_ref().unwrap()
    }

    pub fn distc_pnts(&self, pnts: ([i32; 2], [i32; 2])) -> i32 {
        f64::sqrt(self.distc_sq(pnts) as f64) as i32
    }
    
    /// takes two connection indices as args
    pub fn distc_idx<T: CastToNum>
    (&mut self, idx_1: T, idx_2: T) -> i32 {
        self.distc_pnts(self.two_points(idx_1, idx_2))
    }
    
    pub fn index<T: CastToNum>(&self, a: T) -> usize {
        self.fast_mod(a.to_i32(), self.point_cnt as i32)
    }

    /// takes connection indices as arguments
    pub fn two_points<T: CastToNum>(&self, c_1: T, c_2: T) -> 
    ([i32; 2], [i32; 2]) {
        (self.point(c_1), self.point(c_2))
    }

    fn get_sq_distc(&mut self) -> &Vec<i32> {
        if let None = self.sq_distcs {
            self.fill_sq_distcs();
        }
        &self.sq_distcs.as_ref().unwrap()
    }
    
    fn fill_distcs(&mut self) -> &Data {
        let mut distcs = Vec::new();
        for i in self.get_sq_distc().iter() {
            distcs.push(f64::sqrt(*i as f64) as i32);
        }
        self.distcs = Some(distcs);

        self
    }

    fn fill_total_distc(&mut self) -> &Data {
        let mut sum = 0;
        for i in self.get_distcs() {
            sum += i;
        }
        self.total_distc = Some(sum as i32);
        
        self
    }
    
    fn fill_points_rand(&mut self) -> &Data {
        let mut rng = rand::thread_rng();
        self.points = [].to_vec();
        self.connections = [].to_vec();
        for i in 0..self.point_cnt {
            self.points.push(
                [
                    rng.gen_range(
                        settings::MARGIN..self.width - settings::MARGIN),
                    rng.gen_range(
                        settings::MARGIN..self.height - settings::MARGIN)
                ]
            );
            self.connections.push(i);
        }
        
        self
    }

    fn fill_sq_distcs(&mut self) -> &Data{
        let mut sq_distcs = Vec::new();
        for i in 0..self.point_cnt as usize{
            let j = self.index(i + 1);
            sq_distcs.push(
                self.distc_sq(
                    self.two_points(i, j),
                )
            );
        }
        self.sq_distcs = Some(sq_distcs);

        self
    }
    
    fn distc_sq(&self, (pnt_1, pnt_2): ([i32; 2], [i32; 2])) -> i32 {
        let delt_x = pnt_1[0] - pnt_2[0];
        let delt_y = pnt_1[1] - pnt_2[1];
        i32::pow(delt_x, 2) + i32::pow(delt_y, 2)
    }

    fn fast_mod(&self, a: i32, max_index: i32) -> usize {
        match a {
            a if a >= max_index => 
                self.fast_mod( a - max_index as i32, max_index),
            a if a < 0 => 
                self.fast_mod(a + max_index as i32, max_index),
            a => a as usize
        }
    }

    fn point<T: CastToNum>(&self, cnctn_idx: T) -> [i32; 2] {
        let pnt_idx 
            = self.connections[self.index(cnctn_idx)] as usize;
        self.points[pnt_idx]
    }

}
