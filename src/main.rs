

mod bindings;
use bindings::print_num;

fn main() {

    print!("call c ");
    unsafe {
        print_num(333);
    } 
}
