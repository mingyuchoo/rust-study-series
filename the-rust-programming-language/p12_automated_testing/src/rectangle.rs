#[cfg(test)]
mod tests
{
    // `tests` 모듈을 선언하기
    use super::*; // 상대경로 `super`로 상위 모듈 경로를 현재 범위 안으로 가져오기

    #[test]
    fn larger_can_hold_smaller()
    {
        let larger = Rectangle { length: 8,
                                 width:  7, };
        let smaller = Rectangle { length: 5,
                                  width:  1, };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger()
    {
        let larger = Rectangle { length: 8,
                                 width:  7, };
        let smaller = Rectangle { length: 5,
                                  width:  1, };
        assert!(!smaller.can_hold(&larger));
    }
}

#[derive(Debug)]
pub struct Rectangle
{
    length: u32,
    width:  u32,
}

impl Rectangle
{
    fn can_hold(&self,
                other: &Rectangle)
                -> bool
    {
        self.length > other.length && self.width > other.width
    }
}
