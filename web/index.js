import * as visual from "./visual.js";
import * as data from "./data.js";



const two_opt_loop = () => {
    visual.render();
    
    if (data.Data.two_opt_mut_step()) {
        requestAnimationFrame(two_opt_loop);
    }
}
const next_ngbr_loop = () => {
    visual.render();
    
    if (data.Data.nxt_ngbr_mut_step()) {
        requestAnimationFrame(next_ngbr_loop);
    }
}

const two_opt = () => {
    visual.drawStart();
    data.Data.complete_two_opt();
    setTimeout(() => {
        visual.render();
    }, 1000)
}

let n = 0;
let m = 0;
let o = 0;
const solve_loop = () => {
    visual.render();
    if (data.Data.two_opt_mut_step()) {
        requestAnimationFrame(solve_loop);
    }
    else if (n++ < 300) {
        requestAnimationFrame(solve_loop);
    }
    else if (data.Data.nxt_ngbr_mut_step()) {
        requestAnimationFrame(solve_loop);
    }
    // else if (m++ < 50) {
    //     requestAnimationFrame(solve);
    // }
    // else if (data.Data.two_opt_mut_step()) {
    //     requestAnimationFrame(solve);
    // }
    else if (o++ < 100) {
        requestAnimationFrame(solve_loop);
    }
    
}

// solve();

two_opt();

// next_ngbr();
