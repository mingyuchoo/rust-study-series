mod hash_maps;
mod strings;
mod vectors;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    vectors::call1();

    strings::call1();
    strings::call2();
    strings::call3();
    strings::call4();
    strings::call5();
    strings::call6();

    hash_maps::call1();
    hash_maps::call2();
    hash_maps::call3();
    hash_maps::call4();
    hash_maps::call5();

    Ok(())
}
