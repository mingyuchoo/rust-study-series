use idiomatic_expressions::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    _ = idiomatic_options::options();
    _ = idiomatic_results::results();

    Ok(())
}
