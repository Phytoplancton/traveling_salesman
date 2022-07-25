import * as wasm from "../pkg/traveling_salesman.js";
import * as settings from "./settings.js";

let wasm_obj = await wasm.default();

let Data = wasm.Data.new(
    window.innerWidth, 
    window.innerHeight, 
    settings.POINT_CNT
);

let points;
let cnctns;
const updatePoints = () => {
    points = new Uint32Array(
        wasm_obj.memory.buffer, 
        Data.get_points_ptr(),
        Data.point_cnt * 2
    )
    cnctns = new Uint16Array(
        wasm_obj.memory.buffer,
        Data.get_connections_ptr(),
        Data.point_cnt
    )
}

const point = (index, coordinate) => {
    let pnt_idx = cnctns[index];
    return points[pnt_idx * 2 + coordinate];
}

export {
    point, 
    Data,
    updatePoints,
}