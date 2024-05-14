#![no_std]
#![no_main]
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use reg::{ahben, analog, dir, driver, lckr, odr, opendrain, speed};
use tock_registers::interfaces::Writeable;

mod reg;

#[entry]
fn main() -> ! {
    // sysctrl 基地址0x4001 0000
    // gpioc 基地址0x4800 0800
    let sysctrl: &reg::Sysctrl = unsafe { &*(0x4001_0000 as *const _) }; // 直接从地址转换为寄存器组的控制对象
    sysctrl.ahben.write(ahben::GPIOC::enable); //启用这个ahben的锁
    let gpioc: &reg::Gpioc = unsafe { &*(0x4800_0800 as *const _) }; //生成gpioc寄存器组的控制对象
    gpioc.lckr.write(lckr::KEYPIN13::unlock); //往lckr写入0x5A5AFFFF
    gpioc.analog.write(analog::PIN13::digital); // 往analog的13位写入0，剩下依此类推
    gpioc.dir.write(dir::PIN13::output);
    gpioc.opendrain.write(opendrain::PIN13::push_pull);
    gpioc.driver.write(driver::PIN13::high);
    gpioc.speed.write(speed::PIN13::low);
    gpioc.odr.write(odr::PIN13::low);

    loop {}
}
