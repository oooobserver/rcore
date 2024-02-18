#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Hello, shenye!");
    let nums = [114,514,123,456];
    for n in nums{
        println!("{}", n);
    } 
    0
}