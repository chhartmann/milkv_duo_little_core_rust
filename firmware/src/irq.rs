use sg2000_pac::{Plic, Vector, __EXTERNAL_INTERRUPTS};

const DEFAULT_PRIORITY: u32 = 7;
const HART_NO: usize = 0;

#[no_mangle]
pub extern "C" fn MachineExternal() {
    let plic = unsafe { sg2000_pac::Plic::steal() };
    loop {
        let irq_no = plic.claim(HART_NO).read().bits();
        if irq_no == 0 {
            break;
        }

        if (irq_no as usize) < __EXTERNAL_INTERRUPTS.len() {
            unsafe {
                match &__EXTERNAL_INTERRUPTS[irq_no as usize] {
                    Vector { _reserved: 0 } => {}
                    Vector { _handler } => {
                        _handler();
                    }
                }
            }
        }

        unsafe {
            plic.claim(HART_NO).write(|w| w.bits(irq_no));
        }
    }
}

pub fn enable_irq(plic: &Plic, irq_no: usize) {
    unsafe {
        plic.priority(irq_no).write(|w| w.bits(DEFAULT_PRIORITY));
        plic.enable(HART_NO)
            .bits(irq_no / 32)
            .modify(|r, w| w.bits(r.bits() | (1 << (irq_no % 32))));
    }
}
