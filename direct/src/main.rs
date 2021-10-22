mod lib;

use crate::lib::factories;

fn main() {
    factories::random_string::fake();
    factories::id::fake();
    factories::id::some_what_function();
}
