#![no_std]
#![no_main]

use arduino_hal::{hal::port::{PD3, PD4, PD5}, port::{mode::Output, Pin}};
use embedded_hal::digital::v2::InputPin;
use panic_halt as _;

const WAIT_STATE: u8 = 0b00;
const GO_STATE: u8 = 0b01;

struct TrafficLight {
    red: Pin<Output, PD5>,
    yellow: Pin<Output, PD4>,
    green: Pin<Output, PD3>,
    current_state: u8
}

impl TrafficLight {
    fn new(red: Pin<Output, PD5>, yellow: Pin<Output, PD4>, green: Pin<Output, PD3>) -> Self {
        let mut traffic_light = TrafficLight{
            red,
            yellow,
            green,
            current_state: WAIT_STATE
        };
        
        // Assert ll is working
        traffic_light.all_high();
        traffic_light.all_low();
        

        // Siwtch to wait state
        traffic_light.red.set_high();

        traffic_light
        
    }

    fn change_state(&mut self, transition_time_in_s: u16) {
        if self.current_state == WAIT_STATE {
            self.to_go(transition_time_in_s);
        } else {
            self.to_wait(transition_time_in_s);
        }
    }

    fn to_go(&mut self, transition_time: u16) {
        self.red.set_high(); 
        self.yellow.set_high();

        arduino_hal::delay_ms(transition_time * 1000);

        self.red.set_low();
        self.yellow.set_low();
        self.green.set_high();

        self.current_state = GO_STATE;
    }

    fn to_wait(&mut self, transition_time: u16) {
        self.green.set_low();
        self.yellow.set_high();

        arduino_hal::delay_ms(transition_time * 1000);

        self.yellow.set_low();
        self.red.set_high();

        self.current_state = WAIT_STATE;
    }

    fn all_high(&mut self) {
        self.red.set_high();
        self.yellow.set_high();
        self.green.set_high();
    }

    fn all_low(&mut self) {
        self.red.set_low();
        self.yellow.set_low();
        self.green.set_low();
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let red = pins.d5.into_output();
    let yellow = pins.d4.into_output();
    let green = pins.d3.into_output();

    let mut traffic_light = TrafficLight::new(red, yellow, green);
    let transition_time_in_s: u16 = 2;

    let pedestrian_button = pins.d2;


    loop {
        // if pedestrian_button.is_high(){
        //     traffic_light.all_high();
        // } else {
        //     traffic_light.all_low();
        // }
        if pedestrian_button.is_high(){
            traffic_light.to_wait(transition_time_in_s);
            arduino_hal::delay_ms(10 * 1000);
        } else { 
            arduino_hal::delay_ms(10 * 1000);
            traffic_light.change_state(transition_time_in_s);
        }
    }
}

