mod two_opt;
mod next_neightbour;
mod heatshrink;
mod data;

use two_opt::TwoOpt;
use next_neightbour::NextNeightbour;
use heatshrink::Heatshrink;
use data::Data;

pub struct Body {
    data: Data,
    two_opt: TwoOpt,
    next_neightbour: NextNeightbour,
    heatshrink: Heatshrink,
}


#[wasm_bindgen]
impl Body {
    pub fn new(width: i32, height: i32, point_cnt: u16) 
    -> Body {
        
        let data = 
        Data::new(width, height, point_cnt);
        let two_opt = 
        TwoOpt::new(&data);
        let next_neightbour = 
        NextNeightbour::new(&data);
        let heatshrink = 
        Heatshrink::new(&data);       

        Body {
            data,
            two_opt,
            next_neightbour,
            heatshrink,
        }        
    }
}
