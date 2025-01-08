mod alg;
mod data;

use alg::normal::add::add;
use alg::pro::mul::mul;
use alg::stuct::data::get_data;

fn main() {
    println!("{}", add(1, 2));
    println!("{}", mul(2, 3));
    print!("data: {}", get_data());
}