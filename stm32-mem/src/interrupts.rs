#![allow(dead_code)]

use main;
use ::stm32::interrupts::IVTable;

pub fn interrupt() {
	loop {}
}

#[link_section = ".ivtable"]
#[used]
static IVTABLE: IVTable = IVTable { 
	main: main,
	nmi_handler: interrupt,
	hard_fault: interrupt,
    memmanage: interrupt,
	bus_fault: interrupt,
	usage_fault: interrupt,
    reserved1: 0,
    svcall: interrupt,
    debugmonitor: interrupt,
    reserved2: 0,
    pendsv: interrupt,
    systick: interrupt,

    wwdg: interrupt,
    pvd: interrupt,
    tamper: interrupt,
    rtc: interrupt,
    flash: interrupt,

    rcc: interrupt,
    exti0: interrupt,
    exti1: interrupt,
    exti2: interrupt,
    exti3: interrupt,

    exti4: interrupt,
    dma1_channel1: interrupt,
    dma1_channel2: interrupt,
    dma1_channel3: interrupt,
    dma1_channel4: interrupt,

    dma1_channel5: interrupt,
    dma1_channel6: interrupt,
    dma1_channel7: interrupt,
    adc1_2: interrupt,
    can1_tx: interrupt,

    can1_rx0: interrupt,
    can1_rx1: interrupt,
    can1_sce: interrupt,
    exti9_5: interrupt,
    tim1_brk: interrupt,

    tim1_up: interrupt,
    tim1_trg_com: interrupt,
    tim1_cc: interrupt,
    tim2: interrupt,
    tim3: interrupt,

    tim4: interrupt,
    i2c1_ev: interrupt,
    i2c1_er: interrupt,
    i2c2_ev: interrupt,
    i2c2_er: interrupt,

    spi: interrupt,
    spi2: interrupt,
    usart1: interrupt,
    usart2: interrupt,
    usart3: interrupt,

    exti15_10: interrupt,
    rtcalarm: interrupt,
    otg_fs_wkp: interrupt,
    reserved3: 0,
    reserved4: 0,

    reserved5: 0,
    reserved6: 0,
    reserved7: 0,
    reserved8: 0,
    reserved9: 0,

    tim5: interrupt,
    spi3: interrupt,
    uart4: interrupt,
    uart5: interrupt,
    tim6: interrupt,

    tim7: interrupt,
    dma2_channel1: interrupt,
    dma2_channel2: interrupt,
    dma2_channel3: interrupt,
    dma2_channel4: interrupt,

    dma2_channel5: interrupt,
    eth: interrupt,
    eth_wkup: interrupt,
    can2_tx: interrupt,
    can2_rx0: interrupt,

    can2_rx1: interrupt,
    can2_sce: interrupt,
    otg_fs: interrupt,
};

