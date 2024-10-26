#[derive(Debug)]
struct Rectangle
{
    width:  u32,
    height: u32,
}

/// Defining Methods
impl Rectangle
{
    fn area(&self) -> u32
    {
        self.width * self.height
    }

    // Associated function:
    //   - are associated with the type named after the `impl`.
    //   - don'tneed an instance of the type ot work with.
    fn square(size: u32) -> Rectangle
    {
        Rectangle { width:  size,
                    height: size, }
    }
}

/// Multiple `impl` Blocks
impl Rectangle
{
    fn can_hold(&self,
                other: &Rectangle)
                -> bool
    {
        self.width > other.width && self.height > other.height
    }
}

pub fn call1()
{
    let rect1 = Rectangle { width:  30,
                            height: 50, };

    println!("사각형의 면적: {} 제곱 픽셀", rect1.area());
}

pub fn call2()
{
    let rect1 = Rectangle { width:  30,
                            height: 50, };
    let rect2 = Rectangle { width:  10,
                            height: 40, };
    let rect3 = Rectangle { width:  60,
                            height: 45, };

    println!("rect1은 rect2를 포함하는가? {}", rect1.can_hold(&rect2));
    println!("rect1은 rect3를 포함하는가? {}", rect1.can_hold(&rect3));
}

pub fn call3()
{
    let rect1 = Rectangle::square(40);
    println!("사각형의 면적: {} 제곱 픽셀", rect1.area());
}
