//! # Art
//!
//! 미술품 모델링하기 위한 라이브러리

pub use self::kinds::PrimaryColor; // 모듈 경로를 범위 안으로 가져오기
pub use self::kinds::SecondaryColor; // 모듈 경로를 범위 안으로 가져오기
pub use self::utils::mix; // 모듈 경로를 범위 안으로 가져오기

pub mod kinds {
    // `kinds` 모듈을 선언하고 공기하기

    /// RYB 색상 모델에 따른 주 색상
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// RYB 색상 모델에 따른 보조 색상
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    // `utils` 모듈을 선언하고 공기하기

    use crate::kinds::*; // 절대경로로 모듈 을 현재 범위 안으로 가져오기

    /// 두 개의 주 색상을 조합해서 보조 색상을 생성합니다.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // TODO: change here
        SecondaryColor::Green
    }
}
