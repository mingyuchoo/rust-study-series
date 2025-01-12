#[derive(Debug)]
struct Rectangle {
    width:  u32,
    height: u32,
}

/// 1. using variables
pub fn program1() {
    let width1 = 30;
    let height1 = 50;

    println!("사각형의 면적: {} 제곱 픽셀", area(width1, height1));

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}

/// 2. using a tuple
pub fn program2() {
    let rect1 = (30, 50);

    println!("사각형의 면적: {} 제곱 픽셀", area(rect1));

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
}

/// 3. using a structure
pub fn program3() {
    let rect1 = Rectangle {
        width:  30,
        height: 50,
    };

    println!("사각형의 면적: {} 제곱 픽셀", area(&rect1));

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}

pub fn print_rectangle() {
    let rect1 = Rectangle {
        width:  30,
        height: 50,
    };
    println!("rect1: {rect1:?}");
    println!("rect1: {rect1:#?}");
}
