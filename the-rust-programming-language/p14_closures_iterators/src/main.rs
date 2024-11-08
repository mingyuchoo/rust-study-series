use p14_closures_iterators::{closures,
                             iterators};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    call_closures();
    call_iterators();
}

fn call_closures() {
    println!("-- call1()");
    closures::call1();

    println!("-- call2()");
    closures::call2();

    println!("-- call3()");
    closures::call3();

    println!("-- call4()");
    closures::call4();

    println!("-- call5()");
    closures::call5();

    println!("-- call6()");
    closures::call6();

    Ok(())
}

fn call_iterators() {
    iterators::call1();
}
