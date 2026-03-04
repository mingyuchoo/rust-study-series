mod utils;

use js_sys::Math;
use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// 셀의 상태를 나타내는 열거형
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead  = 0,
    Alive = 1,
}

impl Cell {
    /// 셀의 상태를 토글한다 (Dead ↔ Alive)
    fn toggle(&mut self) {
        *self = match *self {
            | Cell::Dead => Cell::Alive,
            | Cell::Alive => Cell::Dead,
        };
    }
}

/// Conway's Game of Life 우주(Universe)
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

/// 테스트 전용 메서드 (wasm_bindgen 미적용)
impl Universe {
    /// 셀 배열을 직접 가져온다 (테스트용)
    pub fn get_cells(&self) -> &[Cell] { &self.cells }

    /// 지정된 좌표의 셀을 Alive로 설정한다 (테스트용)
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for &(row, col) in cells.iter() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

impl Default for Universe {
    fn default() -> Self { Self::new() }
}

#[wasm_bindgen]
impl Universe {
    /// 64x64 크기의 우주를 랜덤 패턴으로 생성한다
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;

        let cells = (0 .. width * height)
            .map(|_| if Math::random() < 0.5 { Cell::Alive } else { Cell::Dead })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    /// Conway 규칙에 따라 다음 세대를 계산한다
    ///
    /// 규칙:
    /// 1. 살아있는 셀의 이웃이 2개 미만이면 죽음 (과소)
    /// 2. 살아있는 셀의 이웃이 2~3개이면 생존
    /// 3. 살아있는 셀의 이웃이 3개 초과이면 죽음 (과밀)
    /// 4. 죽은 셀의 이웃이 정확히 3개이면 탄생
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0 .. self.height {
            for col in 0 .. self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    | (Cell::Alive, x) if x < 2 => Cell::Dead,
                    | (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    | (Cell::Alive, x) if x > 3 => Cell::Dead,
                    | (Cell::Dead, 3) => Cell::Alive,
                    | (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    /// 우주의 너비를 반환한다
    pub fn width(&self) -> u32 { self.width }

    /// 우주의 높이를 반환한다
    pub fn height(&self) -> u32 { self.height }

    /// 셀 배열의 WASM 메모리 포인터를 반환한다
    pub fn cells(&self) -> *const Cell { self.cells.as_ptr() }

    /// 지정된 좌표의 셀 상태를 토글한다
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }

    /// 우주의 너비를 변경한다 (모든 셀을 Dead로 초기화)
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0 .. width * self.height).map(|_| Cell::Dead).collect();
    }

    /// 우주의 높이를 변경한다 (모든 셀을 Dead로 초기화)
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0 .. self.width * height).map(|_| Cell::Dead).collect();
    }

    /// 텍스트로 렌더링한 결과를 반환한다
    pub fn render(&self) -> String { self.to_string() }
}

impl Universe {
    /// (row, col) 좌표를 1차원 인덱스로 변환한다
    fn get_index(&self, row: u32, column: u32) -> usize { (row * self.width + column) as usize }

    /// 주어진 셀의 살아있는 이웃 수를 센다 (토러스 토폴로지)
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{symbol}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
