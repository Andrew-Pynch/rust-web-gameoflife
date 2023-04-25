import { CELL_SIZE } from './constants';
import { height, universe, width } from './simulation';

export const canvas = document.getElementById('game-of-life-canvas');
export const ctx = canvas.getContext('2d');

canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

export function setupCanvas(toggleCell) {
    canvas.addEventListener('mousedown', (event) => {
        isPainting = true;
        toggleCell(event);
    });

    canvas.addEventListener('mousemove', (event) => {
        if (isPainting) {
            toggleCell(event);
        }
    });

    canvas.addEventListener('mouseup', () => {
        isPainting = false;
    });

    canvas.addEventListener('mouseleave', () => {
        isPainting = false;
    });
}

export const toggleCell = (event) => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    universe.toggle_cell(row, col);

    drawGrid();
    drawCells();
};
