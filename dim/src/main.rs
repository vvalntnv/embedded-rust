#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::adc::*;
use arduino_hal::prelude::*;

use arduino_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer2Pwm, Timer1Pwm};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    let mut adc = Adc::new(dp.ADC, Default::default());
    let potentiometer = pins.a0.into_analog_input(&mut adc);

    let timer_2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64); 
    let timer_1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);

    let mut light_1 = pins.d11.into_output().into_pwm(&timer_2);
    let mut light_2 = pins.d10.into_output().into_pwm(&timer_1);
    let mut light_3 = pins.d9.into_output().into_pwm(&timer_1);


    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
 
    loop {
        let read_potentiometer = adc.read_blocking(&potentiometer);
        let value = (read_potentiometer / 4) as u8;

        ufmt::uwriteln!(&mut serial, "{}", value).unwrap_infallible();

        if value == 0 {
            light_1.disable();
            light_2.disable();
            light_3.disable();

            continue;
        }

        light_1.enable();
        light_2.enable();
        light_3.enable();

        light_1.set_duty(value);
        light_2.set_duty(value);
        light_3.set_duty(value);
    }
}
