import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg.wasm";

// 셀 크기 및 색상 상수
const CELL_SIZE = 8;
const GRID_COLOR = "#2a2a4a";
const DEAD_COLOR = "#16213e";
const ALIVE_COLOR = "#e94560";

// Universe 생성
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Canvas 설정
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
const ctx = canvas.getContext("2d");

// 컨트롤 요소
const playPauseBtn = document.getElementById("play-pause");
const speedSlider = document.getElementById("speed");
const fpsDisplay = document.getElementById("fps-display");

let animationId = null;
let targetFps = parseInt(speedSlider.value, 10);

// 그리드 선을 그린다
function drawGrid() {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // 세로선
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // 가로선
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
}

// WASM 메모리에서 셀을 읽어 그린다
function drawCells() {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  // Alive 셀 그리기
  ctx.fillStyle = ALIVE_COLOR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = row * width + col;
      if (cells[idx] !== Cell.Alive) continue;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  // Dead 셀 그리기
  ctx.fillStyle = DEAD_COLOR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = row * width + col;
      if (cells[idx] !== Cell.Dead) continue;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
}

// 애니메이션 루프
let lastFrameTime = 0;

function renderLoop(timestamp) {
  const interval = 1000 / targetFps;

  if (timestamp - lastFrameTime >= interval) {
    universe.tick();
    drawGrid();
    drawCells();
    lastFrameTime = timestamp;
  }

  animationId = requestAnimationFrame(renderLoop);
}

// 실행 중 여부 확인
function isRunning() {
  return animationId !== null;
}

// Play/Pause 토글
function play() {
  playPauseBtn.textContent = "⏸ 일시정지";
  animationId = requestAnimationFrame(renderLoop);
}

function pause() {
  playPauseBtn.textContent = "▶ 재생";
  cancelAnimationFrame(animationId);
  animationId = null;
}

playPauseBtn.addEventListener("click", () => {
  if (isRunning()) {
    pause();
  } else {
    play();
  }
});

// 속도 조절
speedSlider.addEventListener("input", (event) => {
  targetFps = parseInt(event.target.value, 10);
  fpsDisplay.textContent = `${targetFps} fps`;
});

// Canvas 클릭으로 셀 토글
canvas.addEventListener("click", (event) => {
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
});

// 초기 렌더링 후 자동 시작
drawGrid();
drawCells();
play();
