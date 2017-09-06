#![no_std]
#![no_main]
#![feature(asm, lang_items, start, naked_functions, used)]


mod std_repl;
mod interrupts;
extern crate stm32;


fn init_clock() {
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

fn enable_led() {
    use stm32::gpio;
    use stm32::clock::Peripheral;

    stm32::RCC().enable_peripheral(Peripheral::IOPortC);

    let portc = stm32::PORT_C();
    portc.set_port_pin_mode(gpio::Pin::P13, gpio::PortPinMode::OutputMode50MHz);
    portc.clear_port_pin_out(gpio::Pin::P13);
}

fn enable_uart() {
    use stm32::gpio::*;
    use stm32::clock::Peripheral;

    let rcc = stm32::RCC();
    rcc.enable_peripheral(Peripheral::USART);

    // setup USART
    let usart1 = stm32::USART_1();
    usart1.set_baud(468, 12);
    //usart1.set_baud_calc(72000000, 9600);
    usart1.enable_transmitter();
    usart1.enable_receiver();

    usart1.enable();
    
    rcc.enable_peripheral(Peripheral::AlternateFunctionIO);
    rcc.enable_peripheral(Peripheral::IOPortA);

    let porta = stm32::PORT_A();
    // setup output potr
    porta.set_port_pin_mode(Pin::P9, PortPinMode::OutputMode50MHz);
    porta.set_port_pin_config(Pin::P9, PortPinConfig::AlternateFunctionOutputPushPull);
    // setup input port
    porta.set_port_pin_config(Pin::P10, PortPinConfig::FloatingInput);  
}

fn uart_tx(ch : u8) {
    while stm32::USART_1().get_read_not_empty() {};
    stm32::USART_1().send_data(ch);
}

pub fn main() {
    init_clock();

    enable_led();

    enable_uart();
    
    let s = "STM32 Serial App";

    let mut v = 'A' as u8;
    loop {
        uart_tx(v);
        /* v = v + 1;
        if v > ('Z' as u8) { v = ('A' as u8); } */
    } 
}

