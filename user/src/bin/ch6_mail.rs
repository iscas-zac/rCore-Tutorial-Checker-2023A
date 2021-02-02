#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{spawn, mail_read, mail_write, sleep, wait, exit};

const BUF_LEN: usize = 256;
const MAIL_NUM: usize = 256;

#[no_mangle]
fn main() -> i32 {
    let pid = spawn("ch6_mail0");
    println!("I am father");
    println!("father sleep 1s");
    sleep(1000 as usize);
    for i in 0..16 {
        let buffer = [i as u8; BUF_LEN];
        assert_eq!(mail_write(pid as usize, &buffer), BUF_LEN as isize);
    }
    println!("father wirte 16 mails succeed");
    let mut buffer = [16 as u8; BUF_LEN];
    assert_eq!(mail_write(pid as usize, &buffer), -1);
    println!("father wirte 1 mail fail");
    println!("father sleep 2s");
    sleep(2000 as usize);
    assert_eq!(mail_write(pid as usize, &buffer), BUF_LEN as isize);
    println!("father wirte 1 mail succeed");

    let mut xstate: i32 = -100;
    assert!(wait(&mut xstate) > 0);
    assert_eq!(xstate, 0);
    println!("mail test OK!");
    0
}