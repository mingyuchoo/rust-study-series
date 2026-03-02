mod date_util;
mod id_gen;
pub mod manager;
pub mod model;
pub mod stats;
pub mod validation;

use wasm_bindgen::prelude::*;

// 기존 예제 함수 유지 (하위 호환)
#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize { left + right }

// manager::DiaryManager와 model::Mood는 #[wasm_bindgen]으로 직접 노출됨

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_함수가_정상_동작한다() {
        assert_eq!(add(2, 2), 4);
    }
}
