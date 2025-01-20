#![no_std]
#![no_main]
use core::cell::RefCell;
use core::ops::DerefMut;

use cortex_m::interrupt::{self as cortexinterrupt, Mutex};
// use cortex_m::peripheral;
use tm4c123x;
// use tm4c123x::interrupt as tm4interrupt;
use tm4c123x::interrupt;
// use tm4c123x::CorePeripherals;

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
use cortex_m_rt::entry;

// static MY_GPIO: Mutex<RefCell<Option<tm4c123x::GPIO_PORTF>>> = Mutex::new(RefCell::new(None));
// static MY_TIMER1: Mutex<RefCell<Option<tm4c123x::TIMER1>>> = Mutex::new(RefCell::new(None));
static G_PERIPH: Mutex<RefCell<Option<tm4c123x::Peripherals>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // https://microcontrollerslab.com/timer-interrupt-tm4c123-generate-delay-with-timer-interrupt-service-routine/

    let peripherals = tm4c123x::Peripherals::take().unwrap();
    let cperipherals = tm4c123x::CorePeripherals::take().unwrap();

    // GPIO Port F Run Mode Clock Gating Control
    // Set bit 5 - Enable and provide a clock to GPIO Port F in Run mode.
    peripherals.SYSCTL.rcgcgpio.modify(|_, w| w.r5().set_bit());

    // cortexinterrupt::free(|cs| MY_GPIO.borrow(cs).replace(Some(peripherals.GPIO_PORTF)));
    // cortexinterrupt::free(|cs| MY_TIMER1.borrow(cs).replace(Some(&peripherals.TIMER1)));

    // cortexinterrupt::free(|cs| {
    //     let mut my_gpio = MY_GPIO.borrow(cs).borrow_mut();
    //     my_gpio
    //         .as_mut()
    //         .unwrap()
    //         .dir
    //         .modify(|r, w| unsafe { w.bits(r.bits() | 1 << 2) });
    //     my_gpio
    //         .as_mut()
    //         .unwrap()
    //         .den
    //         .modify(|r, w| unsafe { w.bits(r.bits() | 1 << 2) });
    // });
    peripherals
        .GPIO_PORTF
        .dir
        .modify(|r, w| unsafe { w.bits(r.bits() | 1 << 3) });
    peripherals
        .GPIO_PORTF
        .den
        .modify(|r, w| unsafe { w.bits(r.bits() | 1 << 3) });

    // Timer1A 1 second delay configuration
    peripherals.SYSCTL.rcgctimer.modify(|_, w| w.r1().set_bit());
    // SYSCTL->RCGCTIMER |= (1<<1);  /*enable clock Timer1 subtimer A in run mode */
    peripherals.TIMER1.ctl.write(|w| w.taen().clear_bit());
    // TIMER1->CTL = 0; /* disable timer1 output */
    peripherals
        .TIMER1
        .cfg
        .write(|w| unsafe { w.cfg().bits(0x4) });
    // TIMER1->CFG = 0x4; /*select 16-bit configuration option */
    peripherals.TIMER1.tamr.write(|w| w.tamr().period());
    // TIMER1->TAMR = 0x02; /*select periodic down counter mode of timer1 */
    peripherals
        .TIMER1
        .tapr
        .write(|w| unsafe { w.bits(250 - 1) });
    // TIMER1->TAPR = 250-1; /* TimerA prescaler value */
    peripherals
        .TIMER1
        .tailr
        .write(|w| unsafe { w.bits(64000 - 1) });
    // TIMER1->TAILR = 64000-1 ; /* TimerA counter starting count down value  */
    peripherals.TIMER1.icr.write(|w| w.tatocint().set_bit());
    // TIMER1->ICR = 0x1;          /* TimerA timeout flag bit clears*/
    peripherals.TIMER1.imr.modify(|_, w| w.tatoim().set_bit());
    // TIMER1->IMR |=(1<<0); /*enables TimerA time-out  interrupt mask */
    peripherals.TIMER1.ctl.modify(|_, w| w.taen().set_bit());
    // TIMER1->CTL |= 0x01;        /* Enable TimerA module */
    // cperipherals.NVIC.request(interrupt::TIMER1A);
    unsafe {
        cperipherals.NVIC.iser[0].modify(|w| w | 1 << 21);
    }
    // NVIC->ISER[0] |= (1<<21); /*enable IRQ21 */
    cortexinterrupt::free(|cs| G_PERIPH.borrow(cs).replace(Some(peripherals)));
    // if peripherals.TIMER1.mis.read().bits() & 0x01 > 0 {
    //     peripherals
    //         .GPIO_PORTF
    //         .data
    //         .modify(|r, w| unsafe { w.bits(r.bits() ^ 1 << 2) });
    //     peripherals.TIMER1.icr.write(|w| w.tatocint().set_bit());
    // }
    loop {
        // your code goes here
    }
}

#[interrupt]
fn TIMER1A() {
    // https://docs.rust-embedded.org/book/concurrency/#sharing-peripherals
    // Your interrupt handler
    cortexinterrupt::free(|cs| {
        if let Some(ref mut p) = G_PERIPH.borrow(cs).borrow_mut().deref_mut() {
            if p.TIMER1.mis.read().bits() & 0x01 > 0 {
                p.GPIO_PORTF
                    .data
                    .modify(|r, w| unsafe { w.bits(r.bits() ^ 1 << 3) });
                p.TIMER1.icr.write(|w| w.tatocint().set_bit());
            }
        }
    });
}
