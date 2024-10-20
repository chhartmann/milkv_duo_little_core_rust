use core::fmt;
use sg2000_pac::uart0;

// We use RegisterBlock instead of Uart0 to be generic across all uarts

pub fn putc_blocking(uart: &uart0::RegisterBlock, value: u8) {
    while !uart.lsr().read().transmitter_empty_bit().bit() {}
    unsafe { uart.rbr_thr_dll().write(|w| w.rbr_thr_dll().bits(value)) };
}

pub struct UartWriter<'a> {
    uart: &'a uart0::RegisterBlock,
    /// If true, add a \r before each \n
    add_cr: bool,
}

impl<'a> UartWriter<'a> {
    pub fn new(uart: &'a uart0::RegisterBlock, add_cr: bool) -> Self {
        UartWriter { uart, add_cr }
    }
}

impl<'a> fmt::Write for UartWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            if self.add_cr && b == b'\n' {
                putc_blocking(self.uart, b'\r');
            }
            putc_blocking(self.uart, b);
        }
        Ok(())
    }
}
