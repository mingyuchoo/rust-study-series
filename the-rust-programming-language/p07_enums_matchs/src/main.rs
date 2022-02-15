mod enumerations;
mod matches;
mod options;

fn main() {
    enumerations::call1();

    options::call1();
    options::call2();
    options::call3();

    matches::call1();
    matches::call2();
    matches::call3();
}
