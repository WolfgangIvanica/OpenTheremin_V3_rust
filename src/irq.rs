
// local imports
use crate::sintable1;

// IRQ Variables
static mut G_TABLEPOS: u16 = 0;
static mut G_TABLEINC: u16 = 0;
static mut G_VOLUME: u16 = 0;

pub fn set_tableinc(inc: u16) {
    unsafe { G_TABLEINC = inc };
}

pub fn set_volume(vol: u16) {
    unsafe { G_VOLUME = vol };
}

#[avr_device::interrupt(atmega328p)]
fn INT1() {
    let ispi: arduino_uno::pac::SPI = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    let iexti: arduino_uno::pac::EXINT = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    let idac1cs: arduino_uno::pac::PORTB =
        unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    let ilatch: arduino_uno::pac::PORTD = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

    iexti.eimsk.write(|w| w.int1().clear_bit());
    //avr_device::interrupt::disable();

    unsafe { G_TABLEPOS = G_TABLEPOS.wrapping_add(G_TABLEINC) };

    let tp = unsafe { G_TABLEPOS/128 } as usize;
    let vol = unsafe {G_VOLUME} as i16;
    let dacval = sintable1::SINE_TABLE[tp] as i32;

    let scaledsample = (((unsafe {dacval.unchecked_mul(vol.into())}) / 256) + 2048) as u16;
    let hb: u8 = (((scaledsample >> 8) & 0x0F) | 0x70) as u8;
    let lb: u8 = ((scaledsample >> 0) & 0xFF) as u8;

    unsafe {
        ilatch.portd.write(|w| w.pd7().clear_bit()); // D7 = latch
        ilatch.portd.write(|w| w.pd7().set_bit());
        idac1cs.portb.write(|w| w.pb2().clear_bit()); //B2=CS
        ispi.spdr.write(|w| w.bits(hb));
        while ispi.spsr.read().spif().bit_is_clear() {}
        ispi.spdr.write(|w| w.bits(lb));
        while ispi.spsr.read().spif().bit_is_clear() {}
        idac1cs.portb.write(|w| w.pb2().set_bit())
    };
    //unsafe { avr_device::interrupt::enable(); }
    iexti.eimsk.write(|w| w.int1().set_bit());
}
