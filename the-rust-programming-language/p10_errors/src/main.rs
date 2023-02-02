mod guess;
mod recoverable;
mod unrecoverable;

fn main() {
    unrecoverable::call1();

    recoverable::call1();
    recoverable::call2();
    recoverable::call3();

    guess::call1();
}
