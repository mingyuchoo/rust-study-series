mod mod_keyword;
mod pub_keyword;
mod use_keyword;
mod as_keyword;
mod pub_use_keyword;

fn main() {
    mod_keyword::eat_at_restaurant();

    pub_keyword::eat_at_restaurant();

    use_keyword::eat_at_restaurant();
    use_keyword::call1();

    // as_keyword::call1();
    // as_keyword::call2();

    pub_use_keyword::eat_at_restaurant();

}
