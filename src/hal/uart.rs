use core::ops::Deref;
use common::VolatileCell;
use common::asm;

use hal::rcc;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Uart {
    regs: *const Registers
}

impl Uart {
    const fn new(addr: usize) -> Uart {
        Uart {
            regs: addr as *const Registers
        }
    }
}

impl Deref for Uart {
    type Target = Registers;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.regs }
    }
}

pub const USART1: Uart = Uart::new(0x40011000);
pub const USART2: Uart = Uart::new(0x40004400);
pub const USART3: Uart = Uart::new(0x40004800);
pub const UART4:  Uart = Uart::new(0x40004c00);
pub const UART5:  Uart = Uart::new(0x40005000);
pub const USART6: Uart = Uart::new(0x40011400);
pub const UART7:  Uart = Uart::new(0x40007800);
pub const UART8:  Uart = Uart::new(0x40007c00);

#[repr(C)]
pub struct Registers {
    pub sr:  VolatileCell<u32>,
    pub dr:  VolatileCell<u32>,
    pub brr: VolatileCell<u32>,
    pub cr1: VolatileCell<u32>,
    pub cr2: VolatileCell<u32>,
    pub cr3: VolatileCell<u32>,
    pub gtpr: VolatileCell<u32>,
}

pub fn enable(dev: Uart, baud_rate: u32, stop_bits: StopBits, parity: Parity, flow_control: bool) {   
    set_baudrate(dev, baud_rate);
    set_parity(dev, parity);
    set_stopbits(dev, stop_bits);
    set_flow_control(dev, if flow_control {FlowControl::RtsAndCts} else {FlowControl::None});
    enable_interrupts(dev);
    enable_error_interrupts(dev);
    set_databits(dev, 8);
    set_mode(dev, Mode::Duplex);

    dev.cr1.check(flags::cr1::UE);
}

fn set_baudrate(dev: Uart, baud: u32) {
    let clock = if dev == USART1 || dev == USART6 {
        rcc::apb2_frequency()
    } else {
        rcc::apb1_frequency()
    };

    dev.brr.set(((clock << 1) + baud) / (baud << 1));
}

fn set_databits(dev: Uart, bits: u32) {
    if bits == 8 {
        dev.cr1.uncheck(flags::cr1::M); /* 8 data bits */
    } else {
        dev.cr1.check(flags::cr1::M); /* 9 data bits */
    }
}

fn set_stopbits(dev: Uart, stopbits: StopBits) {
    dev.cr2.mask_set(flags::cr2::STOPBITS_MASK, flags::cr2::STOPBITS_SHIFT, stopbits as u32);
}

fn set_parity(dev: Uart, parity: Parity) {
    dev.cr2.mask_set(flags::cr2::PARITY_MASK, flags::cr2::PARITY_SHIFT, parity as u32);
}

fn set_mode(dev: Uart, mode: Mode) {
    dev.cr1.mask_set(flags::cr1::MODE_MASK, flags::cr1::MODE_SHIFT, mode as u32);
}

fn set_flow_control(dev: Uart, flowcontrol: FlowControl) {
    dev.cr3.mask_set(flags::cr3::FLOWCONTROL_MASK, flags::cr3::FLOWCONTROL_SHIFT, flowcontrol as u32);
}

fn disable(dev: Uart) {
    dev.cr1.uncheck(flags::cr1::UE);
}

fn send(dev: Uart, data: u16) {
    dev.dr.set((data as u32) & flags::dr::MASK);
}

fn recv(dev: Uart) -> u16 {
    (dev.dr.get() & flags::dr::MASK) as u16
}

fn wait_send_ready(dev: Uart) {
    while !dev.sr.test(flags::sr::TXE) { asm::nop(); };
}

fn wait_recv_ready(dev: Uart) {
    while !dev.sr.test(flags::sr::RXNE) { asm::nop(); };
}

fn send_blocking(dev: Uart, data: u16) {
    wait_send_ready(dev);
    send(dev, data);
}

fn recv_blocking(dev: Uart) -> u16 {
    wait_recv_ready(dev);
    recv(dev)
}

fn enable_interrupts(dev: Uart) {
    dev.cr1.check(flags::cr1::RXNEIE | flags::cr1::TCIE);
}

fn disable_interrupts(dev: Uart) {
    dev.cr1.uncheck(flags::cr1::RXNEIE | flags::cr1::TCIE);
}

fn enable_error_interrupts(dev: Uart) {
    // dev.cr1.check(flags::cr1::PEIE);
    // dev.cr2.check(flags::cr2::LBDIE);
    dev.cr3.check(flags::cr3::EIE);
}

fn disable_error_interrupts(dev: Uart) {
    // dev.cr1.uncheck(flags::cr1::PEIE);
    // dev.cr2.uncheck(flags::cr2::LBDIE);
    dev.cr3.uncheck(flags::cr3::EIE);
}

pub mod flags {
    pub mod sr {
        /** CTS: CTS flag */
        pub const CTS: u32 = 1 << 9; 

        /** LBD: LIN break detection flag */
        pub const LBD: u32 = 1 << 8; 

        /** TXE: Transmit data buffer empty */
        pub const TXE: u32 = 1 << 7; 

        /** TC: Transmission complete */
        pub const TC: u32 = 1 << 6; 

        /** RXNE: Read data register not empty */
        pub const RXNE: u32 = 1 << 5; 

        /** IDLE: Idle line detected */
        pub const IDLE: u32 = 1 << 4; 

        /** ORE: Overrun error */
        pub const ORE: u32 = 1 << 3; 

        /** NE: Noise error flag */
        pub const NE: u32 = 1 << 2; 

        /** FE: Framing error */
        pub const FE: u32 = 1 << 1; 

        /** PE: Parity error */
        pub const PE: u32 = 1 << 0; 
    }

    pub mod dr {
        pub const MASK: u32 = 0x1FF; 
    }

    pub mod brr {
        pub const DIV_MANTISSA_MASK: u32 = 0xFFF << 4;
        /* DIV_Fraction[3:0]: fraction of USARTDIV */
        pub const DIV_FRACTION_MASK: u32 = 0xF;
    }

    pub mod cr1 {
        pub const OVER8: u32 = 1 << 15;

        pub const UE: u32 = 1 << 13; 

        /* M: Word length */
        pub const M: u32 = 1 << 12; 

        /* WAKE: Wakeup method */
        pub const WAKE: u32 = 1 << 11; 

        /* PCE: Parity control enable */
        pub const PCE: u32 = 1 << 10; 

        /* PS: Parity selection */
        pub const PS: u32 = 1 << 9; 

        /* PEIE: PE interrupt enable */
        pub const PEIE: u32 = 1 << 8; 

        /* TXEIE: TXE interrupt enable */
        pub const TXEIE: u32 = 1 << 7; 

        /* TCIE: Transmission complete interrupt enable */
        pub const TCIE: u32 = 1 << 6; 

        /* RXNEIE: RXNE interrupt enable */
        pub const RXNEIE: u32 = 1 << 5; 

        /* IDLEIE: IDLE interrupt enable */
        pub const IDLEIE: u32 = 1 << 4; 

        /* TE: Transmitter enable */
        pub const TE: u32 = 1 << 3; 

        /* RE: Receiver enable */
        pub const RE: u32 = 1 << 2; 

        /* RWU: Receiver wakeup */
        pub const RWU: u32 = 1 << 1;

        /* SBK: Send break */
        pub const SBK: u32 = 1 << 0;
        
        pub const MODE_MASK: u32 = 3 << 2;
        pub const MODE_SHIFT: u32 = 2;
    }

    pub mod cr2 {
        /* LINEN: LIN mode enable */
        pub const LINEN: u32 = 1 << 14; 

        /* STOP[13:12]: STOP bits */
        pub const STOPBITS_MASK: u32 = 0x03 << 12; 
        pub const STOPBITS_SHIFT: u32 = 12; 

        /* CLKEN: Clock enable */
        pub const CLKEN: u32 = 1 << 11; 

        /* CPOL: Clock polarity */
        pub const CPOL: u32 = 1 << 10; 

        /* CPHA: Clock phase */
        pub const CPHA: u32 = 1 << 9; 

        /* LBCL: Last bit clock pulse */
        pub const LBCL: u32 = 1 << 8; 

        /* LBDIE: LIN break detection interrupt enable */
        pub const LBDIE: u32 = 1 << 6; 

        /* LBDL: LIN break detection length */
        pub const LBDL: u32 = 1 << 5; 

        /* ADD[3:0]: Address of the usart node */
        pub const ADD_MASK: u32 = 0xF; 

        pub const PARITY_MASK: u32 = 3 << 9;
        pub const PARITY_SHIFT: u32 = 9;
    }

    pub mod cr3 {
        pub const ONEBIT: u32 = 1 << 11;
        /* CTSIE: CTS interrupt enable */
        /* Note: N/A on UART4 & UART5 */
        pub const CTSIE: u32 = 1 << 10; 

        /* CTSE: CTS enable */
        /* Note: N/A on UART4 & UART5 */
        pub const CTSE: u32 = 1 << 9; 

        /* RTSE: RTS enable */
        /* Note: N/A on UART4 & UART5 */
        pub const RTSE: u32 = 1 << 8; 

        /* DMAT: DMA enable transmitter */
        /* Note: N/A on UART5 */
        pub const DMAT: u32 = 1 << 7; 

        /* DMAR: DMA enable receiver */
        /* Note: N/A on UART5 */
        pub const DMAR: u32 = 1 << 6; 

        /* SCEN: Smartcard mode enable */
        /* Note: N/A on UART4 & UART5 */
        pub const SCEN: u32 = 1 << 5; 

        /* NACK: Smartcard NACK enable */
        /* Note: N/A on UART4 & UART5 */
        pub const NACK: u32 = 1 << 4; 

        /* HDSEL: Half-duplex selection */
        pub const HDSEL: u32 = 1 << 3; 

        /* IRLP: IrDA low-power */
        pub const IRLP: u32 = 1 << 2; 

        /* IREN: IrDA mode enable */
        pub const IREN: u32 = 1 << 1; 

        /* EIE: Error interrupt enable */
        pub const EIE: u32 = 1 << 0; 

        pub const FLOWCONTROL_MASK: u32 = 3 << 8;
        pub const FLOWCONTROL_SHIFT: u32 = 8;
    }

    pub mod gtpr {
        /* GT[7:0]: Guard time value */
        /* Note: N/A on UART4 & UART5 */
        pub const GT_MASK: u32 = 0xFF << 8; 

        /* PSC[7:0]: Prescaler value */
        /* Note: N/A on UART4/5 */
        pub const PSC_MASK: u32 = 0xFF; 
    }
}

pub enum UartError {
    ParityError,
    FramingError,
    OverrunError,
    RepeatCallError,
    ResetError,
}

#[derive(Copy, Clone, Debug)]
pub enum StopBits {
    One     = 0, /* 1 stop bit */
    Half    = 1, /* 0.5 stop bits */
    Two     = 2, /* 2 stop bits */
    OneHalf = 3  /* 1.5 stop bits */
}

#[derive(Copy, Clone, Debug)]
pub enum Parity {
    None = 0,
    Odd  = 1,
    Even = 2,
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Receive  = 1,
    Transmit = 2,
    Duplex   = 3
}

pub enum FlowControl {
    None      = 0,
    Rts       = 1,
    Cts       = 2,
    RtsAndCts = 3
}