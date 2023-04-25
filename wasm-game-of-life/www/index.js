import { setupCanvas, toggleCell } from './canvas';
import { drawCells, drawGrid } from './render';
import { initialize } from './simulation';

setupCanvas(toggleCell);
drawGrid();
drawCells();
initialize();
