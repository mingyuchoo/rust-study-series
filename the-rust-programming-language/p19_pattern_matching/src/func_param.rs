pub fn call()
{
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32))
{
    println!("현재위치:({x},{y})");
}
