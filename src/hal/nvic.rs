use core::mem;

use common::VolatileCell;
const NVIC_BASE: usize   = 0xE000_E100;

#[repr(C, packed)]
struct Nvic {
    pub iser: [VolatileCell<u32>; 8], _reserved0: [u32; 24],
    pub icer: [VolatileCell<u32>; 8], _reserved1: [u32; 24],
    pub ispr: [VolatileCell<u32>; 8], _reserved2: [u32; 24],
    pub icpr: [VolatileCell<u32>; 8], _reserved3: [u32; 24],
    pub iabr: [VolatileCell<u32>; 8], _reserved4: [u32; 56],
    pub ipr:  [VolatileCell<u8>; 240]
}

#[repr(C)]
#[derive(Copy,Clone)]
#[allow(non_camel_case_types)]
pub enum NvicIdx {
    WWDG,
    PVD,
    TAMP_STAMP,
    RTC_WKUP,
    FLASH,
    RCC,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    DMA1_STREAM0,
    DMA1_STREAM1,
    DMA1_STREAM2,
    DMA1_STREAM3,
    DMA1_STREAM4,
    DMA1_STREAM5,
    DMA1_STREAM6,
    ADC,
    CAN1_TX,
    CAN1_RX0,
    CAN1_RX1,
    CAN1_SCE,
    EXTI9_5,
    TIM1_BRK_TIM9,
    TIM1_UP_TIM10,
    TIM1_TRG_COM_TIM11,
    TIM1_CC,
    TIM2,
    TIM3,
    TIM4,
    I2C1_EV,
    I2C1_ER,
    I2C2_EV,
    I2C2_ER,
    SPI1,
    SPI2,
    USART1,
    USART2,
    USART3,
    EXTI15_10,
    RTC_ALARM,
    OTG_FS_WKUP,
    TIM8_BRK_TIM12,
    TIM8_UP_TIM13,
    TIM8_TRG_COM_TIM14,
    TIM8_CC,
    DMA1_STREAM7,
    FSMC,
    SDIO,
    TIM5,
    SPI3,
    UART4,
    UART5,
    TIM6_DAC,
    TIM7,
    DMA2_STREAM0,
    DMA2_STREAM1,
    DMA2_STREAM2,
    DMA2_STREAM3,
    DMA2_STREAM4,
    ETH,
    ETH_WKUP,
    CAN2_TX,
    CAN2_RX0,
    CAN2_RX1,
    CAN2_SCE,
    OTG_FS,
    DMA2_STREAM5,
    DMA2_STREAM6,
    DMA2_STREAM7,
    USART6,
    I2C3_EV,
    I2C3_ER,
    OTG_HS_EP1_OUT,
    OTG_HS_EP1_IN,
    OTG_HS_WKUP,
    OTG_HS,
    DCMI,
    CRYP,
    HASH_RNG,
    FPU,
        _reserved0,
        _reserved1,
        _reserved2,
        _reserved3,
        _reserved4,
        _reserved5,
    LCD_TFT,
    LCD_TFT_1,
}

impl ::core::default::Default for NvicIdx {
    fn default() -> NvicIdx {
        NvicIdx::WWDG
    }
}

pub fn enable(signal: NvicIdx) {
    let nvic: &mut Nvic = unsafe {mem::transmute(NVIC_BASE)};
    let interrupt = signal as usize;

    unsafe {
        nvic.ipr[interrupt].set(0u8);
        nvic.iser[interrupt / 32].set((1 << (interrupt & 0b11111)) as u32);
    }
}

pub fn disable(signal: NvicIdx) {
    let nvic: &mut Nvic = unsafe {mem::transmute(NVIC_BASE)};
    let interrupt = signal as usize;

    unsafe {
        nvic.icer[interrupt / 32].set((1 << (interrupt & 0b11111)) as u32);
    };
}

pub fn clear_pending(signal: NvicIdx) {
    let nvic: &mut Nvic = unsafe {mem::transmute(NVIC_BASE)};
    let interrupt = signal as usize;
    unsafe {
        nvic.icpr[interrupt / 32].set((1 << (interrupt & 0b11111)) as u32);
    };
}
