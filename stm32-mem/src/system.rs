
use ::stm32;

pub fn init_clock() {
    use stm32::clock::*;
    use stm32::flash;

    let rcc = stm32::RCC();

    rcc.enable_hse();
    while !rcc.get_hse_ready() {}

    // use 8 MHz external oscillator
    // set PLL to 72 MHz (8 MHz * 9)
    rcc.set_pll_multiplier(PLLMultiplier::Mult9);
    rcc.set_pll_source(PLLSource::HighSpeedExternal);

    // set prescaler
    rcc.set_apb_1_prescaler(APBPrescaler::Div2);

    let flash = stm32::FLASH();
    flash.enable_prefetch();
    flash.set_latency(flash::FlashLatency::TwoWait);
    rcc.enable_pll();
    while !rcc.get_pll_ready() {}

    rcc.set_system_clock(SystemClockSwitch::PLLOutput);
}

pub fn enable_led() {
    use stm32::gpio;
    use stm32::clock::Peripheral;

    stm32::RCC().enable_peripheral(Peripheral::IOPortC);

    let portc = stm32::PORT_C();
    portc.set_port_pin_mode(gpio::Pin::P13, gpio::PortPinMode::OutputMode50MHz);
    portc.clear_port_pin_out(gpio::Pin::P13);
}
