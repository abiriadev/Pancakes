use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    println!("Hello, world!");

    Pancakes::hello_macro();

    // let sql = sql!(SELECT * FROM posts WHERE id = 1);
    //
    // println!("{:?}", sql);
}
