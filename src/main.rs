#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(const_fn_transmute)]
#![feature(global_asm)]

#[macro_use]
extern crate bitflags;
extern crate spin;

#[macro_use]
pub mod arch;
pub use arch::*;

pub mod io;
pub use io::*;

pub mod panic;

pub mod drivers;

const BUFFER_SIZE: usize = 128;
const ARG_BUFFER_SIZE: usize = 32;

fn kmain() -> ! {
    println!("--==firefly==--");
    print!(">> ");

    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut buffer_idx = 0usize;

    loop {
        let mut c_raw = Option::<u8>::None;
        {
            let mut uart = arch::devices::serial::UART1.lock();
            if let Some(ref mut u) = *uart {
                c_raw = u.read()
            }
        }

        if let Some(c) = c_raw {
            match c {
                8 | 0x7F => {
                    if buffer_idx > 0 {
                        buffer[buffer_idx - 1] = 0;
                        buffer_idx -= 1;
                    }
                    print!("{}", 8 as char);
                }
                b'\n' | b'\r' => {
                    println!();
                    buffer[buffer_idx] = b' ';
                    buffer_idx += 1;

                    let command_str = match core::str::from_utf8(&buffer) {
                        Ok(com) => com.trim(),
                        Err(e) => {
                            println!("error: {}", e);
                            let _ = buffer.iter_mut().for_each(|i| *i = 0);
                            buffer_idx = 0;
                            continue;
                        }
                    };

                    let mut command_iter = command_str.split_whitespace();
                    let command = match command_iter.next() {
                        Some(com) => com,
                        None => continue,
                    };

                    let mut args: [&str; ARG_BUFFER_SIZE] = [""; ARG_BUFFER_SIZE];
                    let mut arg_count = 0;
                    args.iter_mut().zip(command_iter).for_each(|(buffer, arg)| {
                        *buffer = arg;
                        arg_count += 1
                    });

                    match command {
                        "ls" => println!("no file system"),
                        "help" => println!("no help for you"),
                        "quit" | "exit" => println!("you cannot leave"),
                        "echo" => {
                            args.iter().for_each(|arg| print!("{} ", arg));
                            println!();
                        }
                        _ => println!("invalid command: {}", command),
                    }

                    buffer.iter_mut().for_each(|i| *i = 0);
                    buffer_idx = 0;

                    print!(">> ");
                }
                _ => {
                    if buffer_idx > BUFFER_SIZE - 1 {
                        println!("buffer exceeded\nignoring further inputs");
                        continue;
                    }
                    buffer[buffer_idx] = c;
                    buffer_idx += 1;
                    print!("{}", c as char);
                }
            }
        }
    }
}
