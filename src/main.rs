#![no_std]
#![no_main]

extern crate libc;

use core::{
    ptr,
    panic::PanicInfo
};


#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
    let default_panic_msg = b"the program has panicked";
    if let Some(payload) = info.payload().downcast_ref::<&str>()
    {
        unsafe{ libc::puts(payload.as_ptr() as *const i8); }
    } else
    {
        unsafe{ libc::puts(default_panic_msg.as_ptr() as *const i8); }
    }

    unsafe{ libc::abort() }
}

fn print(msg: &[u8])
{
    unsafe
    {
    if libc::printf(msg.as_ptr() as *const i8) < 0
    {
        panic!("failed to print the message");
    }
    }
}

fn xorshift(mut seed: u64) -> u64
{
    seed ^= seed << 13;
    seed ^= seed >> 7;
    seed ^ (seed << 17)
}

fn parse_uint(buf: &[u8]) -> Option<u64>
{
    let mut found_digits = false;
    let mut counter: u64 = 0;

    for byte in buf
    {
        match byte
        {
            b'0'..=b'9' =>
            {
                let number = byte - b'0';

                counter = counter.checked_mul(10)?;
                counter = counter.checked_add(number as u64)?;

                found_digits = true;
            },
            b' ' | b'\n' => (),
            b'\0' => break,
            _ => return None
        }
    }

    if found_digits
    {
        Some(counter)
    } else
    {
        None
    }
}

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize
{
    print(b"hewo! lets pley a guess the number game :3\n\0");
    let seed = unsafe{ libc::time(ptr::null_mut()) };

    let max_val = 10000;
    let correct_value = xorshift(seed as u64) % max_val + 1;

    print(b"i thought of a secret value between 1 and 10000!!\n\0");
    print(b"try to guess it and ill give u tips :3\n\0");

    loop
    {
        print(b"ur guess: \0");
        unsafe{ libc::fflush(ptr::null_mut()) };

        const BUF_SIZE: usize = 32;
        let mut input_buffer = [0_u8; BUF_SIZE];
        unsafe
        {
        let input_buffer = input_buffer.as_mut_ptr() as *mut libc::c_void;
        if libc::read(libc::STDIN_FILENO, input_buffer, BUF_SIZE) < 0
        {
            continue;
        }
        }

        let converted = match parse_uint(&input_buffer)
        {
            Some(x) => x,
            None =>
            {
                print(b"sowy i duno wut that number is,,,,\n\0");
                continue;
            }
        };

        if converted == correct_value
        {
            print(b"you got the right number!! :3\n\0");
            break;
        } else if converted < correct_value
        {
            print(b"hehe the number is bigger :3\n\0");
        } else
        {
            print(b"nyo its smoler!! \xF0\x9F\xA5\xBA\n\0");
        }
    }

    0
}
