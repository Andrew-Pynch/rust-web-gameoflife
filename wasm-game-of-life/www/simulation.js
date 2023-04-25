import { Universe } from 'wasm-game-of-life';
import { fps } from './fps';
import { drawCells, drawGrid } from './render';

export const universe = Universe.new();

export const width = universe.get_width();
export const height = universe.get_height();

export const simulationSpeedSlider =
    document.getElementById('simulation-speed');
export const playPauseButton = document.getElementById('play-pause');
export let animationId = null;
export let simulationSpeed = 50;
export let isPainting = false;

export const initialize = () => {
    animationId = requestAnimationFrame(renderLoop);
};

simulationSpeedSlider.addEventListener('input', () => {
    if (isPaused() && simulationSpeedSlider.value > 0) {
        play();
    }
});

export const renderLoop = () => {
    fps.render();

    drawGrid();
    drawCells();

    for (let i = 0; i < 9; i++) {
        universe.tick();
    }

    animationId = requestAnimationFrame(renderLoop);

    // // Update simulation speed based on slider value
    // simulationSpeed = 1000 / simulationSpeedSlider.value;

    // // Check if the slider value is 0, and pause the simulation
    // if (simulationSpeedSlider.value == 0) {
    //     pause();
    //     return;
    // }

    // // wait simulationSpeed milliseconds before rendering again
    // setTimeout(() => {
    //     universe.tick();
    //     animationId = requestAnimationFrame(renderLoop);
    // }, simulationSpeed);

    // if (animationId === 0) {
    //     animationId = null;
    // }
};

export const isPaused = () => {
    return animationId === null;
};

export const play = () => {
    playPauseButton.textContent = 'Pause';
    renderLoop();
};

export const pause = () => {
    playPauseButton.textContent = 'Play';
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener('click', (event) => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

playPauseButton.textContent = 'Pause';
