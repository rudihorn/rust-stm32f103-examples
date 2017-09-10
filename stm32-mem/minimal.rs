macro_rules! register_bank {
    ($name:ident, $priv:ident, $t:ty, $addr:expr) => (
        const $priv : *mut $t = $addr as *mut $t;
        #[allow(non_snake_case)]
        pub fn $name() -> &'static mut $t {
            return unsafe { &mut (*$priv) };
        }
    )
}

#[macro_export]
macro_rules! write_bit_fns {
    ($name_en:ident, $name_dis:ident, $reg:ident, $pin:expr) => (
        pub fn $name_en(&mut self) {
            unsafe { write_volatile(&mut(self.$reg), common::set(read_volatile(&(self.$reg)), $pin)) };
        }
        pub fn $name_dis(&mut self) {
            unsafe { write_volatile(&mut(self.$reg), common::clear(read_volatile(&(self.$reg)), $pin)) };
        }
    )
}

#[macro_export]
macro_rules! read_bit_fns {
    ($name:ident, $reg:ident, $pin:expr) => (
        pub fn $name(&mut self) -> bool {
            return unsafe { (read_volatile(&(self.$reg)) >> $pin) & 0b1 } > 0;
        }
    )
}

pub struct FlashRegisters {
    // 0x00 : Access Control Register
    pub acr: Register,

    // 0x04 : 
    pub keyr: Register,

    // 0x08 :
    pub optkeyr: Register,
}

pub enum FlashLatency {
    // 0 Hz < SYSCLK <= 24 MHz
    ZeroWait,
    // 24 MHz < SYSCLK <= 48 MHz
    OneWait,
    // 48 MHz < SYSCLK <= 72 MHz
    TwoWait,
}

impl FlashRegisters {
    pub fn set_latency(&mut self, lat : FlashLatency) {
        let lat = lat as u32;
        unsafe { write_volatile(&mut(self.acr), common::replace(self.acr, lat, 0b111, 0)) };
    }

    read_bit_fns!(get_prefetch_status, acr, 5);

    write_bit_fns!(enable_prefetch, disable_prefetch, acr, 4);
    write_bit_fns!(enable_half_cycle_access, disable_half_cycle_access, acr, 4);
}


// declare registers
register_bank!(FLASH, FLASH_PRIV, flash::FlashRegisters, 0x40022000);










// lets me write
    let flash = stm32::FLASH();
    flash.enable_prefetch();
