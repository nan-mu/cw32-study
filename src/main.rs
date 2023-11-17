#![no_std]
#![no_main]
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cw32f030_hal as hal;

#[entry]
fn main() -> ! {
    let dp = hal::svd::Peripherals::take().unwrap();
    //dp.SYSCTRL.hsi.write(|w| w.div().variant(0b0101));
    dp.SYSCTRL.ahben.write(|w| w.gpioc().bit(true)); //打开ahb桥上时钟，gpioc在ahb上，1为打开
    dp.GPIOC.lock.write(|w| w.pin13().bit(false)); //打开gpioc寄存器操作锁，1为锁，0为开
    dp.GPIOC.analog.write(|w| w.pin13().bit(false)); //设置pc13为数字，0为数字，1为模拟
    dp.GPIOC.dir.write(|w| w.pin13().bit(false)); //设置pc13为输出，0为输出，1为输入
    dp.GPIOC.opendrain.write(|w| w.pin13().bit(false)); //设置pc13输出模式，0为推挽输出，1为开漏输出
    dp.GPIOC.driver.write(|w| w.pin13().bit(false)); //设置pc13输出驱动能力。0为高驱动，1为低驱动
    dp.GPIOC.speed.write(|w| w.pin13().bit(true)); //设置pc13输出速度，0为低速，1为高速
    dp.GPIOC.odr().write(|w| w.pin13().bit(false)); //设置pc13输出电平，0为低，1为高
    loop {}
}
