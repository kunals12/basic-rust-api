pub mod user;
pub use user::{create_user, hello_user};
pub mod home;
pub use home::hello;

pub fn logging(path: &str) {
    println!("{}", path)
}
