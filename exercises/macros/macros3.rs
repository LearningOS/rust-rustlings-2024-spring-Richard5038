// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

// #[macro_use] attribute can be used to make a module's macro
// scope not end when the module is closed.

mod macros {
    //Macros defined in a module are not visible outside that module by default. To make the code compile, we need to make the macro my_macro accessible from the main module. You can do this by using the #[macro_export] attribute on the module that contains the macro definition.
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
