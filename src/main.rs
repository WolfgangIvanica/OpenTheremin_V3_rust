#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(llvm_asm)]

use arduino_uno::{adc, spi, spi::Settings, prelude::*};
use embedded_hal;
//use panic_halt as _;

// local imports
mod irq;
mod sintable1;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut serial: arduino_uno::Serial<arduino_uno::hal::port::mode::Floating> =
        unsafe { core::mem::MaybeUninit::uninit().assume_init() };

    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").void_unwrap();

    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}\r",
            loc.file(),
            loc.line(),
            loc.column(),
        )
        .void_unwrap();
    }

    loop {}
}

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // LED defines
    let mut led_play = pins.a4.into_output(&mut pins.ddr);
    let mut led_standby = pins.a5.into_output(&mut pins.ddr);
    led_standby.set_low().void_unwrap();
    led_play.set_low().void_unwrap();

    // function button
    let function_button = pins.d6.into_pull_up_input(&mut pins.ddr);

    // ADC
    let mut adc = adc::Adc::new(dp.ADC, Default::default());
    let mut poti_pitch = pins.a0.into_analog_input(&mut adc);
    let mut poti_volume = pins.a1.into_analog_input(&mut adc);
    let mut poti_waveform = pins.a2.into_analog_input(&mut adc);
    let mut poti_timbre = pins.a3.into_analog_input(&mut adc);

    // Serial
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );

    // SPI
    let (_, _) = spi::Spi::new(
        dp.SPI,
        pins.d13.into_output(&mut pins.ddr),
        pins.d11.into_output(&mut pins.ddr),
        pins.d12.into_pull_up_input(&mut pins.ddr),
        pins.d10.into_output(&mut pins.ddr),
        Settings {
            data_order: spi::DataOrder::MostSignificantFirst,
            clock: spi::SerialClockRate::OscfOver2,
            mode: embedded_hal::spi::Mode {
                polarity: embedded_hal::spi::Polarity::IdleLow,
                phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
            },
        }
    );

    // Latch Pin DAC
    let mut _dac_latch = pins.d7.into_output(&mut pins.ddr);


    // INT1
    let ei = dp.EXINT;
    ei.eicra.write(|w| w.isc1().val_0x03());
    ei.eimsk.write(|w| w.int().bits(0x00));
    unsafe {
        avr_device::interrupt::enable(); // Enable interrupts
    }

    // Hello
    ufmt::uwriteln!(&mut serial, "Hello OpenTheremin!\r").void_unwrap();

    // Start muted
    irq::set_volume(25);
    irq::set_tableinc(549); // MIDDLE_C = 261.6 * HZ_FAC = 2.09785
    
    let mut btn_pressed: bool = false;
    loop {
        if function_button.is_low().void_unwrap() {
            if !btn_pressed {
                btn_pressed = true;
                let value_pitch: u16 = nb::block!(adc.read(&mut poti_pitch)).void_unwrap();
                let value_volume: u16 = nb::block!(adc.read(&mut poti_volume)).void_unwrap();
                let value_waveform: u16 = nb::block!(adc.read(&mut poti_waveform)).void_unwrap();
                let value_timbre: u16 = nb::block!(adc.read(&mut poti_timbre)).void_unwrap();
                ufmt::uwriteln!(&mut serial, "Pitch   : {}\r", value_pitch).void_unwrap();
                ufmt::uwriteln!(&mut serial, "Volume  : {}\r", value_volume).void_unwrap();
                ufmt::uwriteln!(&mut serial, "Waveform: {}\r", value_waveform).void_unwrap();
                ufmt::uwriteln!(&mut serial, "Timbre  : {}\r", value_timbre).void_unwrap();
            }
        } else {
            if btn_pressed {
                btn_pressed = false;
                let tv = irq::get_tablepos();
                ei.eimsk.write(|w| w.int().bits(0x02));
                ufmt::uwriteln!(&mut serial, "Tableval: {}\r", tv).void_unwrap();
            }
        }
        led_standby.toggle().void_unwrap();
        arduino_uno::delay_ms(200);
    }
}
