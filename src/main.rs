#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::InputPin;
use microbit::{board::Board, display::blocking::Display, hal::Timer};

#[entry]
fn main() -> ! {
    if let Some(mut board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display = Display::new(board.display_pins);

        #[allow(non_snake_case)]
        let letter_I = [
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        let heart = [
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_R = [
            [0, 1, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_u = [
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_s = [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_t = [
            [1, 1, 1, 1, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];
        loop {
            if let Ok(true) = board.buttons.button_a.is_low() {
                display.show(&mut timer, [
                    [1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1],
                ], 1000);
                display.clear();
                timer.delay_ms(250_u32);
            } else {
                display.show(&mut timer, letter_I, 1000);
                display.show(&mut timer, heart, 1000);
                display.show(&mut timer, letter_R, 1000);
                display.show(&mut timer, letter_u, 1000);
                display.show(&mut timer, letter_s, 1000);
                display.show(&mut timer, letter_t, 1000);
                display.clear();
                timer.delay_ms(250_u32);
            }
        }
    }

    panic!("End");
}