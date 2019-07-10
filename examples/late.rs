//! examples/late.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use heapless::{
    consts::*,
    spsc::{Consumer, Producer, Queue},
};
use lm3s6965::Interrupt;
use panic_semihosting as _;

#[rtfm::app(device = lm3s6965)]
const APP: () = {
    // Late resources
    struct Resources {
        p: Producer<'static, u32, U4>,
        c: Consumer<'static, u32, U4>,
    }

    #[init]
    fn init(_: init::Context) -> init::LateResources {
        // NOTE: we use `Option` here to work around the lack of
        // a stable `const` constructor
        static mut Q: Option<Queue<u32, U4>> = None;

        *Q = Some(Queue::new());
        let (p, c) = Q.as_mut().unwrap().split();

        // Initialization of late resources
        init::LateResources { p, c }
    }

    #[idle(resources = [c])]
    fn idle(c: idle::Context) -> ! {
        loop {
            if let Some(byte) = c.resources.c.dequeue() {
                hprintln!("received message: {}", byte).unwrap();

                debug::exit(debug::EXIT_SUCCESS);
            } else {
                rtfm::pend(Interrupt::UART0);
            }
        }
    }

    #[task(binds = UART0, resources = [p])]
    fn uart0(c: uart0::Context) {
        c.resources.p.enqueue(42).unwrap();
    }
};
