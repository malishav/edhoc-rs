#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::debug::{self, EXIT_SUCCESS};
use defmt::info;
use defmt_rtt as _;
pub use nrf54l15_app_pac as pac;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    info!("Running.");

    let p = pac::Peripherals::take().unwrap();
    let cracen = p.global_cracen_s;
    let cracen_core = p.global_cracencore_s;

    p.global_p2_s.pin_cnf(9).write(|w| w.dir().output());
    p.global_p2_s.outset().write(|w| w.pin9().set_bit());

    cracen.enable().write(|w| w.pkeikg().set_bit());

    while cracen_core.pk().status().read().pkbusy().bit_is_set() {}

    info!("Cracen PKA engine ready!");

    cracen_core
        .pk()
        .pointers()
        .write(|w| unsafe { w.opptra().bits(0x00).opptrb().bits(0x01) });
    cracen_core.pk().command().write(|w| unsafe {
        w.opeaddr()
            .bits(0x01)
            .opbytesm1()
            .bits(0x1f)
            .selcurve()
            .bits(0x1)
    });
    cracen_core.pk().opsize().write(|w| w.opsize().opsize256());

    cracen_core.pk().control().write(|w| w.start().set_bit());

    while cracen_core.pk().status().read().pkbusy().bit_is_set() {}

    let res = cracen_core.pk().status().read().bits();
    let hw_config = cracen_core.pk().hwconfig().read().bits();

    info!("PKA status: {:x} HW config: {:x}", res, hw_config);

    // exit via semihosting call
    debug::exit(EXIT_SUCCESS);
    loop {}
}
