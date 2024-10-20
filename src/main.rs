#![no_std]
#![no_main]

use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;

use avr_device::interrupt;
use core::cell::RefCell;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));

macro_rules! print {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwrite!(console, $($t)*);
                }
            },
        )
    };
}

macro_rules! println {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}

fn put_console(console: Console) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}

#[derive(PartialEq, Clone, Copy)]
enum KeypadState {
    Pressed,
    Idle,
}


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    put_console(serial);


    let mut rows = [
        pins.d2.into_output_high().downgrade(),
        pins.d3.into_output_high().downgrade(),
        pins.d4.into_output_high().downgrade(),
        pins.d5.into_output_high().downgrade(),
    ];

    let mut cols = [
        pins.d6.into_pull_up_input().downgrade(),
        pins.d7.into_pull_up_input().downgrade(),
        pins.d8.into_pull_up_input().downgrade(),
        pins.d9.into_pull_up_input().downgrade(),
    ];

        let mut key_states = [[KeypadState::Idle; 4]; 4];

    loop {
        rows.iter_mut().enumerate().for_each(|(i, row)| {
            row.set_low();

            cols.iter_mut().enumerate().for_each(|(j, col)| {
                if col.is_low() {
                if key_states[i][j] == KeypadState::Idle {
                        print_key(i, j);
                        key_states[i][j] = KeypadState::Pressed; // Update state to Pressed
                    }
                } else {
                                        key_states[i][j] = KeypadState::Idle;
                }
            });

            row.set_high();
        });
    }
}

fn print_key(i: usize, j: usize) {
    let keys = [
        [ '1', '2', '3', 'A'],
        [ '4', '5', '6', 'B'],
        [ '7', '8', '9', 'C'],
        [ '*', '0', '#', 'D'],
    ];


    println!("Pressed key: {}", keys[i][j]);

}
