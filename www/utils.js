import { width } from './simulation';

export function getIndex(row, column) {
    return row * width + column;
}

export function bitIsSet(n, arr) {
    const byte = Math.floor(n / 8);
    const mask = 1 << n % 8;
    return (arr[byte] & mask) === mask;
}
