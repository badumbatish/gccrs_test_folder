#![feature(rustc_attrs)]

#[rustc_builtin_macro]
macro_rules! asm {
    () => {}
}

fn main() {
    //asm!("hi");
    asm!("mov {}, 5", out(reg) x);
}


