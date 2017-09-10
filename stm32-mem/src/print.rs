#![allow(dead_code)]

use ::stm32;

pub fn enable_uart() {
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


pub fn uart_tx(ch : u8) {
    while !stm32::USART_1().get_transmit_empty() {};
    stm32::USART_1().send_data(ch);
}

pub fn print_int (i : u32) {
    if i == 0 { uart_tx('0' as u8); return; };

    let mut i = i;
    let mut s = [0 as u8; 10];
    let mut j = 0;
    while i != 0 {
        let rem = (i % 10) as u8;
        s[j] = '0' as u8 + rem;
        j += 1;
        i = i / 10;
    }

    for x in 0..j {
        uart_tx(s[j-x-1]);
    }
}

pub fn print_hex_digit(i : u8) {
    let d = if i > 9 {
        'A' as u8 + (i - 10)
    } else {
        '0' as u8 + i
    };
    uart_tx(d);
}

pub fn print_hex (i : u32, bytes : u32) {
    for b in 0..bytes {
        let rem = i >> (bytes - b - 1) * 8;
        let byte = (rem & 0xFF) as u8;
        print_hex_digit((byte >> 4) & 0x0F);
        print_hex_digit(byte & 0x0F);
    }
}

pub fn print(s : &str) {
    for a in s.chars() {
        uart_tx(a as u8);
    }
}

pub fn println(s : &str) {
    print(s);
    uart_tx('\n' as u8);
}
