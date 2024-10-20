#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};

use embassy_executor::Spawner;
use embassy_time::Timer;
use riscv::asm::nop;
use sg2000_pac::{interrupt, Uart0};
use uart::UartWriter;
use xuantie::register::mhcr;

mod irq;
pub mod ressource_table;
mod timer;
mod uart;

const LED_MASK: u32 = 1 << 29;
const INPUT_MASK: u32 = 1 << 15;
const INPUT_IRQ_NO: usize = sg2000_pac::Interrupt::GPIO1 as usize;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    riscv::interrupt::disable();
    let uart0 = unsafe { Uart0::steal() };
    let mut uart_writer = UartWriter::new(&uart0, true);
    let _ = writeln!(uart_writer, "{}", info);

    loop {}
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    unsafe {
        // enable the I-cache, for faster execution
        mhcr::set_ie();
    }

    let perpipherals = sg2000_pac::Peripherals::take().unwrap();
    let gpio0 = perpipherals.gpio0;
    let gpio1 = perpipherals.gpio1;
    let plic = perpipherals.plic;

    // We assume the uart is already configured, which is the case with uart0, because it is the
    // one also used by the kernel.
    let uart0 = perpipherals.uart0;

    unsafe {
        // enable interrupt and the PLIC
        riscv::interrupt::enable();
        riscv::register::mie::set_mext();
        riscv::register::mie::set_mtimer();
        plic.priority_threshold(0).write(|w| w.bits(0));

        // set the led gpio as output
        gpio0.ddr().modify(|r, w| w.bits(r.bits() | LED_MASK));

        // Set the input irq as level, active high and enable it
        gpio1
            .int_polarity()
            .modify(|r, w| w.bits(r.bits() | INPUT_MASK));
        gpio1.inten().modify(|r, w| w.bits(r.bits() | INPUT_MASK));
        irq::enable_irq(&plic, INPUT_IRQ_NO);
    }

    spawner.spawn(print_hellos(uart0)).unwrap();

    loop {
        // toggle the led gpio
        unsafe { gpio0.dr().modify(|r, w| w.bits(r.bits() ^ LED_MASK)) };

        // wait for some time
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn print_hellos(uart0: Uart0) {
    let mut uart_writer = UartWriter::new(&uart0, true);

    loop {
        for i in 0..10 {
            writeln!(&mut uart_writer, "HELLO {i}").unwrap();
            Timer::after_secs(2).await
        }
    }
}

interrupt!(GPIO1, gpio1_handler);

fn gpio1_handler() {
    let perpipherals = unsafe { sg2000_pac::Peripherals::steal() };
    let gpio0 = perpipherals.gpio0;
    // toggle the led gpio
    unsafe { gpio0.dr().modify(|r, w| w.bits(r.bits() ^ LED_MASK)) };

    // wait for some time, but less than in main, this way the led blink faster
    for _ in 0..10000000 {
        nop();
    }
}
