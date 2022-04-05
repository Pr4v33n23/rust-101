use fibonacci::fibonacci;

mod fibonacci;

fn main() {

    let buffer = fibonacci::fibonacci();
    println!("{:?}", buffer);
}


