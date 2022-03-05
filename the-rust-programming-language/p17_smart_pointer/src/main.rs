mod boxes;
mod derefs;
mod drops;

fn main() {
  boxes::call1();
  boxes::call2();

  derefs::call3();

  drops::call1();
  drops::call2();
}
