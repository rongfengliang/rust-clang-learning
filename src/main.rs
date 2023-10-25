
#[link(name = "num", kind = "static")]
extern "C" {
    fn print_num(num:i32);
}

fn main() {
    
    print!("call c ");
    unsafe {
        print_num(333);
    } 
}
