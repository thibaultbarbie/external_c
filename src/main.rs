extern {
    fn foo() -> i32;
    fn bar(x : i32) -> i32;
}

fn main() {
    unsafe {
        println!("{}",foo());   // print 4 
        println!("{}",bar(5));  // print 25
    }
}
