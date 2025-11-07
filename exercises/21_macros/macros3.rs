// TODO: Fix the compiler error without taking the macro definition out of this
// module.

mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }//i'll let macro be usable out of module
	pub(crate) use my_macro;
}

fn main() {
   // use macros::macro_rules;	
   macros::my_macro!();
}
