fn serve_order() {}

mod back_of_house {
  fn fix_incorrect_order() {
    cook_order();

    // use relative path by `super` keyword
    super::serve_order();
  }
  fn cook_order() {}
}