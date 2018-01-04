extern crate gtk;


mod gui {
	pub mod gtk3;
}

fn main() {
    println!("Hello, world!");
    gui::gtk3::launch();
}
