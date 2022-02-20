mod unrecoverable;
mod recoverable;
mod guess;

fn main() {
    unrecoverable::call1();

    recoverable::call1();
    recoverable::call2();
    recoverable::call3();

    guess::call1();
}
