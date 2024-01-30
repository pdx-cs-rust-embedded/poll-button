//! Randomize the MB2 LED "display" while the A button is pressed.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
#[rustfmt::skip]
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{
        prelude::*,
        timer::Timer,
    },
};
use nanorand::{pcg64::Pcg64, Rng};

use panic_halt as _;

/// Start a new run on the frame buffer `fb` from a random
/// position given by the `rng`.
fn randomize_fb(rng: &mut Pcg64, fb: &mut [[u8; 5]; 5]) {
    for r in fb.iter_mut() {
        for cell in r.iter_mut() {
            *cell = rng.generate_range(0..=1);
        }
    }
}

#[entry]
fn init() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let buttons = board.buttons;
    let mut fb = [[0; 5]; 5];
    let mut rng = Pcg64::new_seed(1);

    loop {
        if buttons.button_a.is_low().unwrap() {
            randomize_fb(&mut rng, &mut fb);
        }
        display.show(&mut timer, fb, 100);
    }
}
