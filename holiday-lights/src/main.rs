#![no_std]
#![no_main]

use arduino_hal::{hal::port::{PB1, PB2, PB3}, port::{mode::PwmOutput, Pin}, simple_pwm::{IntoPwmPin, Prescaler, Timer1Pwm, Timer2Pwm}};
use panic_halt as _;

const DIM_TRESHOLD: u32 = 500;

struct HolidayLights<'a> {
    red_chain: &'a mut Pin<PwmOutput<Timer2Pwm>, PB3>,
    green_chain: &'a mut Pin<PwmOutput<Timer1Pwm>, PB2>,
    yellow_chain: &'a mut Pin<PwmOutput<Timer1Pwm>, PB1>,
    current_mode: u8,
    speed: u32,
    current_progress: u32,
}

impl<'a> HolidayLights<'a> {
    fn new(
        red_chain: &'a mut Pin<PwmOutput<Timer2Pwm>, PB3>, 
        green_chain: &'a mut Pin<PwmOutput<Timer1Pwm>, PB2>,
        yellow_chain: &'a mut Pin<PwmOutput<Timer1Pwm>, PB1>,
        speed: u32,
    ) -> Self { 
        red_chain.enable();
        green_chain.enable();
        yellow_chain.enable();  

        HolidayLights { red_chain, green_chain, yellow_chain, speed, current_mode: 0, current_progress: 0}
    }

    fn next_mode(&mut self) {
        self.current_mode = self.current_mode + 1;
        if self.current_mode >= 4 {
            self.current_mode = 0;
        }

        self.current_progress = 0;
    } 

    fn next_step(&mut self) {
        match self.current_mode {
            0 => self.base_mode(),
            1 => self.glow_dim(),
            2 => self.constant(),
            3 => self.quick_burst(),
            _ => self.current_mode = 0
        }
    }
 
    fn base_mode(&mut self) {
        self.current_progress += self.speed;

        let progress = (self.current_progress / 3) as u8;
        self.red_chain.set_duty(progress);
        self.green_chain.set_duty(progress);
        self.yellow_chain.set_duty(progress);
    }

    fn glow_dim(&mut self) {
        self.current_progress += self.speed;

        let mode = self.current_progress / DIM_TRESHOLD;  
        let progress = self.current_progress - (mode * DIM_TRESHOLD);
       
        let duty = 2 * progress - (2 * progress * progress / DIM_TRESHOLD);

        match mode {
            0 => self.red_chain.set_duty(duty as u8), 
            1 => self.green_chain.set_duty(duty as u8),
            2 => self.yellow_chain.set_duty(duty as u8),
            _ => {
                self.current_progress = 0;
            }
        } 
    }

    fn constant(&mut self) {
        self.red_chain.set_duty(255);
        self.green_chain.set_duty(255);
        self.yellow_chain.set_duty(255);
    }

    fn quick_burst(&mut self) {

        arduino_hal::delay_ms(100);
        match self.current_progress {
            0 => {
                self.red_chain.set_duty(255);
                self.green_chain.set_duty(0);
                self.yellow_chain.set_duty(0);
                self.current_progress += 1
            }
            1 => {
                self.red_chain.set_duty(0);
                self.green_chain.set_duty(255);
                self.yellow_chain.set_duty(0);
                self.current_progress += 1
            }
            2 => {
                self.red_chain.set_duty(0);
                self.green_chain.set_duty(0);
                self.yellow_chain.set_duty(255);
                self.current_progress += 1
            }
            _ => {
                self.current_progress = 0;
            }
        }
    }

}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale8);
    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale8);

    let mut red_chain = pins.d11.into_output().into_pwm(&timer2);
    let mut green_chain = pins.d10.into_output().into_pwm(&timer1);
    let mut yellow_chain = pins.d9.into_output().into_pwm(&timer1);

    let mut holiday_lights = HolidayLights::new(&mut red_chain, &mut green_chain, &mut yellow_chain, 2);

    let button = pins.d4.into_pull_up_input();

    loop { 
        arduino_hal::delay_ms(10);

        if button.is_high() {
            holiday_lights.next_mode();
            while button.is_high() {}
        }

        holiday_lights.next_step();
    }
}
