use crate::body::Body;

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

// #[test]
// fn test_print() {
//     let mut data = Body::data::new(500, 500, 5);
//     data.two_opt_mut_step();
//     println!("height: {}", data.height);
//     println!("width: {}", data.width);
//     println!("point_cnt: {}", data.point_cnt); 
//     println!("length: {}", data.get_total_distc()); 
// }

// #[test]
// fn test_getters() {
//     let data = data::Data::new(500, 500, 5);
//     data.get_connections();
//     data.get_points();
// }

// #[test] 
// fn two_opt() {
//     let mut data = data::Data::new(500, 500, 5);
//     while data.two_opt_mut_step() {
//         println!("length: {}", data.get_total_distc());
//     }
// }

// #[test]
// fn test_next_ngbr() {
//     let mut data = data::Data::new(500, 500, 15);
//     while data.two_opt_mut_step() {};
//     while data.nxt_ngbr_mut_step() {
//         println!("length: {}", data.get_total_distc());        
//     }
// }
