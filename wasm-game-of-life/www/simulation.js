import { Universe } from 'wasm-game-of-life';
import { drawCells, drawGrid } from './render';

export const universe = Universe.new();
export const width = universe.width();
export const height = universe.height();
export const simulationSpeedSlider =
    document.getElementById('simulation-speed');
export const playPauseButton = document.getElementById('play-pause');
export let animationId = null;
export let simulationSpeed = 50;
export let isPainting = false;

simulationSpeedSlider.addEventListener('input', () => {
    if (isPaused() && simulationSpeedSlider.value > 0) {
        play();
    }
});

export const renderLoop = () => {
    drawGrid();
    drawCells();

    // Update simulation speed based on slider value
    simulationSpeed = 1000 / simulationSpeedSlider.value;

    // Check if the slider value is 0, and pause the simulation
    if (simulationSpeedSlider.value == 0) {
        pause();
        return;
    }

    // wait simulationSpeed milliseconds before rendering again
    setTimeout(() => {
        universe.tick();
        animationId = requestAnimationFrame(renderLoop);
    }, simulationSpeed);
};

export const isPaused = () => {
    return animationId === null;
};

export const play = () => {
    if (isPaused()) {
        playPauseButton.textContent = '⏸';
        renderLoop();
    }
};

export const pause = () => {
    playPauseButton.textContent = '▶';
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener('click', (event) => {
    if (isPaused()) {
        if (simulationSpeedSlider.value == 0) {
            simulationSpeedSlider.value = 1;
        }
        play();
    } else {
        pause();
    }
});

playPauseButton.textContent = '⏸';
