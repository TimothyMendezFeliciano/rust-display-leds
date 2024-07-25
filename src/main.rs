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
        let mut index = 2;
        let encendido = 1;

        #[allow(non_snake_case)]
        let mut letter_t = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, encendido, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];

        loop {
            if let Ok(true) = board.buttons.button_a.is_low() {
                if index < 4 {
                    letter_t[2][index] = 0;
                    letter_t[2][index+1] = encendido;
                    index+=1;
                }
                display.show(&mut timer, letter_t, 2000);
                display.clear();
                timer.delay_ms(250_u32);
            } else if let Ok(true) = board.buttons.button_b.is_low() {
                if index >0 {
                    letter_t[2][index] = 0;
                    letter_t[2][index-1] = encendido;
                    index-=1;
                }
                display.show(&mut timer, letter_t, 2000);
                display.clear();
                timer.delay_ms(250_u32);
            }
        }
    }

    panic!("End");
}