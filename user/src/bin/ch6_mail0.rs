#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{mail_read, mail_write, sleep};

const BUF_LEN: usize = 256;
const MAIL_NUM: usize = 256;

#[no_mangle]
fn main() -> i32 {
    println!("I am child");
    let mut buffer = [0u8; BUF_LEN];
    assert_eq!(mail_read(&mut buffer), -1);
    println!("child read 1 mail fail");
    println!("child sleep 2s");
    sleep(2000 as usize);
    for i in 0..16 {
        let mut buffer = [0u8; BUF_LEN];
        assert_eq!(mail_read(&mut buffer), BUF_LEN as isize);
        assert_eq!(buffer, [i as u8; BUF_LEN]);
    }
    println!("child read 16 mails succeed");
    assert_eq!(mail_read(&mut buffer), -1);
    println!("child read 1 mail fail");
    println!("child sleep 1s");
    sleep(1000 as usize);
    assert_eq!(mail_read(&mut buffer), BUF_LEN as isize);
    assert_eq!(buffer, [16 as u8; BUF_LEN]);
    println!("child read 1 mail succeed");
    println!("child exit");
    0
}