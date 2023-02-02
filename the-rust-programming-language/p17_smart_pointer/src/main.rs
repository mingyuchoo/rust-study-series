mod boxes;
mod derefs;
mod drops;
mod memory_leak;
mod rcs;
mod refcells;

fn main() {
    boxes::call1();
    boxes::call2();

    derefs::call3();

    drops::call1();
    drops::call2();

    rcs::call1();

    refcells::call1();

    memory_leak::call1();
    memory_leak::call2();
}
