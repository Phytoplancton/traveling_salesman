mod cast_to_num;

use cast_to_num::CastToNum;
use wasm_bindgen::prelude::*;
use rand::{self, Rng};
use crate::settings;
use std::cmp;

#[wasm_bindgen]
pub struct Data {
    width: i32,
    height: i32,
    pnt_cnt: u16,
    total_distc: Option<i32>,
    points: Vec<[i32; 2]>,
    connections: Vec<u16>,
    sq_distcs: Option<Vec<i32>>,
    distcs: Option<Vec<i32>>,
}

#[wasm_bindgen]
impl Data {
    pub fn points_ptr(&self) -> *const [i32; 2] {
        self.points.as_ptr()
    }

    pub fn connections_ptr(&self) -> *const u16 {
        self.connections.as_ptr()
    }
    
    pub fn total_distc(&mut self) -> i32 {
        match self.total_distc {
            Some(d) => d,
            None => self
                .fill_total_distc()
                .total_distc
                .unwrap()
        }
    }
}

//getters 
impl Data {
    pub fn distcs(&mut self) -> &Vec<i32> {
        if let None = self.distcs {
            self.fill_distcs();            
        }
        &self.distcs.as_ref().unwrap()
    }

    fn sq_distc(&mut self) -> &Vec<i32> {
        if let None = self.sq_distcs {
            self.fill_sq_distcs();
        }
        &self.sq_distcs.as_ref().unwrap()
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn pnt_cnt(&self) -> u16 {
        self.pnt_cnt
    }

    pub fn connections(&self) -> &Vec<u16> {
        &self.connections
    }
}

///setters
impl Data {
    pub fn set_connections(&mut self, cnctns: Vec<u16>) ->
    &Data {
        self.connections = cnctns;
        self.reset_distcs();

        self
    }

    pub fn swap_connections(
    &mut self, idx1: usize, idx2: usize) -> &Data {
        let elem = self.connections.remove(idx1);
        let new_index = if idx2 < idx1
            {idx2 + 1} else {idx2};
        self.connections.insert(new_index, elem);
        self.reset_distcs();

        self
    }
}

//self manipulation
impl Data {
    fn fill_distcs(&mut self) -> &Data{
        let mut distcs = Vec::new();
        for i in self.sq_distc().iter() {
            distcs.push(f64::sqrt(*i as f64) as i32);
        }
        self.distcs = Some(distcs);

        self
    }

    fn fill_total_distc(&mut self) -> &Data{
        let mut sum = 0;
        for i in self.distcs() {
            sum += i;
        }
        self.total_distc = Some(sum as i32);
        
        self
    }
    
    fn fill_points_rand(&mut self) -> &Data{
        let mut rng = rand::thread_rng();
        self.points = [].to_vec();
        self.connections = [].to_vec();
        let min_size = 
            cmp::min(self.width, self.height);
        let margin = 
            (min_size as f32 * settings::MARGIN_FCTR) as i32;

        for i in 0..self.pnt_cnt {
            self.points.push(
                [
                    rng.gen_range(
                        margin..self.width - margin),
                    rng.gen_range(
                        margin..self.height - margin)
                ]
            );
            self.connections.push(i);
        }
        
        self
    }

    fn fill_sq_distcs(&mut self) -> &Data{
        let mut sq_distcs = Vec::new();
        for i in 0..self.pnt_cnt as usize{
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

    fn reset_distcs(&mut self) -> &Data{
        self.distcs = None;
        self.sq_distcs = None;
        self.total_distc = None;

        self
    }
}

//tools
impl Data {
    pub fn new(width: i32, height: i32, pnt_cnt: u16) -> Data {
        let mut data = Data {
            width,
            height, 
            pnt_cnt,
            total_distc: None,
            points: Vec::new(),
            connections: Vec::new(),
            sq_distcs: None,
            distcs: None,
        }; 
        data.fill_points_rand();
        data
    }

    pub fn distc_pnts(&self, pnts: ([i32; 2], [i32; 2])) -> i32 {
        f64::sqrt(self.distc_sq(pnts) as f64) as i32
    }
    
    /// takes two connection indices as args
    pub fn distc_idx<T: CastToNum>
    (&mut self, idx_1: T, idx_2: T) -> i32 {
        self.distc_pnts(self.two_points(idx_1, idx_2))
    }
    
    /// index mod number of points
    pub fn index<T: CastToNum>(&self, a: T) -> usize {
        self.fast_mod(a.to_i32(), self.pnt_cnt as i32)
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

    /// takes connection index as parameter
    fn point<T: CastToNum>(&self, cnctn_idx: T) -> [i32; 2] {
        let pnt_idx 
            = self.connections[self.index(cnctn_idx)] as usize;
        self.points[pnt_idx]
    }

    /// takes connection indices as arguments
    pub fn two_points<T: CastToNum>(&self, c_1: T, c_2: T) -> 
    ([i32; 2], [i32; 2]) {
        (self.point(c_1), self.point(c_2))
    }

    
}
