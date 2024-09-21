mod enumerations;
mod matches;
mod options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enumerations::call1();

    options::call1();
    options::call2();
    options::call3();

    matches::call1();
    matches::call2();
    matches::call3();

    Ok(())
}
