#![no_main]
#![no_std]

#[allow(unused_imports)]
use panic_semihosting;

use rtfm::app;

use nrf52840_hal::{gpio, prelude::*};

use nrf52840_pac as pac;

#[app(device = nrf52840_pac)]
const APP: () = {
    static mut TIMER: pac::TIMER1 = ();
    static mut LED_RED: gpio::Pin<gpio::Output<gpio::PushPull>> = ();
    static mut LED_GREEN: gpio::Pin<gpio::Output<gpio::PushPull>> = ();
    static mut LED_BLUE: gpio::Pin<gpio::Output<gpio::PushPull>> = ();
    static mut STATE: u8 = 0u8;
    static mut RTC: pac::RTC0 = ();

    #[init]
    fn init() {
        let p0 = device.P0.split();
        let p1 = device.P1.split();
        // Configure low frequency clock source
        device
            .CLOCK
            .lfclksrc
            .write(|w| w.src().xtal().external().disabled().bypass().disabled());
        // Start high frequency clock
        device.CLOCK.events_hfclkstarted.reset();
        device
            .CLOCK
            .tasks_hfclkstart
            .write(|w| w.tasks_hfclkstart().set_bit());
        // Start low frequency clock
        device.CLOCK.events_lfclkstarted.reset();
        device
            .CLOCK
            .tasks_lfclkstart
            .write(|w| w.tasks_lfclkstart().set_bit());

        // Configure timer1 to generate a interrupt every second
        let timer1 = device.TIMER1;
        timer1.mode.write(|w| w.mode().timer());
        timer1.bitmode.write(|w| w.bitmode()._32bit());
        timer1.prescaler.write(|w| unsafe { w.prescaler().bits(4) });
        timer1.cc[0].write(|w| unsafe { w.bits(100000) });
        timer1.shorts.write(|w| w.compare0_stop().enabled());
        timer1.intenset.write(|w| w.compare0().set_bit());
        timer1.tasks_start.write(|w| w.tasks_start().set_bit());

        let rtc = device.RTC0;
        rtc.intenset.write(|w| w.tick().set_bit() );
        rtc.prescaler.write(|w| unsafe { w.prescaler().bits(4095) });
        rtc.tasks_start.write(|w| w.tasks_start().set_bit() );

        TIMER = timer1;
        LED_RED = p0.p0_08.degrade().into_push_pull_output(gpio::Level::High);
        LED_GREEN = p1.p1_09.degrade().into_push_pull_output(gpio::Level::High);
        LED_BLUE = p0.p0_12.degrade().into_push_pull_output(gpio::Level::Low);
        RTC = rtc;
    }

    #[interrupt(resources = [TIMER, LED_RED, LED_GREEN, LED_BLUE, STATE],)]
    fn TIMER1() {
        let timer = resources.TIMER;
        // Clear event and restart
        timer.events_compare[0].write(|w| w.events_compare().clear_bit());
        timer.tasks_clear.write(|w| w.tasks_clear().set_bit());
        timer.tasks_start.write(|w| w.tasks_start().set_bit());
        /*
        *resources.STATE = match *resources.STATE {
            0 => {
                (*resources.LED_RED).set_high();
                (*resources.LED_GREEN).set_high();
                (*resources.LED_BLUE).set_low();
                1
            },
            1 => {
                (*resources.LED_RED).set_high();
                (*resources.LED_GREEN).set_low();
                (*resources.LED_BLUE).set_high();
                2
            },
            2 => {
                (*resources.LED_RED).set_low();
                (*resources.LED_GREEN).set_high();
                (*resources.LED_BLUE).set_high();
                0
            },
            _ => {
                0
            }
        }
        */
    }

    #[interrupt(resources = [RTC, LED_RED, LED_GREEN, LED_BLUE, STATE],)]
    fn RTC0() {
        let rtc = resources.RTC;
        // Clear event and restart
        rtc.events_tick.write(|w| w.events_tick().clear_bit());

        *resources.STATE = match *resources.STATE {
            0 => {
                (*resources.LED_RED).set_high();
                (*resources.LED_GREEN).set_high();
                (*resources.LED_BLUE).set_low();
                1
            },
            1 => {
                (*resources.LED_RED).set_high();
                (*resources.LED_GREEN).set_low();
                (*resources.LED_BLUE).set_high();
                2
            },
            2 => {
                (*resources.LED_RED).set_low();
                (*resources.LED_GREEN).set_high();
                (*resources.LED_BLUE).set_high();
                0
            },
            _ => {
                0
            }
        }
    }
};
