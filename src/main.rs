mod info;
mod logo;
mod render;

use crate::info::gather_info;
use crate::render::print_with_logo;

fn main() {
    let info = gather_info();
    print_with_logo(&info);
}
