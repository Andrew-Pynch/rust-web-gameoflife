/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Cell {
  Dead = 0,
  Alive = 1,
}
/**
*/
export class Universe {
  free(): void;
/**
* @returns {Universe}
*/
  static new(): Universe;
/**
* @returns {number}
*/
  get_width(): number;
/**
* @returns {number}
*/
  get_height(): number;
/**
* @returns {number}
*/
  cells(): number;
/**
* Set the width of the universe.
*
* Resets all cells to the dead state.
* @param {number} width
*/
  set_width(width: number): void;
/**
* Set the height of the universe.
*
* Resets all cells to the dead state.
* @param {number} height
*/
  set_height(height: number): void;
/**
* @param {number} row
* @param {number} column
*/
  toggle_cell(row: number, column: number): void;
/**
* @param {number} row
* @param {number} col
*/
  spawn_glider(row: number, col: number): void;
/**
* @param {number} row
* @param {number} col
*/
  spawn_pulsar(row: number, col: number): void;
/**
*/
  tick(): void;
}
