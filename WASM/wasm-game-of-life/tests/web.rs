//! Game of Life 테스트 스위트

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;
use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

/// 빈 우주에서 tick 후에도 모든 셀이 Dead인지 확인한다
#[wasm_bindgen_test]
fn empty_universe_stays_empty() {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    // set_width/set_height가 모든 셀을 Dead로 초기화함
    universe.tick();

    let expected_cells: Vec<u8> = vec![0; 36];
    let actual: Vec<u8> = universe.get_cells().iter().map(|c| *c as u8).collect();
    assert_eq!(actual, expected_cells);
}

/// Blinker 패턴 (주기 2 진동자) tick 검증
///
/// 초기 상태 (세로):    tick 후 (가로):
///   . . . . . .        . . . . . .
///   . . # . . .        . . . . . .
///   . . # . . .        . # # # . .
///   . . # . . .        . . . . . .
///   . . . . . .        . . . . . .
///   . . . . . .        . . . . . .
#[wasm_bindgen_test]
fn blinker_oscillates() {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);

    // 세로 Blinker 설정
    universe.set_cells(&[(1, 2), (2, 2), (3, 2)]);

    universe.tick();

    // tick 후 가로 Blinker가 되어야 함
    let cells = universe.get_cells();
    let alive_positions: Vec<(u32, u32)> = cells
        .iter()
        .enumerate()
        .filter(|(_, c)| **c as u8 == 1)
        .map(|(i, _)| ((i / 6) as u32, (i % 6) as u32))
        .collect();

    assert_eq!(alive_positions, vec![(2, 1), (2, 2), (2, 3)]);
}
