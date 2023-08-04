#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut on_off_matrix = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let (mut row_idx, mut col_idx) = (0, 0);

    loop {
        on_off_matrix[row_idx][col_idx] = 1;
        display.show(&mut timer, on_off_matrix, 10_u32);

        display.clear();
        on_off_matrix[row_idx][col_idx] = 0;
        timer.delay_ms(10_u32);

        if row_idx == 0 && col_idx != 4 {
            col_idx += 1
        } else if row_idx != 4 && col_idx == 4 {
            row_idx += 1
        } else if row_idx == 4 && col_idx != 0 {
            col_idx -= 1
        } else if row_idx != 0 && col_idx == 0 {
            row_idx -= 1
        } else {
            panic!("shit, what happened?")
        } 
    }
}