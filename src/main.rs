pub mod readable;
use readable::roll_using_readable_lemire;

fn main() {
    for _n in 0..150 {
        println!("rolled a {}", roll_using_readable_lemire(6));
    }
}
