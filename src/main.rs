#![no_main]
#![no_std]

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use cortex_m::interrupt;
use cortex_m::iprintln;
use cortex_m::peripheral::ITM;
use cortex_m_rt::entry;
use stm32f3_discovery::{leds::Leds, stm32f3xx_hal, switch_hal};
use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::{
    delay::Delay,
    gpio::{gpioe, Output, PushPull},
    hal::blocking::delay::DelayMs,
    pac,
};
use switch_hal::{ActiveHigh, OutputSwitch, Switch};

type LedArray = [Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>; 8];

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::disable();

    let itm = unsafe { &mut *ITM::ptr() };
    let stim = &mut itm.stim[0];

    iprintln!(stim, "{}", info);

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

fn init_leds() -> (Delay, LedArray) {
    let device_peripherals = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_peripherals.RCC.constrain();

    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_peripherals.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let delay = Delay::new(core_peripherals.SYST, clocks);

    let mut gpioe = device_peripherals
        .GPIOE
        .split(&mut reset_and_clock_control.ahb);
    let leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );

    (delay, leds.into_array())
}

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = init_leds();
    let ms = 50_u8;
    let mut current = 0;

    loop {
        match current {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                let next = (current + 1) % 8;

                leds[next].on().ok();
                delay.delay_ms(ms);

                leds[current].off().ok();
                delay.delay_ms(ms);

                current += 1;
            }
            _ => {
                current = 0;
            }
        }
    }
}
