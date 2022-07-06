import * as canvas from "./canvas.js";
import * as data from "./data.js";
import * as settings from "./settings.js";

const drawPoints = () => {
    canvas.ctx.fillStyle = settings.DRAWING_COLOR;
    canvas.ctx.font = settings.FONT;
  
    for (let i = 0; i < data.Data.point_cnt; i++) {
        let x = data.point(i, 0);
        let y = data.point(i, 1);
        canvas.ctx.beginPath();
        canvas.ctx.arc(x, y, settings.RADIUS, 0, Math.PI * 2, false);
        canvas.ctx.fill();
        
        // canvas.ctx.fillText(i, x - 6, y + 25)
    }
}
  
const drawLines = () => {
    canvas.ctx.strokeStyle = settings.DRAWING_COLOR;
    canvas.ctx.lineWidth = settings.LINE_WIDTH;

    canvas.ctx.beginPath();
    let x = data.point(0, 0);
    let y = data.point(0, 1);
    canvas.ctx.moveTo(x, y);
    for (let i = 0; i < data.Data.point_cnt; i++) {
        let x = data.point(i, 0);
        let y = data.point(i, 1);
        canvas.ctx.lineTo(x, y);
    }
    canvas.ctx.closePath();
    canvas.ctx.stroke();

}

const drawLeng = () => {
    canvas.ctx.fillStyle = settings.DRAWING_COLOR;
    canvas.ctx.font = settings.FONT;
    let x =  window.innerWidth - settings.TEXT_WIDTH_MARGIN;
    let y = window.innerHeight - settings.TEXT_HEIGHT_MARGIN;
    let text = 
        `total length: ${data.Data.get_total_distc()}`;

    canvas.ctx.clearRect(x - 10, y - 20, window.innerWidth - x, window.innerHeight - y);
    canvas.ctx.fillText(text, x, y);
}

const drawStart = () => {
    canvas.ctx.fillStyle = settings.DRAWING_COLOR;
    canvas.ctx.font = settings.FONT;

    let x = settings.TEXT_WIDTH_MARGIN;
    let y = window.innerHeight - settings.TEXT_HEIGHT_MARGIN;
    let text = "start";

    canvas.ctx.fillText(text, x, y);

    
}

const render = () => {
    canvas.ctx.fillStyle = settings.CANVAS_COLOR(0.03)
    canvas.ctx.fillRect(0, 0, canvas.canvas.width, canvas.canvas.height);
    data.updatePoints();
    drawLines();
    drawPoints();
    drawLeng();
}


export {
    render,
    drawStart
}