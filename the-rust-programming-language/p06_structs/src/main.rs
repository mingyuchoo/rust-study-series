mod example;
mod instances;
mod methods;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    instances::create_instance();

    example::program1();
    example::program2();
    example::program3();
    example::print_rectangle();

    methods::call1();
    methods::call2();
    methods::call3();

    Ok(())
}
