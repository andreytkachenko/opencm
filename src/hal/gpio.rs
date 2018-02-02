use core::ops::Deref;
use common::VolatileCell;

const GPIO_BASE: usize = 0x4002_0000;
const SIZE: usize = 0x00000400;

pub const GPIOA: Gpio = Gpio { regs: (GPIO_BASE + SIZE * 0) as *const Registers };
pub const GPIOB: Gpio = Gpio { regs: (GPIO_BASE + SIZE * 1) as *const Registers };
pub const GPIOC: Gpio = Gpio { regs: (GPIO_BASE + SIZE * 2) as *const Registers };
pub const GPIOD: Gpio = Gpio { regs: (GPIO_BASE + SIZE * 3) as *const Registers };
pub const GPIOE: Gpio = Gpio { regs: (GPIO_BASE + SIZE * 4) as *const Registers };
pub const GPIOF: Gpio = Gpio { regs: (GPIO_BASE + SIZE * 5) as *const Registers };
pub const GPIOG: Gpio = Gpio { regs: (GPIO_BASE + SIZE * 6) as *const Registers };
pub const GPIOH: Gpio = Gpio { regs: (GPIO_BASE + SIZE * 7) as *const Registers };
pub const GPIOI: Gpio = Gpio { regs: (GPIO_BASE + SIZE * 8) as *const Registers };
pub const GPIOJ: Gpio = Gpio { regs: (GPIO_BASE + SIZE * 9) as *const Registers };
pub const GPIOK: Gpio = Gpio { regs: (GPIO_BASE + SIZE * 10) as *const Registers };

pub fn get_port_by_index(index: u16) -> Gpio {
    match index {
        0 => GPIOA,
        1 => GPIOB,
        2 => GPIOC,
        3 => GPIOD,
        4 => GPIOE,
        5 => GPIOF,
        6 => GPIOG,
        7 => GPIOH,
        8 => GPIOI,
        9 => GPIOJ,
        10 => GPIOK,
        _ => panic!("Port not found!")
    }
}

mod flags {
    pub const LCKK: u32 = 1 << 16;
}

pub mod function {
    use super::PeripheralFunction as PF;

    pub const RTC_50HZ: PF = PF::AF0;
    pub const MCO: PF   = PF::AF0;
    pub const TAMPER: PF = PF::AF0;
    pub const SWJ: PF   = PF::AF0;
    pub const TRACE: PF = PF::AF0;
    pub const TIM1: PF  = PF::AF1;
    pub const TIM2: PF  = PF::AF1;
    pub const TIM3: PF  = PF::AF2;
    pub const TIM4: PF  = PF::AF2;
    pub const TIM5: PF  = PF::AF2;
    pub const TIM8: PF  = PF::AF3;
    pub const TIM9: PF  = PF::AF3;
    pub const TIM10: PF = PF::AF3;
    pub const TIM11: PF = PF::AF3;
    pub const I2C1: PF  = PF::AF4;
    pub const I2C2: PF  = PF::AF4;
    pub const I2C3: PF  = PF::AF4;
    pub const SPI1: PF  = PF::AF5;
    pub const SPI2: PF  = PF::AF5;
    pub const SPI3: PF  = PF::AF6;
    pub const USART1: PF = PF::AF7;
    pub const USART2: PF = PF::AF7;
    pub const USART3: PF = PF::AF7;
    pub const I2S3_EXT: PF = PF::AF7;
    pub const UART4: PF = PF::AF8;
    pub const UART5: PF = PF::AF8;
    pub const USART6: PF = PF::AF8;
    pub const CAN1: PF  = PF::AF9;
    pub const CAN2: PF  = PF::AF9;
    pub const TIM12: PF = PF::AF9;
    pub const TIM13: PF = PF::AF9;
    pub const TIM14: PF = PF::AF9;
    pub const OTG_FS: PF = PF::AF10;
    pub const OTG_HS: PF = PF::AF10;
    pub const ETH: PF    = PF::AF11;
    pub const FMC: PF    = PF::AF12;
    pub const FSMC: PF   = PF::AF12;
    pub const OTG_HS_FS: PF = PF::AF12;
    pub const SDIO: PF   = PF::AF12;
    pub const DCMI: PF   = PF::AF13;
    pub const EVENTOUT: PF   = PF::AF15;
}

#[repr(C)]
pub struct Registers {
    pub moder:   VolatileCell<u32>,
    pub otyper:  VolatileCell<u32>,
    pub ospeedr: VolatileCell<u32>,
    pub pupdr:   VolatileCell<u32>,
    pub idr:     VolatileCell<u32>,
    pub odr:     VolatileCell<u32>,
    pub bsr:     VolatileCell<u16>,
    pub brr:     VolatileCell<u16>,
    pub lckr:    VolatileCell<u32>,
    pub afrl:    VolatileCell<u32>,
    pub afrh:    VolatileCell<u32>
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Gpio {
    regs: *const Registers
}

impl Gpio {
    const fn new(addr: usize) -> Gpio {
        Gpio {
            regs: addr as *const Registers
        }
    }
}

impl Deref for Gpio {
    type Target = Registers;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.regs }
    }
}

impl Gpio {
    pub fn set_high(&self, pins: u16) {
        self.bsr.check(pins);
    }

    pub fn set_low(&self, pins: u16) {
        self.brr.check(pins);
    }

    pub fn toggle(&self, pins: u16) {
        let port = self.odr.get() as u16;

        self.bsr.check(!port & pins);
        self.brr.check(port & pins);
    }

    pub fn write(&self, value: u16) {
        self.odr.set(value as u32);
    }

    pub fn read(&self) -> u16 {
        self.idr.get() as u16
    }

    pub fn read_pins(&self, pins: u16) -> u16 {
        (self.idr.get() as u16) & pins
    }

    pub fn read_pin(&self, pin: u16) -> bool {
        (self.idr.get() as u16) & (1u16 << pin) != 0
    }

    pub fn lock(&self, pins: u16) {
        let mut reg32;

        /* Special "Lock Key Writing Sequence", see datasheet. */
        self.lckr.set(flags::LCKK | pins as u32);	/* Set LCKK. */
        self.lckr.set(!flags::LCKK & pins as u32);	/* Clear LCKK. */
        self.lckr.set(flags::LCKK | pins as u32);	/* Set LCKK. */

        reg32 = self.lckr.get();			/* Read LCKK. */
        reg32 = self.lckr.get();			/* Read LCKK again. */

        /* Tell the compiler the variable is actually used. It will get
        * optimized out anyways.
        */

        reg32 = reg32;
    }

    pub fn set_pin_output_speed(&self, pin: u16, speed: OutputSpeed) {
        self.ospeedr.mask_set(0b11, (pin as u32) << 1, speed as u32);
    }

    pub fn set_pin_output_type(&self, pin: u16, mode: OutputType) {
        match mode {
            OutputType::PushPull => self.otyper.uncheck(pin as u32),
            OutputType::OpenDrain => self.otyper.check(pin as u32)
        };
    }

    pub fn set_pin_mode(&self, pin: u16, mode: PinMode) {
        self.moder.mask_set(0b11, (pin as u32) << 1, mode as u32);
    }

    pub fn set_pin_pupd(&self, pin: u16, pupd: PuPdMode) {
        self.pupdr.mask_set(0b11, (pin as u32) << 1, pupd as u32);
    }

    pub fn set_pin_af(&self, pin: u16, af: PeripheralFunction) {
        let reg = if pin > 7 {
            &self.afrh
        } else {
            &self.afrl
        };

        reg.mask_set(0b1111, pin as u32 % 8, af as u32);
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PuPdMode {
    None = 0x0,
    Up   = 0x1,
    Down = 0x2,
}

#[derive(Debug, Copy, Clone)]
pub enum PinMode {
    Input  = 0x0,
    Output = 0x1,
    AF     = 0x2,
    Analog = 0x3
}

#[derive(Debug, Copy, Clone)]
pub enum OutputSpeed {
    Low,
    Medium,
    Fast,
    High
}

#[derive(Debug, Copy, Clone)]
pub enum OutputType {
    PushPull,
    OpenDrain
}

#[derive(Copy,Clone)]
pub enum PeripheralFunction {
    AF0  = 0x0, // RTC_50Hz; MCO; TAMPER; SWJ; TRACE
    AF1  = 0x1, // TIM1; TIM2
    AF2  = 0x2, // TIM3; TIM4; TIM5
    AF3  = 0x3, // TIM8; TIM9; TIM10; TIM11
    AF4  = 0x4, // I2C1; I2C2; I2C3
    AF5  = 0x5, // SPI1; SPI2
    AF6  = 0x6, // SPI3
    AF7  = 0x7, // USART1; USART2; USART3; I2S3ext
    AF8  = 0x8, // UART4; UART5; USART6
    AF9  = 0x9, // CAN1; CAN2; TIM12; TIM13; TIM14
    AF10 = 0xa, // OTG_FS; OTG_HS
    AF11 = 0xb, // ETH
    AF12 = 0xc, // FSMC; OTG_HS_FS; SDIO
    AF13 = 0xd, // DCMI
    AF14 = 0xe, //
    AF15 = 0xf  // EVENTOUT
}