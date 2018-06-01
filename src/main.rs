#[macro_use]
extern crate test_macro as the_macro;

#[macro_use]
extern crate serde_derive;

// Issue one: not referring to the type alias through the correct path.

trait Trait { }

decl_issue_one! {
    #[derive(Deserialize)]
    pub enum TheEnum<T: Trait>;
}

// Issue two: not referring to the trait through the correct path AND drops the trait name.

struct MyDispatch;

impl ::the_macro::Dispatch for MyDispatch {
    type Call = ();
}

decl_issue_two! {
    #[derive(Deserialize)]
    pub enum TheEnum2 {
        MyDispatch,
    }
}

fn main() {

}