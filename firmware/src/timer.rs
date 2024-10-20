use core::cell::Cell;

use critical_section::Mutex;
use embassy_time_driver::{AlarmHandle, Driver, TICK_HZ};

// Ratio between the selected tick rate and the clock rate (25Mhz)
const TICK_RATIO: u64 = 25_000_000 / TICK_HZ;

static ALARAM_ALLOCATED: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static ALARAM_CALLBACK: Mutex<Callback> = Mutex::new(Callback::new());

type CallbackAndCtx = (fn(*mut ()), *mut ());

struct Callback(Cell<Option<CallbackAndCtx>>);

unsafe impl Send for Callback {}

impl Callback {
    const fn new() -> Self {
        Callback(Cell::new(None))
    }
}

struct Sg2000TimerDriver {}

impl Driver for Sg2000TimerDriver {
    fn now(&self) -> u64 {
        riscv::register::time::read64() / TICK_RATIO
    }
    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle> {
        critical_section::with(|cs| {
            let alarm_allocated = ALARAM_ALLOCATED.borrow(cs);
            if alarm_allocated.get() {
                None
            } else {
                alarm_allocated.set(true);
                Some(AlarmHandle::new(0))
            }
        })
    }
    fn set_alarm_callback(&self, _alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ()) {
        critical_section::with(|cs| ALARAM_CALLBACK.borrow(cs).0.set(Some((callback, ctx))))
    }
    fn set_alarm(&self, _alarm: AlarmHandle, timestamp: u64) -> bool {
        critical_section::with(|_| unsafe {
            if timestamp <= (self.now() + 1) {
                return false;
            }

            let timestamp = timestamp * TICK_RATIO;
            let mtimecmp = sg2000_pac::Mtimecmp::steal();
            mtimecmp
                .mtimecmp_high()
                .write(|w| w.bits((timestamp >> 32) as u32));
            mtimecmp.mtimecmp_low().write(|w| w.bits(timestamp as u32));

            true
        })
    }
}

#[no_mangle]
pub extern "C" fn MachineTimer() {
    critical_section::with(|cs| {
        unsafe {
            // "clear" the interrupt by settin mtimecmp to the maximal value
            let mtimecmp = sg2000_pac::Mtimecmp::steal();
            mtimecmp.mtimecmp_high().write(|w| w.bits(u32::MAX));
            mtimecmp.mtimecmp_low().write(|w| w.bits(u32::MAX));
        }
        let callback = ALARAM_CALLBACK.borrow(cs).0.get();
        if let Some((callback, ctx)) = callback {
            callback(ctx)
        }
    })
}

embassy_time_driver::time_driver_impl!(static DRIVER: Sg2000TimerDriver = Sg2000TimerDriver{});
