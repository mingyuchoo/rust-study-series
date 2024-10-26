pub fn scope()
{
    println!("-- scope()");

    let s = "hello";
    {
        {
            // Inner scope
            let s = "world";
            println!("{s}");
        }
        println!("{s}");
    }

    println!("{s}");
}
