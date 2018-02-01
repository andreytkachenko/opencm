use common::VolatileCell;
use common::asm;

use hal::{
	pwr,
	flash
};

const RCC_BASE: usize = 0x4002_3800;
static mut FREQUENCIES: Frequencies = Frequencies::new(8_000_000, 8_000_000, 8_000_000);

#[allow(dead_code)]
pub enum Peripheral {
	/* AHB1 peripherals*/
	GPIOA	    = 0x000,
	GPIOB	    = 0x001,
	GPIOC	    = 0x002,
	GPIOD	    = 0x003,
	GPIOE	    = 0x004,
	GPIOF	    = 0x005,
	GPIOG	    = 0x006,
	GPIOH	    = 0x007,
	GPIOI	    = 0x008,
	GPIOJ	    = 0x009,
	GPIOK	    = 0x00A,
	CRC		    = 0x00C,
	FLTIF	    = 0x00F,
	SRAM1	    = 0x010,
	SRAM2	    = 0x011,
	BKPSRAM	    = 0x012,
	SRAM3	    = 0x013,/* F2xx, F3xx */
	DMA1	    = 0x015,
	DMA2	    = 0x016,
	DMA2D	    = 0x017, /* F4x9 */
	ETHMAC	    = 0x019,
	ETHMACTX	= 0x01A,
	ETHMACRX	= 0x01B,
	ETHMACPTP	= 0x01C,
	OTGHS	    = 0x01D,
	OTGHSULPI	= 0x01E,

	/* AHB2 peripherals */
	DCMI	    = 0x100,
	CRYP	    = 0x104,
	HASH	    = 0x105,
	RNG		    = 0x106,
	OTGFS	    = 0x107,

	/* AHB3 peripherals */
	FSMC	    = 0x200,
	QSPIC	    = 0x201,

	/* APB1 peripherals*/
	TIM2	    = 0x300,
	TIM3	    = 0x301,
	TIM4	    = 0x302,
	TIM5	    = 0x303,
	TIM6	    = 0x304,
	TIM7	    = 0x305,
	TIM12	    = 0x306,
	TIM13	    = 0x307,
	TIM14	    = 0x308,
	WWDG	    = 0x30B,
	SPI2	    = 0x30E,
	SPI3	    = 0x30F,
	USART2	    = 0x311,
	USART3	    = 0x312,
	UART4	    = 0x313,
	UART5	    = 0x314,
	I2C1	    = 0x315,
	I2C2	    = 0x316,
	I2C3	    = 0x317,
	CAN1	    = 0x319,
	CAN2	    = 0x31A,
    PWR		    = 0x31C,
	DAC		    = 0x31D,
	UART7	    = 0x31E,/* F2xx, F3xx */
	UART8	    = 0x31F,/* F2xx, F3xx */

	/* APB2 peripherals */
	TIM1	    = 0x400,
	TIM8	    = 0x401,
	USART1	    = 0x404,
	USART6	    = 0x405,
	ADC1	    = 0x408,
	ADC2	    = 0x409,
	ADC3	    = 0x40A,
	SDIO	    = 0x40B,
	SPI1	    = 0x40C,
	SPI4	    = 0x40D,/* F2xx, F3xx */
	SYSCFG	    = 0x40E,
	TIM9	    = 0x410,
	TIM10	    = 0x411,
	TIM11	    = 0x412,
	SPI5	    = 0x414,/* F2xx, F3xx */
	SPI6	    = 0x415,/* F2xx, F3xx */
	SAI1	    = 0x416,/* F4x9 */
	LTDC	    = 0x41A,/* F4x9 */
	DSI		    = 0x41B,/* F4x9 */
	/* BDCR */
	RTC		    = 0x50F
}

#[repr(C)]
struct Registers {
    pub cr: 	  	VolatileCell<u32>,
    pub pllcfgr:  	VolatileCell<u32>,
    pub cfgr: 	  	VolatileCell<u32>,
    pub cir:	  	VolatileCell<u32>,
    pub ahb1rstr: 	VolatileCell<u32>,
    pub ahb2rstr: 	VolatileCell<u32>,
    pub ahb3rstr: 	VolatileCell<u32>, _reserved0: [u8; 4usize],
    pub apb1rstr: 	VolatileCell<u32>,
    pub apb2rstr: 	VolatileCell<u32>, _reserved1: [u8; 8usize],
    pub ahb1enr:  	VolatileCell<u32>,
    pub ahb2enr:  	VolatileCell<u32>,
    pub ahb3enr:  	VolatileCell<u32>, _reserved2: [u8; 4usize],
    pub apb1enr:  	VolatileCell<u32>,
    pub apb2enr:  	VolatileCell<u32>, _reserved3: [u8; 8usize],
    pub ahb1lpenr: 	VolatileCell<u32>,
    pub ahb2lpenr: 	VolatileCell<u32>,
    pub ahb3lpenr: 	VolatileCell<u32>, _reserved4: [u8; 4usize],
    pub apb1lpenr: 	VolatileCell<u32>,
    pub apb2lpenr: 	VolatileCell<u32>, _reserved5: [u8; 8usize],
    pub bdcr:     	VolatileCell<u32>,
    pub csr:      	VolatileCell<u32>, _reserved6: [u8; 8usize],
    pub sscgr: 		VolatileCell<u32>,
    pub plli2scfgr: VolatileCell<u32>,
}

pub fn enable(perph: Peripheral) {
    let reg: &Registers = unsafe { &*(RCC_BASE as *const Registers) };

    let p = perph as u32;
	let bit = (p & 0xFF) as u32;
	let bus = (p >> 8) & 0xFF;

	match bus {
		0 => reg.ahb1enr.check(bit),
		1 => reg.ahb2enr.check(bit),
		2 => reg.ahb3enr.check(bit),
		3 => reg.apb1enr.check(bit),
		4 => reg.apb2enr.check(bit),
		5 => reg.bdcr.check(bit),
		_ => {}
	}
}

pub fn disable(perph: Peripheral) {
    let reg: &Registers = unsafe { &*(RCC_BASE as *const Registers) };

    let p = perph as u32;
	let bit = (p & 0xFF) as u32;
	let bus = (p >> 8) & 0xFF;

	match bus {
		0 => reg.ahb1enr.uncheck(bit),
		1 => reg.ahb2enr.uncheck(bit),
		2 => reg.ahb3enr.uncheck(bit),
		3 => reg.apb1enr.uncheck(bit),
		4 => reg.apb2enr.uncheck(bit),
		5 => reg.bdcr.uncheck(bit),
		_ => {}
	}
}

pub fn reset_pulse(perph: Peripheral) {
	let rcc: &Registers = unsafe { &*(RCC_BASE as *const Registers) };

    let p = perph as u32;
	let bit = (p & 0xFF) as u32;
	let bus = (p >> 8) & 0xFF;

	match bus {
		0 => { rcc.ahb1rstr.check(bit); rcc.ahb1rstr.uncheck(bit); },
		1 => { rcc.ahb2rstr.check(bit); rcc.ahb2rstr.uncheck(bit); },
		2 => { rcc.ahb3rstr.check(bit); rcc.ahb3rstr.uncheck(bit); },
		3 => { rcc.apb1rstr.check(bit); rcc.apb1rstr.uncheck(bit); },
		4 => { rcc.apb2rstr.check(bit); rcc.apb2rstr.uncheck(bit); },
		// 5 => rcc.bdcr.uncheck(bit)
		_ => {}
	}
}

pub fn reset_hold(perph: Peripheral) {
	let rcc: &Registers = unsafe { &*(RCC_BASE as *const Registers) };

    let p = perph as u32;
	let bit = (p & 0xFF) as u32;
	let bus = (p >> 8) & 0xFF;

	match bus {
		0 => rcc.ahb1rstr.check(bit),
		1 => rcc.ahb2rstr.check(bit),
		2 => rcc.ahb3rstr.check(bit),
		3 => rcc.apb1rstr.check(bit),
		4 => rcc.apb2rstr.check(bit),
		// 5 => rcc.bdcr.uncheck(bit)
		_ => {}
	}
}

pub fn reset_release(perph: Peripheral) {
	let rcc: &Registers = unsafe { &*(RCC_BASE as *const Registers) };

    let p = perph as u32;
	let bit = (p & 0xFF) as u32;
	let bus = (p >> 8) & 0xFF;

	match bus {
		0 => rcc.ahb1rstr.uncheck(bit),
		1 => rcc.ahb2rstr.uncheck(bit),
		2 => rcc.ahb3rstr.uncheck(bit),
		3 => rcc.apb1rstr.uncheck(bit),
		4 => rcc.apb2rstr.uncheck(bit),
		// 5 => rcc.bdcr.uncheck(bit)
		_ => {}
	}
}

pub fn set_clock(base_clock: CrystalClock, target_clock: Clock) {
	let scale = CLOCK_SCALE[base_clock as usize][target_clock as usize];

	/* Enable internal high-speed oscillator. */
	osc_on(flags::Osc::HSI);
	wait_for_osc_ready(flags::Osc::HSI);

	/* Select HSI as SYSCLK source. */
	set_sysclk_source(flags::cfgr::SW_HSI);

	// /* Enable external high-speed oscillator 8MHz. */
	osc_on(flags::Osc::HSE);
	wait_for_osc_ready(flags::Osc::HSE);

	// /* Enable/disable high performance mode */
	if !scale.power_save {
		pwr::set_vos_scale(pwr::flags::VOSScale::Scale1);
	} else {
		pwr::set_vos_scale(pwr::flags::VOSScale::Scale2);
	}

	// /*
	//  * Set prescalers for AHB, ADC, ABP1, ABP2.
	//  * Do this before touching the PLL (TODO: why?).
	//  */
	set_hpre(scale.hpre);
	set_ppre1(scale.ppre1);
	set_ppre2(scale.ppre2);

	set_main_pll_hse(scale.pllm, scale.plln, scale.pllp, scale.pllq, scale.pllr);

	// /* Enable PLL oscillator and wait for it to stabilize. */
	osc_on(flags::Osc::PLL);
	wait_for_osc_ready(flags::Osc::PLL);

	// /* Configure flash settings. */
	flash::set_ws(flash::flags::acr::DCEN | flash::flags::acr::ICEN | scale.flash_config);

	// /* Select PLL as SYSCLK source. */
	set_sysclk_source(flags::cfgr::SW_PLL);

	// /* Wait for PLL clock to be selected. */
	wait_for_sysclk_status(flags::Osc::PLL);

	// /* Set the peripheral clock frequencies used. */

	let master_clock: u32 = 1000_000 * (scale.plln >> 1);

	unsafe {
		FREQUENCIES.update(
			master_clock,
			match scale.ppre1 {
				flags::cfgr::PPRE_DIV_NONE => master_clock,
				flags::cfgr::PPRE_DIV_2    => master_clock >> 1,
				flags::cfgr::PPRE_DIV_4    => master_clock >> 2,
				flags::cfgr::PPRE_DIV_8    => master_clock >> 3,
				flags::cfgr::PPRE_DIV_16   => master_clock >> 4,
				_ => unreachable!()
			},
			match scale.ppre2 {
				flags::cfgr::PPRE_DIV_NONE => master_clock,
				flags::cfgr::PPRE_DIV_2    => master_clock >> 1,
				flags::cfgr::PPRE_DIV_4    => master_clock >> 2,
				flags::cfgr::PPRE_DIV_8    => master_clock >> 3,
				flags::cfgr::PPRE_DIV_16   => master_clock >> 4,
				_ => unreachable!()
			}
		);
	}
	// rcc_ahb_frequency  = clock->ahb_frequency;
	// rcc_apb1_frequency = clock->apb1_frequency;
	// rcc_apb2_frequency = clock->apb2_frequency;

	/* Disable internal high-speed oscillator. */
	osc_off(flags::Osc::HSI);
}

fn osc_on(osc: flags::Osc)
{
	let rcc: &Registers = unsafe { &*(RCC_BASE as *const Registers) };

	match osc {
		flags::Osc::PLL => rcc.cr.check(flags::cr::PLLON),
		flags::Osc::HSE => rcc.cr.check(flags::cr::HSEON),
		flags::Osc::PLLSAI => rcc.cr.check(flags::cr::PLLSAION),
		flags::Osc::PLLI2S => rcc.cr.check(flags::cr::PLLI2SON),
		flags::Osc::HSI => rcc.cr.check(flags::cr::HSION),
		flags::Osc::LSE => rcc.bdcr.check(flags::bdcr::LSEON),
		flags::Osc::LSI => rcc.csr.check(flags::csr::LSION),
	};
}

fn osc_off(osc: flags::Osc) {
	let rcc: &Registers = unsafe { &*(RCC_BASE as *const Registers) };

	match osc {
		flags::Osc::PLL => rcc.cr.uncheck(flags::cr::PLLON),
		flags::Osc::HSE => rcc.cr.uncheck(flags::cr::HSEON),
		flags::Osc::PLLSAI => rcc.cr.uncheck(flags::cr::PLLSAION),
		flags::Osc::PLLI2S => rcc.cr.uncheck(flags::cr::PLLI2SON),
		flags::Osc::HSI => rcc.cr.uncheck(flags::cr::HSION),
		flags::Osc::LSE => rcc.bdcr.uncheck(flags::bdcr::LSEON),
		flags::Osc::LSI => rcc.csr.uncheck(flags::csr::LSION),
	};
}

fn is_osc_ready(osc: flags::Osc) -> bool
{
	let rcc: &Registers = unsafe { &*(RCC_BASE as *const Registers) };

	match osc {
		flags::Osc::PLL => rcc.cr.test(flags::cr::PLLRDY),
		flags::Osc::HSE => rcc.cr.test(flags::cr::HSERDY),
		flags::Osc::PLLSAI => rcc.cr.test(flags::cr::PLLSAIRDY),
		flags::Osc::PLLI2S => rcc.cr.test(flags::cr::PLLI2SRDY),
		flags::Osc::HSI => rcc.cr.test(flags::cr::HSIRDY),
		flags::Osc::LSE => rcc.bdcr.test(flags::bdcr::LSERDY),
		flags::Osc::LSI => rcc.csr.test(flags::csr::LSIRDY)
	}
}

fn wait_for_osc_ready(osc: flags::Osc) {
	while !is_osc_ready(osc) {
		asm::nop();
	};
}

fn set_sysclk_source(clk: u32) {
	let rcc: &Registers = unsafe { &*(RCC_BASE as *const Registers) };
	let mut reg32 = rcc.cfgr.get();

	reg32 &= !0b11;
	rcc.cfgr.set(reg32 | clk);
}

fn wait_for_sysclk_status(osc: flags::Osc) {
	let rcc: &Registers = unsafe { &*(RCC_BASE as *const Registers) };
	
	match osc {
		flags::Osc::PLL => while ((rcc.cfgr.get() >> flags::cfgr::SWS_SHIFT) & flags::cfgr::SWS_MASK) != flags::cfgr::SWS_PLL {asm::nop();},
		flags::Osc::HSE => while ((rcc.cfgr.get() >> flags::cfgr::SWS_SHIFT) & flags::cfgr::SWS_MASK) != flags::cfgr::SWS_HSE {asm::nop();},
		flags::Osc::HSI => while ((rcc.cfgr.get() >> flags::cfgr::SWS_SHIFT) & flags::cfgr::SWS_MASK) != flags::cfgr::SWS_HSI {asm::nop();},
		_ => {}
	};
}

fn set_ppre2(ppre2: u32) {
	let rcc: &Registers = unsafe { &*(RCC_BASE as *const Registers) };
	let mut reg32 = rcc.cfgr.get();

	reg32 &= !0b111 << 13;
	rcc.cfgr.set(reg32 | (ppre2 << 13));
}

fn set_ppre1(ppre1: u32) {
	let rcc: &Registers = unsafe { &*(RCC_BASE as *const Registers) };
	let mut reg32 = rcc.cfgr.get();

	reg32 &= !0b111 << 10;
	rcc.cfgr.set(reg32 | (ppre1 << 10));
}

fn set_hpre(hpre: u32) {
	let rcc: &Registers = unsafe { &*(RCC_BASE as *const Registers) };
	let mut reg32 = rcc.cfgr.get();

	reg32 &= 0b1111 << 4;
	rcc.cfgr.set(reg32 | (hpre << 4));
}


pub fn set_main_pll_hse(pllm: u32, plln: u32, pllp: u32, pllq: u32, mut pllr: u32) {
	let rcc: &Registers = unsafe { &*(RCC_BASE as *const Registers) };
	
	/* Use reset value if not legal, for parts without pllr */
	if pllr < 2 {
		pllr = 2;
	}

	rcc.pllcfgr.set(flags::pllcfgr::PLLSRC | /* HSE */
		((pllm & flags::pllcfgr::PLLM_MASK) << flags::pllcfgr::PLLM_SHIFT) |
		((plln & flags::pllcfgr::PLLN_MASK) << flags::pllcfgr::PLLN_SHIFT) |
		((((pllp >> 1) - 1) & flags::pllcfgr::PLLP_MASK) << flags::pllcfgr::PLLP_SHIFT) |
		((pllq & flags::pllcfgr::PLLQ_MASK) << flags::pllcfgr::PLLQ_SHIFT) |
		((pllr & flags::pllcfgr::PLLR_MASK) << flags::pllcfgr::PLLR_SHIFT));
}

pub fn ahb_frequency() -> u32 {
	unsafe {FREQUENCIES.ahb}
}

pub fn apb1_frequency() -> u32 {
	unsafe {FREQUENCIES.apb1}
}

pub fn apb2_frequency() -> u32 {
	unsafe {FREQUENCIES.apb2}
}

pub fn ahb_ms_psc() -> u32 {
	unsafe {FREQUENCIES.ahb_mspsc}
}

pub fn apb1_ms_psc() -> u32 {
	unsafe {FREQUENCIES.apb1_mspsc}
}

pub fn apb2_ms_psc() -> u32 {
	unsafe {FREQUENCIES.apb2_mspsc}
}

pub fn ahb_us_psc() -> u32 {
	unsafe {FREQUENCIES.ahb_uspsc}
}

pub fn apb1_us_psc() -> u32 {
	unsafe {FREQUENCIES.apb1_uspsc}
}

pub fn apb2_us_psc() -> u32 {
	unsafe {FREQUENCIES.apb2_uspsc}
}

#[allow(dead_code)]
mod flags {
	pub const HSI: u32 = 0;
	pub const HSE: u32 = 0;
	pub const PLL: u32 = 0;

	pub mod cfgr {
		/* MCO2: Microcontroller clock output 2 */
		pub const MCO2_SHIFT: u32 = 30;
		pub const MC02_MASK: u32 = 0x3;
		pub const MCO2_SYSCLK: u32 = 0x0;
		pub const MCO2_PLLI2S: u32 = 0x1;
		pub const MCO2_HSE: u32 = 0x2;
		pub const MCO2_PLL: u32 = 0x3;

		/* MCO1/2PRE: MCO Prescalers */
		pub const MCO2PRE_SHIFT: u32 = 27;
		pub const MCO2PRE_MASK: u32 = 0x7;
		pub const MCO1PRE_SHIFT: u32 = 24;
		pub const MCO1PRE_MASK: u32 = 0x7;
		pub const MCOPRE_DIV_NONE: u32 = 0x0;
		pub const MCOPRE_DIV_2: u32 = 0x4;
		pub const MCOPRE_DIV_3: u32 = 0x5;
		pub const MCOPRE_DIV_4: u32 = 0x6;
		pub const MCOPRE_DIV_5: u32 = 0x7;

		/* I2SSRC: I2S clock selection */
		pub const I2SSRC: u32 = 1 << 23;

		/* MCO1: Microcontroller clock output 1 */
		pub const MCO1_SHIFT: u32 = 21;
		pub const MCO1_MASK: u32 = 0x3;
		pub const MCO1_HSI: u32 = 0x0;
		pub const MCO1_LSE: u32 = 0x1;
		pub const MCO1_HSE: u32 = 0x2;
		pub const MCO1_PLL: u32 = 0x3;
		pub const MCO_SHIFT: u32 = MCO1_SHIFT;
		pub const MCO_MASK: u32 = MCO1_MASK;

		/* RTCPRE: HSE division factor for RTC clock */
		pub const RTCPRE_SHIFT: u32 = 16;
		pub const RTCPRE_MASK: u32 = 0x1f;

		/* PPRE1/2: APB high-speed prescalers */
		pub const PPRE2_SHIFT: u32 = 13;
		pub const PPRE2_MASK: u32 = 0x7;
		pub const PPRE1_SHIFT: u32 = 10;
		pub const PPRE1_MASK: u32 = 0x7;
		pub const PPRE_DIV_NONE: u32 = 0x0;
		pub const PPRE_DIV_2: u32 = 0x4;
		pub const PPRE_DIV_4: u32 = 0x5;
		pub const PPRE_DIV_8: u32 = 0x6;
		pub const PPRE_DIV_16: u32 = 0x7;

		/* HPRE: AHB high-speed prescaler */
		pub const HPRE_SHIFT: u32 = 4;
		pub const HPRE_MASK: u32 = 0xf;
		pub const HPRE_DIV_NONE: u32 = 0x0;
		pub const HPRE_DIV_2: u32 = 0x8 + 0;
		pub const HPRE_DIV_4: u32 = 0x8 + 1;
		pub const HPRE_DIV_8: u32 = 0x8 + 2;
		pub const HPRE_DIV_16: u32 = 0x8 + 3;
		pub const HPRE_DIV_64: u32 = 0x8 + 4;
		pub const HPRE_DIV_128: u32 = 0x8 + 5;
		pub const HPRE_DIV_256: u32 = 0x8 + 6;
		pub const HPRE_DIV_512: u32 = 0x8 + 7;

		/* SWS: System clock switch status */
		pub const SWS_SHIFT: u32 = 2;
		pub const SWS_MASK: u32 = 0x3;
		pub const SWS_HSI: u32 = 0x0;
		pub const SWS_HSE: u32 = 0x1;
		pub const SWS_PLL: u32 = 0x2;

		/* SW: System clock switch */
		pub const SW_SHIFT: u32 = 0;
		pub const SW_HSI: u32 = 0x0;
		pub const SW_HSE: u32 = 0x1;
		pub const SW_PLL: u32 = 0x2;
	}

	pub mod pllcfgr {
		pub const PLLR_SHIFT: u32 = 28;
		pub const PLLR_MASK: u32 = 0x7;
		
		/* PLLQ: [27:24] */
		pub const PLLQ_SHIFT: u32 = 24;
		pub const PLLQ_MASK: u32 = 0xf;
		pub const PLLSRC: u32 = 1 << 22;
		
		/* PLLP: [17:16] */
		pub const PLLP_SHIFT: u32 = 16;
		pub const PLLP_MASK: u32 = 0x3;
		
		/* PLLN: [14:6] */
		pub const PLLN_SHIFT: u32 = 6;
		pub const PLLN_MASK: u32 = 0x1ff;
		
		/* PLLM: [5:0] */
		pub const PLLM_SHIFT: u32 = 0;
		pub const PLLM_MASK: u32 = 0x3f;
	}

	pub mod cr {
		pub const PLLSAIRDY: u32 = 1 << 29;
		pub const PLLSAION: u32 = 1 << 28;
		pub const PLLI2SRDY: u32 = 1 << 27;
		pub const PLLI2SON: u32 = 1 << 26;
		pub const PLLRDY: u32 = 1 << 25;
		pub const PLLON: u32 = 1 << 24;
		pub const CSSON: u32 = 1 << 19;
		pub const HSEBYP: u32 = 1 << 18;
		pub const HSERDY: u32 = 1 << 17;
		pub const HSEON: u32 = 1 << 16;

		/* HSICAL: [15:8] */
		/* HSITRIM: [7:3] */
		pub const HSITRIM_SHIFT: u32 = 3;
		pub const HSITRIM_MASK: u32 = 0x1f;
		pub const HSIRDY: u32 = 1 << 1;
		pub const HSION: u32 = 1 << 0;
	}

	pub mod csr {
		pub const LPWRRSTF: u32 = 1 << 31;
		pub const WWDGRSTF: u32 = 1 << 30;
		pub const IWDGRSTF: u32 = 1 << 29;
		pub const SFTRSTF: u32 = 1 << 28;
		pub const PORRSTF: u32 = 1 << 27;
		pub const PINRSTF: u32 = 1 << 26;
		pub const BORRSTF: u32 = 1 << 25;
		pub const RMVF: u32 = 1 << 24;
		pub const RESET_FLAGS: u32 = LPWRRSTF | WWDGRSTF | IWDGRSTF | SFTRSTF | PORRSTF | PINRSTF | BORRSTF;
		pub const LSIRDY: u32 = 1 << 1;
		pub const LSION: u32 = 1 << 0;
	}

	pub mod bdcr {
		pub const BDRST: u32 = 1 << 16;
		pub const RTCEN: u32 = 1 << 15;

		/* RCC_BDCR[9:8]: RTCSEL */
		pub const RTCSEL_SHIFT: u32 = 8;
		pub const RTCSEL_MASK: u32 = 0x3;
		pub const RTCSEL_NONE: u32 = 0;
		pub const RTCSEL_LSE: u32 = 1;
		pub const RTCSEL_LSI: u32 = 2;
		pub const RTCSEL_HSE: u32 = 3;
		pub const LSEMOD: u32 = 1 << 3;
		pub const LSEBYP: u32 = 1 << 2;
		pub const LSERDY: u32 = 1 << 1;
		pub const LSEON: u32 = 1 << 0;
	}

	#[derive(Copy, Clone)]
	pub enum Osc {
		PLL,
		PLLSAI,
		PLLI2S,
		HSE,
		HSI,
		LSE,
		LSI
	}
}

pub enum CrystalClock {
	Clock8MHz,
	Clock12MHz,
	Clock16MHz,
	Clock25MHz,
}

pub enum Clock {
	Clock48MHz,
	Clock84MHz,
	Clock120MHz,
	Clock168MHz,
}

#[derive(Copy, Clone)]
struct ClockScale {
	pllm: u32,
	plln: u32,
	pllp: u32,
	pllq: u32,
	pllr: u32,
	hpre: u32,
	ppre1: u32,
	ppre2: u32,
	power_save: bool,
	flash_config: u32,
}

impl ClockScale {
	const fn new(pllm: u32, plln: u32, pllp: u32, pllq: u32, pllr: u32, hpre: u32, 
				 ppre1: u32, ppre2: u32, power_save: bool, flash_config: u32) -> ClockScale 
	{
		ClockScale {
			pllm,
			plln,
			pllp,
			pllq,
			pllr,
			hpre,
			ppre1,
			ppre2,
			power_save,
			flash_config,
		}
	}
}

const CLOCK_SCALE: [[ClockScale; 4]; 4] = [
	[ 
		//              M   N    P  Q  R  HPRE                        PPRE1                    PPRE2                       PWRSV  FLASH_CONFIG
		ClockScale::new(8,  96,  2, 2, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_4, flags::cfgr::PPRE_DIV_2,    true,  flash::flags::acr::LATENCY_3WS),
		ClockScale::new(8,  336, 4, 7, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_2, flags::cfgr::PPRE_DIV_NONE, false, flash::flags::acr::LATENCY_2WS),
		ClockScale::new(8,  240, 2, 5, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_4, flags::cfgr::PPRE_DIV_2,    true,  flash::flags::acr::LATENCY_3WS),
		ClockScale::new(8,  336, 2, 7, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_4, flags::cfgr::PPRE_DIV_2,    false, flash::flags::acr::LATENCY_5WS),
	], [
		ClockScale::new(12, 96,  2, 2, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_4, flags::cfgr::PPRE_DIV_2,    true,  flash::flags::acr::LATENCY_3WS),
		ClockScale::new(12, 336, 4, 7, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_2, flags::cfgr::PPRE_DIV_NONE, false, flash::flags::acr::LATENCY_2WS),
		ClockScale::new(12, 240, 2, 5, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_4, flags::cfgr::PPRE_DIV_2,    true,  flash::flags::acr::LATENCY_3WS),
		ClockScale::new(12, 336, 2, 7, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_4, flags::cfgr::PPRE_DIV_2,    false, flash::flags::acr::LATENCY_5WS),
	], [
		ClockScale::new(16, 96,  2, 2, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_4, flags::cfgr::PPRE_DIV_2,    true,  flash::flags::acr::LATENCY_3WS),
		ClockScale::new(16, 336, 4, 7, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_2, flags::cfgr::PPRE_DIV_NONE, false, flash::flags::acr::LATENCY_2WS),
		ClockScale::new(16, 240, 2, 5, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_4, flags::cfgr::PPRE_DIV_2,    true,  flash::flags::acr::LATENCY_3WS),
		ClockScale::new(16, 336, 2, 7, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_4, flags::cfgr::PPRE_DIV_2,    false, flash::flags::acr::LATENCY_5WS),
	], [
		ClockScale::new(25, 96,  2, 2, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_4, flags::cfgr::PPRE_DIV_2,    true,  flash::flags::acr::LATENCY_3WS),
		ClockScale::new(25, 336, 4, 7, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_2, flags::cfgr::PPRE_DIV_NONE, false, flash::flags::acr::LATENCY_2WS),
		ClockScale::new(25, 240, 2, 5, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_4, flags::cfgr::PPRE_DIV_2,    true,  flash::flags::acr::LATENCY_3WS),
		ClockScale::new(25, 336, 2, 7, 0, flags::cfgr::HPRE_DIV_NONE, flags::cfgr::PPRE_DIV_4, flags::cfgr::PPRE_DIV_2,    false, flash::flags::acr::LATENCY_5WS),
	],
];

struct Frequencies {
	pub ahb: u32,
	pub apb1: u32,
	pub apb2: u32,

	pub ahb_mspsc: u32,
	pub apb1_mspsc: u32,
	pub apb2_mspsc: u32,

	pub ahb_uspsc: u32,
	pub apb1_uspsc: u32,
	pub apb2_uspsc: u32,
}

impl Frequencies {
	const fn new(ahb: u32, apb1: u32, apb2: u32) -> Frequencies {
		Frequencies { 
			ahb, 
			apb1, 
			apb2,

			ahb_mspsc:  ahb  / 1000,
			apb1_mspsc: apb1 / 1000,
			apb2_mspsc: apb2 / 1000,

			ahb_uspsc:  ahb  / 1000_000,
			apb1_uspsc: apb1 / 1000_000,
			apb2_uspsc: apb2 / 1000_000,
		}
	}

	fn update(&mut self, ahb: u32, apb1: u32, apb2: u32) {
		self.ahb = ahb;
		self.apb1 = apb1;
		self.apb2 = apb2;

		self.ahb_mspsc  = ahb  / 1000;
		self.apb1_mspsc = apb1 / 1000;
		self.apb2_mspsc = apb2 / 1000;

		self.ahb_uspsc  = ahb  / 1000_000;
		self.apb1_uspsc = apb1 / 1000_000;
		self.apb2_uspsc = apb2 / 1000_000;
	}
}