import { setupCanvas, toggleCell } from './canvas';
import { drawCells, drawGrid } from './render';
import { renderLoop } from './simulation';

setupCanvas(toggleCell);
drawGrid();
drawCells();
renderLoop();
