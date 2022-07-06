import * as settings from "./settings.js";

const canvas = document.body.appendChild(
    document.createElement("canvas")
    );
const ctx = canvas.getContext('2d');

const defineCanvas = () => {
    canvas.style.position = "absolute";
    canvas.style.background = settings.CANVAS_COLOR()
    canvas.style.border = `1px solid `;
    document.body.style.margin = '0 px';
    canvas.style.zIndex = '0';
};
defineCanvas();

const resizeCanvas = () => {
    canvas.width = window.innerWidth - 20;
    canvas.height = window.innerHeight - 20;
}
resizeCanvas();
window.addEventListener('resize', resizeCanvas);

export {
    ctx,
    canvas,
}