use core::ops::Deref;
use common::VolatileCell;
use rcc;

pub const TIM1:  Timer = Timer { addr: 0x4001_0000 };
pub const TIM2:  Timer = Timer { addr: 0x4000_0000 };
pub const TIM3:  Timer = Timer { addr: 0x4000_0400 };
pub const TIM4:  Timer = Timer { addr: 0x4000_0800 };
pub const TIM5:  Timer = Timer { addr: 0x4000_0c00 };
pub const TIM6:  Timer = Timer { addr: 0x4000_1000 };
pub const TIM7:  Timer = Timer { addr: 0x4000_1400 };
pub const TIM8:  Timer = Timer { addr: 0x4001_0400 };
pub const TIM9:  Timer = Timer { addr: 0x4001_4000 };
pub const TIM10: Timer = Timer { addr: 0x4001_4400 };
pub const TIM11: Timer = Timer { addr: 0x4001_4800 };
pub const TIM12: Timer = Timer { addr: 0x4000_1800 };
pub const TIM13: Timer = Timer { addr: 0x4000_1c00 };
pub const TIM14: Timer = Timer { addr: 0x4000_2000 };

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Timer {
    addr: usize
}

impl Timer {
    const fn new(addr: usize) -> Timer {
        Timer { addr }
    }

    fn is_advanced(&self) -> bool {
        *self == TIM1 || *self == TIM8
    }
}

impl Deref for Timer {
    type Target = Registers;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.addr as *const Registers) }
    }
}

#[repr(C)]
pub struct Registers {
    pub cr1:  VolatileCell<u32>,
    pub cr2:  VolatileCell<u32>, 
    pub smcr: VolatileCell<u32>,
    pub dier: VolatileCell<u32>,
    pub sr:   VolatileCell<u32>,
    pub egr:  VolatileCell<u32>, 
    pub ccmr1: VolatileCell<u32>,
    pub ccmr2: VolatileCell<u32>,
    pub ccer: VolatileCell<u32>,
    pub cnt:  VolatileCell<u32>,
    pub psc:  VolatileCell<u32>,
    pub arr:  VolatileCell<u32>,
    pub rcr:  VolatileCell<u32>,
    pub ccr1: VolatileCell<u32>,
    pub ccr2: VolatileCell<u32>,
    pub ccr3: VolatileCell<u32>,
    pub ccr4: VolatileCell<u32>,
    pub bdtr: VolatileCell<u32>,
    pub dcr:  VolatileCell<u32>,
    pub dmar: VolatileCell<u32>,
    pub or:   VolatileCell<u32>,    
}

#[allow(dead_code)]
mod flags {
    pub mod cr1 {
        /* CKD[1:0]: Clock division */
        pub const CKD_CK_INT: u32 = 0x0 << 8; 
        pub const CKD_CK_INT_MUL_2: u32 = 0x1 << 8; 
        pub const CKD_CK_INT_MUL_4: u32 = 0x2 << 8; 
        pub const CKD_CK_INT_MASK: u32 = 0x3 << 8;

        /* ARPE: Auto-reload preload enable */
        pub const ARPE: u32 = 1 << 7; 

        /* CMS[1:0]: Center-aligned mode selection */
        pub const CMS_EDGE: u32 = 0x0 << 5; 
        pub const CMS_CENTER_1: u32 = 0x1 << 5; 
        pub const CMS_CENTER_2: u32 = 0x2 << 5; 
        pub const CMS_CENTER_3: u32 = 0x3 << 5; 
        pub const CMS_MASK: u32 = 0x3 << 5; 

        /* DIR: Direction */
        pub const DIR_UP: u32 = 0 << 4; 
        pub const DIR_DOWN: u32 = 1 << 4;

        /* OPM: One pulse mode */
        pub const OPM: u32 = 1 << 3; 

        /* URS: Update request source */
        pub const URS: u32 = 1 << 2; 

        /* UDIS: Update disable */
        pub const UDIS: u32 = 1 << 1; 

        /* CEN: Counter enable */
        pub const CEN: u32 = 1 << 0; 
    }

    pub mod cr2 {
        /* OIS4: Output idle state 4 (OC4 output) */
        pub const OIS4: u32 = 1 << 14; 

        /* OIS3N: Output idle state 3 (OC3N output) */
        pub const OIS3N: u32 = 1 << 13; 
        
        /* OIS3: Output idle state 3 (OC3 output) */
        pub const OIS3: u32 = 1 << 12; 
        
        /* OIS2N: Output idle state 2 (OC2N output) */
        pub const OIS2N: u32 = 1 << 11; 
        
        /* OIS2: Output idle state 2 (OC2 output) */
        pub const OIS2: u32 = 1 << 10; 
        
        /* OIS1N: Output idle state 1 (OC1N output) */
        pub const OIS1N: u32 = 1 << 9; 
        
        /* OIS1: Output idle state 1 (OC1 output) */
        pub const OIS1: u32 = 1 << 8; 
        pub const OIS_MASK: u32 = 0x7f << 8; 
        
        /* TI1S: TI1 selection */
        pub const TI1S: u32 = 1 << 7; 

        /* MMS[2:0]: Master mode selection */
        pub const MMS_RESET: u32 = 0x0 << 4; 
        pub const MMS_ENABLE: u32 = 0x1 << 4; 
        pub const MMS_UPDATE: u32 = 0x2 << 4; 
        pub const MMS_COMPARE_PULSE: u32 = 0x3 << 4; 
        pub const MMS_COMPARE_OC1REF: u32 = 0x4 << 4; 
        pub const MMS_COMPARE_OC2REF: u32 = 0x5 << 4; 
        pub const MMS_COMPARE_OC3REF: u32 = 0x6 << 4; 
        pub const MMS_COMPARE_OC4REF: u32 = 0x7 << 4; 
        pub const MMS_MASK: u32 = 0x7 << 4; 

        /* CCDS: Capture/compare DMA selection */
        pub const CCDS: u32 = 1 << 3; 

        /* CCUS: Capture/compare control update selection */
        pub const CCUS: u32 = 1 << 2;

        /* CCPC: Capture/compare preload control */
        pub const CCPC: u32 = 1 << 0; 
    }

    pub mod smcr {
        /* ETP: External trigger polarity */
        pub const ETP: u32 = 1 << 15; 

        /* ECE: External clock enable */
        pub const ECE: u32 = 1 << 14;

        /* ETPS[1:0]: External trigger prescaler */
        pub const ETPS_OFF: u32 = 0x0 << 12; 
        pub const ETPS_ETRP_DIV_2: u32 = 0x1 << 12; 
        pub const ETPS_ETRP_DIV_4: u32 = 0x2 << 12; 
        pub const ETPS_ETRP_DIV_8: u32 = 0x3 << 12; 
        pub const ETPS_MASK: u32 = 0x3 << 12; 

        /* ETF[3:0]: External trigger filter */
        pub const ETF_OFF: u32 = 0x0 << 8; 
        pub const ETF_CK_INT_N_2: u32 = 0x1 << 8; 
        pub const ETF_CK_INT_N_4: u32 = 0x2 << 8; 
        pub const ETF_CK_INT_N_8: u32 = 0x3 << 8; 
        pub const ETF_DTS_DIV_2_N_6: u32 = 0x4 << 8; 
        pub const ETF_DTS_DIV_2_N_8: u32 = 0x5 << 8; 
        pub const ETF_DTS_DIV_4_N_6: u32 = 0x6 << 8; 
        pub const ETF_DTS_DIV_4_N_8: u32 = 0x7 << 8; 
        pub const ETF_DTS_DIV_8_N_6: u32 = 0x8 << 8; 
        pub const ETF_DTS_DIV_8_N_8: u32 = 0x9 << 8; 
        pub const ETF_DTS_DIV_16_N_5: u32 = 0xA << 8; 
        pub const ETF_DTS_DIV_16_N_6: u32 = 0xB << 8; 
        pub const ETF_DTS_DIV_16_N_8: u32 = 0xC << 8; 
        pub const ETF_DTS_DIV_32_N_5: u32 = 0xD << 8; 
        pub const ETF_DTS_DIV_32_N_6: u32 = 0xE << 8; 
        pub const ETF_DTS_DIV_32_N_8: u32 = 0xF << 8; 
        pub const ETF_MASK: u32 = 0xF << 8; 

        /* MSM: Master/slave mode */
        pub const MSM: u32 = 1 << 7; 

        /* TS[2:0]: Trigger selection */

        /** Internal Trigger 0 (ITR0) */
        pub const TS_ITR0: u32 = 0x0 << 4; 
        /** Internal Trigger 1 (ITR1) */
        pub const TS_ITR1: u32 = 0x1 << 4; 
        /** Internal Trigger 2 (ITR2) */
        pub const TS_ITR2: u32 = 0x2 << 4; 
        /** Internal Trigger 3 (ITR3) */
        pub const TS_ITR3: u32 = 0x3 << 4; 
        /** TI1 Edge Detector (TI1F_ED) */
        pub const TS_TI1F_ED: u32 = 0x4 << 4; 
        /** Filtered Timer Input 1 (TI1FP1) */
        pub const TS_TI1FP1: u32 = 0x5 << 4; 
        /** Filtered Timer Input 2 (TI2FP2) */
        pub const TS_TI2FP2: u32 = 0x6 << 4; 
        /** External Trigger input (ETRF) */
        pub const TS_ETRF: u32 = 0x7 << 4; 
        pub const TS_MASK: u32 = 0x7 << 4; 

        /* SMS[2:0]: Slave mode selection */
        /** Slave mode disabled */
        pub const SMS_OFF: u32 = 0x0 << 0; 
        /** Encoder mode 1 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level. */

        pub const SMS_EM1: u32 = 0x1 << 0; 
        /** Encoder mode 2 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level. */

        pub const SMS_EM2: u32 = 0x2 << 0; 
        /** Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending
         *  on the level of the complementary input.
         */
        pub const SMS_EM3: u32 = 0x3 << 0; 
        /** Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the 
         * counter and generates an update of the registers. 
         */
        pub const SMS_RM: u32 = 0x4 << 0;

        /** Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. */
        pub const SMS_GM: u32 = 0x5 << 0;

        /**  Trigger Mode - The counter starts at a rising edge of the trigger TRGI. */
        pub const SMS_TM: u32 = 0x6 << 0; 
        /** External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter. */

        pub const SMS_ECM1: u32 = 0x7 << 0; 
        pub const SMS_MASK: u32 = 0x7 << 0; 
    }

    pub mod dier {
        /* TDE: Trigger DMA request enable */
        pub const TDE: u32 = 1 << 14; 

        /* COMDE: COM DMA request enable */
        pub const COMDE: u32 = 1 << 13;

        /* CC4DE: Capture/Compare 4 DMA request enable */
        pub const CC4DE: u32 = 1 << 12; 
        
        /* CC3DE: Capture/Compare 3 DMA request enable */
        pub const CC3DE: u32 = 1 << 11; 
        
        /* CC2DE: Capture/Compare 2 DMA request enable */
        pub const CC2DE: u32 = 1 << 10; 
        
        /* CC1DE: Capture/Compare 1 DMA request enable */
        pub const CC1DE: u32 = 1 << 9; 
        
        /* UDE: Update DMA request enable */
        pub const UDE: u32 = 1 << 8; 
        
        /* BIE: Break interrupt enable */
        pub const BIE: u32 = 1 << 7; 
        
        /* TIE: Trigger interrupt enable */
        pub const TIE: u32 = 1 << 6; 
        
        /* COMIE: COM interrupt enable */
        pub const COMIE: u32 = 1 << 5; 
        
        /* CC4IE: Capture/compare 4 interrupt enable */
        pub const CC4IE: u32 = 1 << 4; 
        
        /* CC3IE: Capture/compare 3 interrupt enable */
        pub const CC3IE: u32 = 1 << 3; 
        
        /* CC2IE: Capture/compare 2 interrupt enable */
        pub const CC2IE: u32 = 1 << 2; 
        
        /* CC1IE: Capture/compare 1 interrupt enable */
        pub const CC1IE: u32 = 1 << 1; 
        
        /* UIE: Update interrupt enable */
        pub const UIE: u32 = 1 << 0; 
    }

    pub mod sr {
        /* CC4OF: Capture/compare 4 overcapture flag */
        pub const CC4OF: u32 = 1 << 12; 
        /* CC3OF: Capture/compare 3 overcapture flag */
        pub const CC3OF: u32 = 1 << 11; 
        /* CC2OF: Capture/compare 2 overcapture flag */
        pub const CC2OF: u32 = 1 << 10; 
        /* CC1OF: Capture/compare 1 overcapture flag */
        pub const CC1OF: u32 = 1 << 9; 
        /* BIF: Break interrupt flag */
        pub const BIF: u32 = 1 << 7; 
        /* TIF: Trigger interrupt flag */
        pub const TIF: u32 = 1 << 6; 
        /* COMIF: COM interrupt flag */
        pub const COMIF: u32 = 1 << 5; 
        /* CC4IF: Capture/compare 4 interrupt flag */
        pub const CC4IF: u32 = 1 << 4; 
        /* CC3IF: Capture/compare 3 interrupt flag */
        pub const CC3IF: u32 = 1 << 3; 
        /* CC2IF: Capture/compare 2 interrupt flag */
        pub const CC2IF: u32 = 1 << 2; 
        /* CC1IF: Capture/compare 1 interrupt flag */
        pub const CC1IF: u32 = 1 << 1; 
        /* UIF: Update interrupt flag */
        pub const UIF: u32 = 1 << 0;
    }

    pub mod egr {
        /* BG: Break generation */
        pub const BG: u32 = 1 << 7;

        /* TG: Trigger generation */
        pub const TG: u32 = 1 << 6;

        /* COMG: Capture/compare control update generation */
        pub const COMG: u32 = 1 << 5;

        /* CC4G: Capture/compare 4 generation */
        pub const CC4G: u32 = 1 << 4;

        /* CC3G: Capture/compare 3 generation */
        pub const CC3G: u32 = 1 << 3;

        /* CC2G: Capture/compare 2 generation */
        pub const CC2G: u32 = 1 << 2;

        /* CC1G: Capture/compare 1 generation */
        pub const CC1G: u32 = 1 << 1;

        /* UG: Update generation */
        pub const UG: u32 = 1 << 0;
    }

    pub mod ccmr1 {
        /* OC2CE: Output compare 2 clear enable */
        pub const OC2CE: u32 = 1 << 15; 

        /* OC2M[2:0]: Output compare 2 mode */
        pub const OC2M_FROZEN: u32 = 0x0 << 12; 
        pub const OC2M_ACTIVE: u32 = 0x1 << 12; 
        pub const OC2M_INACTIVE: u32 = 0x2 << 12; 
        pub const OC2M_TOGGLE: u32 = 0x3 << 12; 
        pub const OC2M_FORCE_LOW: u32 = 0x4 << 12; 
        pub const OC2M_FORCE_HIGH: u32 = 0x5 << 12; 
        pub const OC2M_PWM1: u32 = 0x6 << 12; 
        pub const OC2M_PWM2: u32 = 0x7 << 12; 
        pub const OC2M_MASK: u32 = 0x7 << 12; 

        /* OC2PE: Output compare 2 preload enable */
        pub const OC2PE: u32 = 1 << 11; 

        /* OC2FE: Output compare 2 fast enable */
        pub const OC2FE: u32 = 1 << 10; 

        /* CC2S[1:0]: Capture/compare 2 selection */
        /* Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER). */
        pub const CC2S_OUT: u32 = 0x0 << 8; 
        pub const CC2S_IN_TI2: u32 = 0x1 << 8; 
        pub const CC2S_IN_TI1: u32 = 0x2 << 8; 
        pub const CC2S_IN_TRC: u32 = 0x3 << 8; 
        pub const CC2S_MASK: u32 = 0x3 << 8; 

        /* OC1CE: Output compare 1 clear enable */
        pub const OC1CE: u32 = 1 << 7; 

        /* OC1M[2:0]: Output compare 1 mode */
        pub const OC1M_FROZEN: u32 = 0x0 << 4; 
        pub const OC1M_ACTIVE: u32 = 0x1 << 4; 
        pub const OC1M_INACTIVE: u32 = 0x2 << 4; 
        pub const OC1M_TOGGLE: u32 = 0x3 << 4; 
        pub const OC1M_FORCE_LOW: u32 = 0x4 << 4; 
        pub const OC1M_FORCE_HIGH: u32 = 0x5 << 4; 
        pub const OC1M_PWM1: u32 = 0x6 << 4; 
        pub const OC1M_PWM2: u32 = 0x7 << 4; 
        pub const OC1M_MASK: u32 = 0x7 << 4; 

        /* OC1PE: Output compare 1 preload enable */
        pub const OC1PE: u32 = 1 << 3; 

        /* OC1FE: Output compare 1 fast enable */
        pub const OC1FE: u32 = 1 << 2; 

        /* CC1S[1:0]: Capture/compare 1 selection */
        /* Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER). */
        pub const CC1S_OUT: u32 = 0x0 << 0; 
        pub const CC1S_IN_TI2: u32 = 0x2 << 0; 
        pub const CC1S_IN_TI1: u32 = 0x1 << 0; 
        pub const CC1S_IN_TRC: u32 = 0x3 << 0; 
        pub const CC1S_MASK: u32 = 0x3 << 0; 

        /* --- Input capture mode --- */

        /* IC2F[3:0]: Input capture 2 filter */
        pub const IC2F_OFF: u32 = 0x0 << 12; 
        pub const IC2F_CK_INT_N_2: u32 = 0x1 << 12; 
        pub const IC2F_CK_INT_N_4: u32 = 0x2 << 12; 
        pub const IC2F_CK_INT_N_8: u32 = 0x3 << 12; 
        pub const IC2F_DTF_DIV_2_N_6: u32 = 0x4 << 12; 
        pub const IC2F_DTF_DIV_2_N_8: u32 = 0x5 << 12; 
        pub const IC2F_DTF_DIV_4_N_6: u32 = 0x6 << 12; 
        pub const IC2F_DTF_DIV_4_N_8: u32 = 0x7 << 12; 
        pub const IC2F_DTF_DIV_8_N_6: u32 = 0x8 << 12; 
        pub const IC2F_DTF_DIV_8_N_8: u32 = 0x9 << 12; 
        pub const IC2F_DTF_DIV_16_N_5: u32 = 0xA << 12; 
        pub const IC2F_DTF_DIV_16_N_6: u32 = 0xB << 12; 
        pub const IC2F_DTF_DIV_16_N_8: u32 = 0xC << 12; 
        pub const IC2F_DTF_DIV_32_N_5: u32 = 0xD << 12; 
        pub const IC2F_DTF_DIV_32_N_6: u32 = 0xE << 12; 
        pub const IC2F_DTF_DIV_32_N_8: u32 = 0xF << 12; 
        pub const IC2F_MASK: u32 = 0xF << 12; 

        /* IC2PSC[1:0]: Input capture 2 prescaler */
        pub const IC2PSC_OFF: u32 = 0x0 << 10; 
        pub const IC2PSC_2: u32 = 0x1 << 10; 
        pub const IC2PSC_4: u32 = 0x2 << 10; 
        pub const IC2PSC_8: u32 = 0x3 << 10; 
        pub const IC2PSC_MASK: u32 = 0x3 << 10; 

        /* IC1F[3:0]: Input capture 1 filter */
        pub const IC1F_OFF: u32 = 0x0 << 4; 
        pub const IC1F_CK_INT_N_2: u32 = 0x1 << 4; 
        pub const IC1F_CK_INT_N_4: u32 = 0x2 << 4; 
        pub const IC1F_CK_INT_N_8: u32 = 0x3 << 4; 
        pub const IC1F_DTF_DIV_2_N_6: u32 = 0x4 << 4; 
        pub const IC1F_DTF_DIV_2_N_8: u32 = 0x5 << 4; 
        pub const IC1F_DTF_DIV_4_N_6: u32 = 0x6 << 4; 
        pub const IC1F_DTF_DIV_4_N_8: u32 = 0x7 << 4; 
        pub const IC1F_DTF_DIV_8_N_6: u32 = 0x8 << 4; 
        pub const IC1F_DTF_DIV_8_N_8: u32 = 0x9 << 4; 
        pub const IC1F_DTF_DIV_16_N_5: u32 = 0xA << 4; 
        pub const IC1F_DTF_DIV_16_N_6: u32 = 0xB << 4; 
        pub const IC1F_DTF_DIV_16_N_8: u32 = 0xC << 4; 
        pub const IC1F_DTF_DIV_32_N_5: u32 = 0xD << 4; 
        pub const IC1F_DTF_DIV_32_N_6: u32 = 0xE << 4; 
        pub const IC1F_DTF_DIV_32_N_8: u32 = 0xF << 4; 
        pub const IC1F_MASK: u32 = 0xF << 4; 

        /* IC1PSC[1:0]: Input capture 1 prescaler */
        pub const IC1PSC_OFF: u32 = 0x0 << 2; 
        pub const IC1PSC_2: u32 = 0x1 << 2; 
        pub const IC1PSC_4: u32 = 0x2 << 2; 
        pub const IC1PSC_8: u32 = 0x3 << 2; 
        pub const IC1PSC_MASK: u32 = 0x3 << 2; 

    }

    pub mod ccmr2 {
        /* OC4CE: Output compare 4 clear enable */
        pub const OC4CE: u32 = 1 << 15; 

        /* OC4M[2:0]: Output compare 4 mode */
        pub const OC4M_FROZEN: u32 = 0x0 << 12; 
        pub const OC4M_ACTIVE: u32 = 0x1 << 12; 
        pub const OC4M_INACTIVE: u32 = 0x2 << 12; 
        pub const OC4M_TOGGLE: u32 = 0x3 << 12; 
        pub const OC4M_FORCE_LOW: u32 = 0x4 << 12; 
        pub const OC4M_FORCE_HIGH: u32 = 0x5 << 12; 
        pub const OC4M_PWM1: u32 = 0x6 << 12; 
        pub const OC4M_PWM2: u32 = 0x7 << 12; 
        pub const OC4M_MASK: u32 = 0x7 << 12; 

        /* OC4PE: Output compare 4 preload enable */
        pub const OC4PE: u32 = 1 << 11; 

        /* OC4FE: Output compare 4 fast enable */
        pub const OC4FE: u32 = 1 << 10; 

        /* CC4S[1:0]: Capture/compare 4 selection */
        /* Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER). */
        pub const CC4S_OUT: u32 = 0x0 << 8; 
        pub const CC4S_IN_TI4: u32 = 0x1 << 8; 
        pub const CC4S_IN_TI3: u32 = 0x2 << 8; 
        pub const CC4S_IN_TRC: u32 = 0x3 << 8; 
        pub const CC4S_MASK: u32 = 0x3 << 8; 

        /* OC3CE: Output compare 3 clear enable */
        pub const OC3CE: u32 = 1 << 7; 

        /* OC3M[2:0]: Output compare 3 mode */
        pub const OC3M_FROZEN: u32 = 0x0 << 4; 
        pub const OC3M_ACTIVE: u32 = 0x1 << 4; 
        pub const OC3M_INACTIVE: u32 = 0x2 << 4; 
        pub const OC3M_TOGGLE: u32 = 0x3 << 4; 
        pub const OC3M_FORCE_LOW: u32 = 0x4 << 4; 
        pub const OC3M_FORCE_HIGH: u32 = 0x5 << 4; 
        pub const OC3M_PWM1: u32 = 0x6 << 4; 
        pub const OC3M_PWM2: u32 = 0x7 << 4; 
        pub const OC3M_MASK: u32 = 0x7 << 4; 

        /* OC3PE: Output compare 3 preload enable */
        pub const OC3PE: u32 = 1 << 3; 

        /* OC3FE: Output compare 3 fast enable */
        pub const OC3FE: u32 = 1 << 2; 

        /* CC3S[1:0]: Capture/compare 3 selection */
        /* Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER). */
        pub const CC3S_OUT: u32 = 0x0 << 0; 
        pub const CC3S_IN_TI3: u32 = 0x1 << 0; 
        pub const CC3S_IN_TI4: u32 = 0x2 << 0; 
        pub const CC3S_IN_TRC: u32 = 0x3 << 0; 
        pub const CC3S_MASK: u32 = 0x3 << 0; 

        /* --- Input capture mode --- */

        /* IC4F[3:0]: Input capture 4 filter */
        pub const IC4F_OFF: u32 = 0x0 << 12; 
        pub const IC4F_CK_INT_N_2: u32 = 0x1 << 12; 
        pub const IC4F_CK_INT_N_4: u32 = 0x2 << 12; 
        pub const IC4F_CK_INT_N_8: u32 = 0x3 << 12; 
        pub const IC4F_DTF_DIV_2_N_6: u32 = 0x4 << 12; 
        pub const IC4F_DTF_DIV_2_N_8: u32 = 0x5 << 12; 
        pub const IC4F_DTF_DIV_4_N_6: u32 = 0x6 << 12; 
        pub const IC4F_DTF_DIV_4_N_8: u32 = 0x7 << 12; 
        pub const IC4F_DTF_DIV_8_N_6: u32 = 0x8 << 12; 
        pub const IC4F_DTF_DIV_8_N_8: u32 = 0x9 << 12; 
        pub const IC4F_DTF_DIV_16_N_5: u32 = 0xA << 12; 
        pub const IC4F_DTF_DIV_16_N_6: u32 = 0xB << 12; 
        pub const IC4F_DTF_DIV_16_N_8: u32 = 0xC << 12; 
        pub const IC4F_DTF_DIV_32_N_5: u32 = 0xD << 12; 
        pub const IC4F_DTF_DIV_32_N_6: u32 = 0xE << 12; 
        pub const IC4F_DTF_DIV_32_N_8: u32 = 0xF << 12; 
        pub const IC4F_MASK: u32 = 0xF << 12; 

        /* IC4PSC[1:0]: Input capture 4 prescaler */
        pub const IC4PSC_OFF: u32 = 0x0 << 10; 
        pub const IC4PSC_2: u32 = 0x1 << 10; 
        pub const IC4PSC_4: u32 = 0x2 << 10; 
        pub const IC4PSC_8: u32 = 0x3 << 10; 
        pub const IC4PSC_MASK: u32 = 0x3 << 10; 

        /* IC3F[3:0]: Input capture 3 filter */
        pub const IC3F_OFF: u32 = 0x0 << 4; 
        pub const IC3F_CK_INT_N_2: u32 = 0x1 << 4; 
        pub const IC3F_CK_INT_N_4: u32 = 0x2 << 4; 
        pub const IC3F_CK_INT_N_8: u32 = 0x3 << 4; 
        pub const IC3F_DTF_DIV_2_N_6: u32 = 0x4 << 4; 
        pub const IC3F_DTF_DIV_2_N_8: u32 = 0x5 << 4; 
        pub const IC3F_DTF_DIV_4_N_6: u32 = 0x6 << 4; 
        pub const IC3F_DTF_DIV_4_N_8: u32 = 0x7 << 4; 
        pub const IC3F_DTF_DIV_8_N_6: u32 = 0x8 << 4; 
        pub const IC3F_DTF_DIV_8_N_8: u32 = 0x9 << 4; 
        pub const IC3F_DTF_DIV_16_N_5: u32 = 0xA << 4; 
        pub const IC3F_DTF_DIV_16_N_6: u32 = 0xB << 4; 
        pub const IC3F_DTF_DIV_16_N_8: u32 = 0xC << 4; 
        pub const IC3F_DTF_DIV_32_N_5: u32 = 0xD << 4; 
        pub const IC3F_DTF_DIV_32_N_6: u32 = 0xE << 4; 
        pub const IC3F_DTF_DIV_32_N_8: u32 = 0xF << 4; 
        pub const IC3F_MASK: u32 = 0xF << 4; 

        /* IC3PSC[1:0]: Input capture 3 prescaler */
        pub const IC3PSC_OFF: u32 = 0x0 << 2; 
        pub const IC3PSC_2: u32 = 0x1 << 2; 
        pub const IC3PSC_4: u32 = 0x2 << 2; 
        pub const IC3PSC_8: u32 = 0x3 << 2; 
        pub const IC3PSC_MASK: u32 = 0x3 << 2; 

    }

    pub mod ccer {
        /* CC4NP: Capture/compare 4 complementary output polarity */
        pub const CC4NP: u32 = 1 << 15; 
        
        /* CC4NE: Capture/compare 4 complementary output enable */
        pub const CC4NE: u32 = 1 << 14;

        /* CC4P: Capture/compare 4 output polarity */
        pub const CC4P: u32 = 1 << 13; 
        
        /* CC4E: Capture/compare 4 output enable */
        pub const CC4E: u32 = 1 << 12; 
        
        /* CC3NP: Capture/compare 3 complementary output polarity */
        pub const CC3NP: u32 = 1 << 11; 
        
        /* CC3NE: Capture/compare 3 complementary output enable */
        pub const CC3NE: u32 = 1 << 10; 
        
        /* CC3P: Capture/compare 3 output polarity */
        pub const CC3P: u32 = 1 << 9; 
        
        /* CC3E: Capture/compare 3 output enable */
        pub const CC3E: u32 = 1 << 8; 
        
        /* CC2NP: Capture/compare 2 complementary output polarity */
        pub const CC2NP: u32 = 1 << 7; 
        
        /* CC2NE: Capture/compare 2 complementary output enable */
        pub const CC2NE: u32 = 1 << 6; 
        
        /* CC2P: Capture/compare 2 output polarity */
        pub const CC2P: u32 = 1 << 5; 
        
        /* CC2E: Capture/compare 2 output enable */
        pub const CC2E: u32 = 1 << 4; 
        
        /* CC1NP: Capture/compare 1 complementary output polarity */
        pub const CC1NP: u32 = 1 << 3; 
        
        /* CC1NE: Capture/compare 1 complementary output enable */
        pub const CC1NE: u32 = 1 << 2; 
        
        /* CC1P: Capture/compare 1 output polarity */
        pub const CC1P: u32 = 1 << 1; 
        
        /* CC1E: Capture/compare 1 output enable */
        pub const CC1E: u32 = 1 << 0; 
    }

    pub mod bdtr {
        /* MOE: Main output enable */
        pub const MOE: u32 = 1 << 15; 
        
        /* AOE: Automatic output enable */
        pub const AOE: u32 = 1 << 14; 
        
        /* BKP: Break polarity */
        pub const BKP: u32 = 1 << 13; 
        
        /* BKE: Break enable */
        pub const BKE: u32 = 1 << 12; 
        
        /* OSSR: Off-state selection of run mode */
        pub const OSSR: u32 = 1 << 11; 
        
        /* OSSI: Off-state selection of idle mode */
        pub const OSSI: u32 = 1 << 10; 
        
        /* LOCK[1:0]: Lock configuration */
        pub const LOCK_OFF: u32     = 0x0 << 8; 
        pub const LOCK_LEVEL_1: u32 = 0x1 << 8; 
        pub const LOCK_LEVEL_2: u32 = 0x2 << 8; 
        pub const LOCK_LEVEL_3: u32 = 0x3 << 8; 
        pub const LOCK_MASK: u32    = 0x3 << 8; 
        
        /* DTG[7:0]: Dead-time generator set-up */
        pub const DTG_MASK: u32 = 0x00FF;

        /* --- TIMx_DCR values ----------------------------------------------------- */

        /* DBL[4:0]: DMA burst length */
        pub const DBL_MASK: u32 = 0x1F << 8; 

        /* DBA[4:0]: DMA base address */
        pub const DBA_MASK: u32 = 0x1F << 0;
    }

    pub mod or {
        /** Internal Trigger 1 remapped to timer 8 trigger out */
        pub const ITR1_RMP_TIM8_TRGOU: u32 = 0x0 << 10;
        
        /** Internal Trigger 1 remapped to PTP trigger out */
        pub const ITR1_RMP_PTP: u32 = 0x1 << 10;
        
        /** Internal Trigger 1 remapped to USB OTG FS SOF */
        pub const ITR1_RMP_OTG_FS_SOF: u32 = 0x2 << 10;
        
        /** Internal Trigger 1 remapped to USB OTG HS SOF */
        pub const ITR1_RMP_OTG_HS_SOF: u32 = 0x3 << 10;
        pub const ITR1_RMP_MASK: u32 = 0x3 << 10;

        /** Internal Trigger 4 remapped to GPIO (see reference manual) */
        pub const ITR4_RMP_GPIO: u32 = 0x0 << 6;
        
        /** Internal Trigger 4 remapped to LSI internal clock */
        pub const ITR4_RMP_LSI: u32 = 0x1 << 6;
        
        /** Internal Trigger 4 remapped to LSE internal clock */
        pub const ITR4_RMP_LSE: u32 = 0x2 << 6;
        
        /** Internal Trigger 4 remapped to RTC output event */
        pub const ITR4_RMP_RTC: u32 = 0x3 << 6;
        
        pub const ITR4_RMP_MASK: u32 = 0x3 << 6;
    }
}

/** Input Capture input polarity */
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputCapturePolarity {
	Rising,
	Falling,
	Both,
}

/** Output Compare channel designators */
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OutputCompareChannel {
	OChannel1 = 0,
	OChannel1N,
	OChannel2,
	OChannel2N,
	OChannel3,
	OChannel3N,
	OChannel4,
}

/** Output Compare mode designators */
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OutputCompareMode {
	Frozen,
	Active,
	Inactive,
	Toggle,
	ForceLow,
	ForceHigh,
	PWM1,
	PWM2,
}

/** Input Capture channel designators */
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputCaptureChannel {
	Channel1,
	Channel2,
	Channel3,
	Channel4,
}

/** Input Capture input filter. The frequency used to sample the
input and the number of events needed to validate an output transition.

TIM_IC_CK_INT_N_x No division from the Deadtime and Sampling Clock frequency
(DTF), filter length x
TIM_IC_DTF_DIV_y_N_x Division by y from the DTF, filter length x
 */
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum InputCaptureFilter {
	OFF,
	CK_INT_N_2,
	CK_INT_N_4,
	CK_INT_N_8,
	DTF_DIV_2_N_6,
	DTF_DIV_2_N_8,
	DTF_DIV_4_N_6,
	DTF_DIV_4_N_8,
	DTF_DIV_8_N_6,
	DTF_DIV_8_N_8,
	DTF_DIV_16_N_5,
	DTF_DIV_16_N_6,
	DTF_DIV_16_N_8,
	DTF_DIV_32_N_5,
	DTF_DIV_32_N_6,
	DTF_DIV_32_N_8,
}

/** Input Capture input prescaler.

InputCapturePrescaler_x Input capture is done every x events*/
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputCapturePrescaler {
	PrescalerOff,
	Prescaler2,
	Prescaler4,
	Prescaler8,
}

/** Input Capture input source.

The direction of the channel (input/output) as well as the input used. */
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputCaptureSource {
	Out   = 0,
	InTI1 = 1,
	InTI2 = 2,
	InTRC = 3,
	InTI3 = 5,
	InTI4 = 6,
}

/** Slave external trigger polarity */
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExternalTriggerPolarity {
	Rising,
	Falling,
}

pub fn reset(timer: Timer) {
    match timer {
        TIM1  => rcc::reset_pulse(rcc::Peripheral::TIM1),
        TIM2  => rcc::reset_pulse(rcc::Peripheral::TIM2),
        TIM3  => rcc::reset_pulse(rcc::Peripheral::TIM3),
        TIM4  => rcc::reset_pulse(rcc::Peripheral::TIM4),
        TIM5  => rcc::reset_pulse(rcc::Peripheral::TIM5),
        TIM6  => rcc::reset_pulse(rcc::Peripheral::TIM6),
        TIM7  => rcc::reset_pulse(rcc::Peripheral::TIM7),
        TIM8  => rcc::reset_pulse(rcc::Peripheral::TIM8),
        TIM9  => rcc::reset_pulse(rcc::Peripheral::TIM9),
        TIM10 => rcc::reset_pulse(rcc::Peripheral::TIM10),
        TIM11 => rcc::reset_pulse(rcc::Peripheral::TIM11),
        TIM12 => rcc::reset_pulse(rcc::Peripheral::TIM12),
        TIM13 => rcc::reset_pulse(rcc::Peripheral::TIM13),
        TIM14 => rcc::reset_pulse(rcc::Peripheral::TIM14),
        Timer { .. } => { } 
    }
}

pub fn enable_update_irq(timer: Timer) {
	timer.dier.check(flags::dier::UIE);
}


pub fn disable_update_irq(timer: Timer) {
	timer.dier.uncheck(flags::dier::UIE);
}

/*---------------------------------------------------------------------------*/
/** @brief Enable Interrupts for a Timer

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] irq Unsigned int32. @ref tim_irq_enable. Logical OR of all interrupt
enable bits to be set
*/

pub fn enable_irq(timer: Timer, irq: u32) {
	timer.dier.check(irq);
}

/*---------------------------------------------------------------------------*/
/** @brief Disable Interrupts for a Timer.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] irq Unsigned int32. @ref tim_irq_enable. Logical OR of all interrupt
enable bits to be cleared
*/

pub fn disable_irq(timer: Timer, irq: u32) {
    timer.dier.uncheck(irq);
}

/*---------------------------------------------------------------------------*/
/** @brief Return Interrupt Source.

Returns true if the specified interrupt flag (UIF, TIF or CCxIF, with BIF or
COMIF for advanced timers) was set and the interrupt was enabled. If the
specified flag is not an interrupt flag, the function returns false.

@todo Timers 6-7, 9-14 have fewer interrupts, but invalid flags are not caught
here.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] flag Unsigned int32. Status register flag  @ref flags::sr::values.
@returns boolean: flag set.
*/

pub fn interrupt_source(timer: Timer, flag: u32) -> bool {
    /* flag not set or interrupt disabled or not an interrupt source */
	if ((timer.sr.get() & timer.dier.get() & flag) == 0) ||
		(flag > flags::sr::BIF) {

		return false;
	}

    if (flag == flags::sr::BIF) || (flag == flags::sr::COMIF) {
		return timer.is_advanced();
	}

    return true;
}

pub fn is_update_flag_checked(timer: Timer) -> bool {
    timer.sr.test(flags::sr::UIF)
}

pub fn clear_update_flag(timer: Timer) {
    timer.sr.set(!flags::sr::UIF);
}

/*---------------------------------------------------------------------------*/
/** @brief Read a Status Flag.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] flag Unsigned int32. Status register flag  @ref flags::sr::values.
@returns boolean: flag set.
*/

pub fn get_flag(timer: Timer, flag: u32) -> bool {
	timer.sr.test(flag)
}

/*---------------------------------------------------------------------------*/
/** @brief Clear a Status Flag.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] flag Unsigned int32. @ref flags::sr::values. Status register flag.
*/

pub fn clear_flag(timer: Timer, flag: u32) {
	timer.sr.set(!flag);
}

/*---------------------------------------------------------------------------*/
/** @brief Set the Timer Mode.

The modes are:

@li Clock divider fn (to form the sampling clock for the input filters,
and the dead-time clock in the advanced timers 1 and 8) -> ratio @li Edge/centre alignment
@li Count direction

The alignment and count direction are effective only for timers 1 to 5 and 8
while the clock divider ratio is effective for all timers except 6,7
The remaining timers are limited hardware timers which do not support these mode
settings.

@note: When center alignment mode is selected, count direction is controlled by
hardware and cannot be written. The count direction setting has no effect
in this case.

@param[in] Timer struct. Timer register address base @ref
tim_reg_fn (TIM1, TIM2 ... TIM5, TIM8) -> base @param[in] clock_div Unsigned int32. Clock Divider Ratio in bits 8,9: @ref
tim_x_cr1_cdr
@param[in] alignment Unsigned int32. Alignment bits in 5,6: @ref tim_x_cr1_cms
@param[in] direction Unsigned int32. Count direction in bit 4,: @ref
tim_x_cr1_dir
*/

pub fn set_mode(timer: Timer, clock_div: u32, alignment: u32, direction: u32) {
	timer.cr1.uncheck(flags::cr1::CKD_CK_INT_MASK | flags::cr1::CMS_MASK | flags::cr1::DIR_DOWN);
	timer.cr1.check(clock_div | alignment | direction);
}

/*---------------------------------------------------------------------------*/
/** @brief Set Input Filter and Dead-time Clock Divider Ratio.

This forms the sampling clock for the input filters and the dead-time clock
in the advanced timers 1 and 8, by division from the timer clock.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] clock_div Unsigned int32. Clock Divider Ratio in bits 8,9: @ref
tim_x_cr1_cdr
*/

pub fn set_clock_division(timer: Timer, clock_div: u32) {
    timer.cr1.uncheck(flags::cr1::CKD_CK_INT_MASK);
    timer.cr1.check(clock_div & flags::cr1::CKD_CK_INT_MASK);
}

/*---------------------------------------------------------------------------*/
/** @brief Enable Auto-Reload Buffering.

During counter operation this causes the counter to be loaded from its
auto-reload register only at the next update event.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn enable_preload(timer: Timer) {
	timer.cr1.check(flags::cr1::ARPE);
}

/*---------------------------------------------------------------------------*/
/** @brief Disable Auto-Reload Buffering.

This causes the counter to be loaded immediately with a new count value when the
auto-reload register is written, so that the new value becomes effective for the
current count cycle rather than for the cycle following an update event.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn disable_preload(timer: Timer) {
	timer.cr1.uncheck(flags::cr1::ARPE);
}

/*---------------------------------------------------------------------------*/
/** @brief Specify the counter alignment mode.

The mode can be edge aligned or centered.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] alignment Unsigned int32. Alignment bits in 5,6: @ref tim_x_cr1_cms
*/

pub fn set_alignment(timer: Timer, alignment: u32) {
    timer.cr1.uncheck(flags::cr1::CMS_MASK);
    timer.cr1.check(alignment & flags::cr1::CMS_MASK);
}

/*---------------------------------------------------------------------------*/
/** @brief Set the Timer to Count Up.

This has no effect if the timer is set to center aligned.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn direction_up(timer: Timer) {
    timer.cr1.uncheck(flags::cr1::DIR_DOWN);
}

/*---------------------------------------------------------------------------*/
/** @brief Set the Timer to Count Down.

This has no effect if the timer is set to center aligned.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn direction_down(timer: Timer) {
    timer.cr1.check(flags::cr1::DIR_DOWN);
}

/*---------------------------------------------------------------------------*/
/** @brief Enable the Timer for One Cycle and Stop.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn one_shot_mode(timer: Timer) {
    timer.cr1.check(flags::cr1::OPM);
}

/*---------------------------------------------------------------------------*/
/** @brief Enable the Timer to Run Continuously.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn continuous_mode(timer: Timer) {
    timer.cr1.uncheck(flags::cr1::OPM);
}

/*---------------------------------------------------------------------------*/
/** @brief Set the Timer to Generate Update IRQ or DMA on any Event.

The events which will generate an interrupt or DMA request can be
@li a counter underflow/overflow,
@li a forced update,
@li an event from the slave mode controller.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn update_on_any(timer: Timer) {
    timer.cr1.uncheck(flags::cr1::URS);
}

/*---------------------------------------------------------------------------*/
/** @brief Set the Timer to Generate Update IRQ or DMA only from Under/Overflow
Events.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn update_on_overflow(timer: Timer) {
    timer.cr1.check(flags::cr1::URS);
}

/*---------------------------------------------------------------------------*/
/** @brief Enable Timer Update Events.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn enable_update_event(timer: Timer) {
    timer.cr1.uncheck(flags::cr1::UDIS);
}

/*---------------------------------------------------------------------------*/
/** @brief Disable Timer Update Events.

Update events are not generated and the shadow registers keep their values.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn disable_update_event(timer: Timer) {
	timer.cr1.check(flags::cr1::UDIS);
}

/*---------------------------------------------------------------------------*/
/** @brief Enable the timer to start counting.

This should be called after the timer initial configuration has been completed.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn enable_counter(timer: Timer) {
    timer.cr1.check(flags::cr1::CEN);
}

/*---------------------------------------------------------------------------*/
/** @brief Stop the timer from counting.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn disable_counter(timer: Timer) {
	timer.cr1.uncheck(flags::cr1::CEN);
}

/*---------------------------------------------------------------------------*/
/** @brief Set Timer Output Idle States High.

This determines the value of the timer output compare when it enters idle state.

@sa @ref timer_set_oc_idle_state_set

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] outputs Unsigned int32. Timer Output Idle State Controls @ref
tim_x_cr2_ois. If several settings are to be made, use the logical OR of the
output control values.
*/

pub fn set_output_idle_state(timer: Timer, outputs: u32) {
    if timer.is_advanced() {
        timer.cr2.check(outputs & flags::cr2::OIS_MASK);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Set Timer Output Idle States Low.

This determines the value of the timer output compare when it enters idle state.

@sa @ref timer_set_oc_idle_state_unset

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] outputs Unsigned int32. Timer Output Idle State Controls @ref
tim_x_cr2_ois
*/

pub fn reset_output_idle_state(timer: Timer, outputs: u32) {
    if timer.is_advanced() {
        timer.cr2.uncheck(outputs & flags::cr2::OIS_MASK);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Set Timer 1 Input to XOR of Three Channels.

The first timer capture input is formed from the XOR of the first three timer
input channels 1, 2, 3.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn set_ti1_ch123_xor(timer: Timer) {
    timer.cr2.check(flags::cr2::TI1S);
}

/*---------------------------------------------------------------------------*/
/** @brief Set Timer 1 Input to Channel 1.

The first timer capture input is taken from the timer input channel 1 only.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn set_ti1_ch1(timer: Timer) {
	timer.cr2.uncheck(flags::cr2::TI1S);
}

/*---------------------------------------------------------------------------*/
/** @brief Set the Master Mode

This sets the Trigger Output TRGO for synchronizing with slave timers or
passing as an internal trigger to the ADC or DAC.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] mode Unsigned int32. Master Mode @ref tim_mastermode
*/

pub fn set_master_mode(timer: Timer, mode: u32) {
    timer.cr2.uncheck(flags::cr2::MMS_MASK);
    timer.cr2.check(mode);
}

/*---------------------------------------------------------------------------*/
/** @brief Set Timer DMA Requests on Capture/Compare Events.

Capture/compare events will cause DMA requests to be generated.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn set_dma_on_compare_event(timer: Timer) {
    timer.cr2.uncheck(flags::cr2::CCDS);
}

/*---------------------------------------------------------------------------*/
/** @brief Set Timer DMA Requests on Update Events.

Update events will cause DMA requests to be generated.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn set_dma_on_update_event(timer: Timer) {
    timer.cr2.check(flags::cr2::CCDS);
}

/*---------------------------------------------------------------------------*/
/** @brief Enable Timer Capture/Compare Control Update with Trigger.

If the capture/compare control bits CCxE, CCxNE and OCxM are set to be
preloaded, they are updated by software generating the COMG event (@ref
timer_generate_event) or when a rising edge occurs on the trigger input TRGI.

@note This setting is only valid for the advanced timer channels with
complementary outputs.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn enable_compare_control_update_on_trigger(timer: Timer) {
    if timer.is_advanced() {
        timer.cr2.check(flags::cr2::CCUS);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Disable Timer Capture/Compare Control Update with Trigger.

If the capture/compare control bits CCxE, CCxNE and OCxM are set to be
preloaded, they are updated by software generating the COMG event (@ref
timer_generate_event).

@note This setting is only valid for the advanced timer channels with
complementary outputs.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn disable_compare_control_update_on_trigger(timer: Timer) {
    if timer.is_advanced() {
        timer.cr2.uncheck(flags::cr2::CCUS);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Enable Timer Capture/Compare Control Preload.

The capture/compare control bits CCxE, CCxNE and OCxM are set to be preloaded
when a COM event occurs.

@note This setting is only valid for the advanced timer channels with
complementary outputs.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn enable_preload_complementry_enable_bits(timer: Timer) {
    if timer.is_advanced() {
        timer.cr2.check(flags::cr2::CCPC);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Disable Timer Capture/Compare Control Preload.

The capture/compare control bits CCxE, CCxNE and OCxM preload is disabled.

@note This setting is only valid for the advanced timer channels with
complementary outputs.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
*/

pub fn disable_preload_complementry_enable_bits(timer: Timer) {
    if timer.is_advanced() {
        timer.cr2.uncheck(flags::cr2::CCPC);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Set the Value for the Timer Prescaler.

The timer clock is prescaled by the 16 bit scale value plus 1.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] value Unsigned int32. Prescaler values 0...0xFFFF.
*/

pub fn set_prescaler(timer: Timer, value: u32) {
    timer.psc.set(value);
}

/*---------------------------------------------------------------------------*/
/** @brief Set the Value for the Timer Repetition Counter.

A timer update event is generated only after the specified number of repeat
count cycles have been completed.

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] value Unsigned int32. Repetition values 0...0xFF.
*/

pub fn set_repetition_counter(timer: Timer, value: u32) {
    if timer.is_advanced() {
        timer.rcr.set(value);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Timer Set Period

Specify the timer period in the auto-reload register.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] period Unsigned int32. Period in counter clock ticks.
*/

pub fn set_period(timer: Timer, period: u32) {
    timer.arr.set(period);
}

/*---------------------------------------------------------------------------*/
/** @brief Timer Enable the Output Compare Clear Function

When this is enabled, the output compare signal is cleared when a high is
detected on the external trigger input. This works in the output compare and
PWM modes only (not forced mode).
The output compare signal remains off until the next update event.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] oc_id enum ::OutputCompareChannel OC channel designators
		TIM_OCx where x=1..4, TIM_OCxN where x=1..3 (no action taken)
*/

pub fn enable_oc_clear(timer: Timer, oc_id: OutputCompareChannel) {
    match oc_id {
        OutputCompareChannel::OChannel1 => timer.ccmr1.check(flags::ccmr1::OC1CE),
        OutputCompareChannel::OChannel2 => timer.ccmr1.check(flags::ccmr1::OC2CE),
        OutputCompareChannel::OChannel3 => timer.ccmr2.check(flags::ccmr2::OC3CE),
        OutputCompareChannel::OChannel4 => timer.ccmr2.check(flags::ccmr2::OC4CE),
        _ => { }
	};
}

/*---------------------------------------------------------------------------*/
/** @brief Timer Disable the Output Compare Clear Function

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] oc_id enum ::OutputCompareChannel OC channel designators
		TIM_OCx where x=1..4, TIM_OCxN where x=1..3 (no action taken)
*/

pub fn disable_oc_clear(timer: Timer, oc_id: OutputCompareChannel) {
    match oc_id {
        OutputCompareChannel::OChannel1 => timer.ccmr1.uncheck(flags::ccmr1::OC1CE),
        OutputCompareChannel::OChannel2 => timer.ccmr1.uncheck(flags::ccmr1::OC2CE),
        OutputCompareChannel::OChannel3 => timer.ccmr2.uncheck(flags::ccmr2::OC3CE),
        OutputCompareChannel::OChannel4 => timer.ccmr2.uncheck(flags::ccmr2::OC4CE),
        _ => { }
	};
}

/*---------------------------------------------------------------------------*/
/** @brief Timer Enable the Output Compare Fast Mode

When this is enabled, the output compare signal is forced to the compare state
by a trigger input, independently of the compare match. This speeds up the
setting of the output compare to 3 clock cycles as opposed to at least 5 in the
slow mode. This works in the PWM1 and PWM2 modes only.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] oc_id enum ::OutputCompareChannel OC channel designators
		TIM_OCx where x=1..4, TIM_OCxN where x=1..3 (no action taken)
*/

pub fn set_oc_fast_mode(timer: Timer, oc_id: OutputCompareChannel) {
	match oc_id {
        OutputCompareChannel::OChannel1 => timer.ccmr1.check(flags::ccmr1::OC1FE),
        OutputCompareChannel::OChannel2 => timer.ccmr1.check(flags::ccmr1::OC2FE),
        OutputCompareChannel::OChannel3 => timer.ccmr2.check(flags::ccmr2::OC3FE),
        OutputCompareChannel::OChannel4 => timer.ccmr2.check(flags::ccmr2::OC4FE),
        _ => { }
	};
}

/*---------------------------------------------------------------------------*/
/** @brief Timer Enable the Output Compare Slow Mode

This disables the fast compare mode and the output compare depends on the
counter and compare register values.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] oc_id enum ::OutputCompareChannel OC channel designators
		TIM_OCx where x=1..4, TIM_OCxN where x=1..3 (no action taken)
*/

pub fn set_oc_slow_mode(timer: Timer, oc_id: OutputCompareChannel) {
    match oc_id {
        OutputCompareChannel::OChannel1 => timer.ccmr1.uncheck(flags::ccmr1::OC1FE),
        OutputCompareChannel::OChannel2 => timer.ccmr1.uncheck(flags::ccmr1::OC2FE),
        OutputCompareChannel::OChannel3 => timer.ccmr2.uncheck(flags::ccmr2::OC3FE),
        OutputCompareChannel::OChannel4 => timer.ccmr2.uncheck(flags::ccmr2::OC4FE),
        _ => { }
	};
}

/*---------------------------------------------------------------------------*/
/** @brief Timer Set Output Compare Mode

Specifies how the comparator output will respond to a compare match. The mode
can be:
@li Frozen - the output does not respond to a match.
@li Active - the output assumes the active state on the first match.
@li Inactive - the output assumes the inactive state on the first match.
@li Toggle - The output switches between active and inactive states on each
match.
@li Force inactive. The output is forced low regardless of the compare state.
@li Force active. The output is forced high regardless of the compare state.
@li PWM1 - The output is active when the counter is less than the compare
register contents and inactive otherwise.
@li PWM2 - The output is inactive when the counter is less than the compare
register contents and active otherwise.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] oc_id enum ::OutputCompareChannel OC channel designators
		TIM_OCx where x=1..4, TIM_OCxN where x=1..3 (no action taken)
@param[in] oc_mode enum ::OutputCompareMode. OC mode designators.
		OutputCompareMode::Frozen, OutputCompareMode::Active, OutputCompareMode::Inactive,
		OutputCompareMode::Toggle, OutputCompareMode::ForceLow, OutputCompareMode::ForceHigh,
		OutputCompareMode::PWM1, OutputCompareMode::PWM2
*/

pub fn set_oc_mode(timer: Timer, oc_id: OutputCompareChannel, oc_mode: OutputCompareMode) {
	match oc_id {
        OutputCompareChannel::OChannel1 => {
            timer.ccmr1.uncheck(flags::ccmr1::CC1S_MASK);
            timer.ccmr1.check(flags::ccmr1::CC1S_OUT);
            timer.ccmr1.uncheck(flags::ccmr1::OC1M_MASK);

            match oc_mode {
                OutputCompareMode::Frozen => timer.ccmr1.check(flags::ccmr1::OC1M_FROZEN),
                OutputCompareMode::Active => timer.ccmr1.check(flags::ccmr1::OC1M_ACTIVE),
                OutputCompareMode::Inactive => timer.ccmr1.check(flags::ccmr1::OC1M_INACTIVE),
                OutputCompareMode::Toggle => timer.ccmr1.check(flags::ccmr1::OC1M_TOGGLE),
                OutputCompareMode::ForceLow => timer.ccmr1.check(flags::ccmr1::OC1M_FORCE_LOW),
                OutputCompareMode::ForceHigh => timer.ccmr1.check(flags::ccmr1::OC1M_FORCE_HIGH),
                OutputCompareMode::PWM1 => timer.ccmr1.check(flags::ccmr1::OC1M_PWM1),
                OutputCompareMode::PWM2 => timer.ccmr1.check(flags::ccmr1::OC1M_PWM2),
            }
        },

        OutputCompareChannel::OChannel2 => {
            timer.ccmr1.uncheck(flags::ccmr1::CC2S_MASK);
            timer.ccmr1.check(flags::ccmr1::CC2S_OUT);
            timer.ccmr1.uncheck(flags::ccmr1::OC2M_MASK);

            match oc_mode {
                OutputCompareMode::Frozen => timer.ccmr1.check(flags::ccmr1::OC2M_FROZEN),
                OutputCompareMode::Active => timer.ccmr1.check(flags::ccmr1::OC2M_ACTIVE),
                OutputCompareMode::Inactive => timer.ccmr1.check(flags::ccmr1::OC2M_INACTIVE),
                OutputCompareMode::Toggle => timer.ccmr1.check(flags::ccmr1::OC2M_TOGGLE),
                OutputCompareMode::ForceLow => timer.ccmr1.check(flags::ccmr1::OC2M_FORCE_LOW),
                OutputCompareMode::ForceHigh => timer.ccmr1.check(flags::ccmr1::OC2M_FORCE_HIGH),
                OutputCompareMode::PWM1 => timer.ccmr1.check(flags::ccmr1::OC2M_PWM1),
                OutputCompareMode::PWM2 => timer.ccmr1.check(flags::ccmr1::OC2M_PWM2),
            }
        },

        OutputCompareChannel::OChannel3 => {
            timer.ccmr2.uncheck(flags::ccmr2::CC3S_MASK);
            timer.ccmr2.check(flags::ccmr2::CC3S_OUT);
            timer.ccmr2.uncheck(flags::ccmr2::OC3M_MASK);

            match oc_mode {
                OutputCompareMode::Frozen => timer.ccmr2.check(flags::ccmr2::OC3M_FROZEN),
                OutputCompareMode::Active => timer.ccmr2.check(flags::ccmr2::OC3M_ACTIVE),
                OutputCompareMode::Inactive => timer.ccmr2.check(flags::ccmr2::OC3M_INACTIVE),
                OutputCompareMode::Toggle => timer.ccmr2.check(flags::ccmr2::OC3M_TOGGLE),
                OutputCompareMode::ForceLow => timer.ccmr2.check(flags::ccmr2::OC3M_FORCE_LOW),
                OutputCompareMode::ForceHigh => timer.ccmr2.check(flags::ccmr2::OC3M_FORCE_HIGH),
                OutputCompareMode::PWM1 => timer.ccmr2.check(flags::ccmr2::OC3M_PWM1),
                OutputCompareMode::PWM2 => timer.ccmr2.check(flags::ccmr2::OC3M_PWM2),
            }
        },

        OutputCompareChannel::OChannel4 => {
            timer.ccmr2.uncheck(flags::ccmr2::CC4S_MASK);
            timer.ccmr2.check(flags::ccmr2::CC4S_OUT);
            timer.ccmr2.uncheck(flags::ccmr2::OC4M_MASK);

            match oc_mode {
                OutputCompareMode::Frozen => timer.ccmr2.check(flags::ccmr2::OC4M_FROZEN),
                OutputCompareMode::Active => timer.ccmr2.check(flags::ccmr2::OC4M_ACTIVE),
                OutputCompareMode::Inactive => timer.ccmr2.check(flags::ccmr2::OC4M_INACTIVE),
                OutputCompareMode::Toggle => timer.ccmr2.check(flags::ccmr2::OC4M_TOGGLE),
                OutputCompareMode::ForceLow => timer.ccmr2.check(flags::ccmr2::OC4M_FORCE_LOW),
                OutputCompareMode::ForceHigh => timer.ccmr2.check(flags::ccmr2::OC4M_FORCE_HIGH),
                OutputCompareMode::PWM1 => timer.ccmr2.check(flags::ccmr2::OC4M_PWM1),
                OutputCompareMode::PWM2 => timer.ccmr2.check(flags::ccmr2::OC4M_PWM2),
            }
        },
        
        _ => {}
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Timer Enable the Output Compare Preload Register

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] oc_id enum ::OutputCompareChannel OC channel designators
		TIM_OCx where x=1..4, TIM_OCxN where x=1..3 (no action taken)
*/

pub fn enable_oc_preload(timer: Timer, oc_id: OutputCompareChannel) {
    match oc_id {
        OutputCompareChannel::OChannel1 => timer.ccmr1.check(flags::ccmr1::OC1PE),
        OutputCompareChannel::OChannel2 => timer.ccmr1.check(flags::ccmr1::OC2PE),
        OutputCompareChannel::OChannel3 => timer.ccmr2.check(flags::ccmr2::OC3PE),
        OutputCompareChannel::OChannel4 => timer.ccmr2.check(flags::ccmr2::OC4PE),
        _ => { }
	};
}

/*---------------------------------------------------------------------------*/
/** @brief Timer Disable the Output Compare Preload Register

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] oc_id enum ::OutputCompareChannel OC channel designators
		TIM_OCx where x=1..4, TIM_OCxN where x=1..3 (no action)
*/

pub fn disable_oc_preload(timer: Timer, oc_id: OutputCompareChannel) {
    match oc_id {
        OutputCompareChannel::OChannel1 => timer.ccmr1.uncheck(flags::ccmr1::OC1PE),
        OutputCompareChannel::OChannel2 => timer.ccmr1.uncheck(flags::ccmr1::OC2PE),
        OutputCompareChannel::OChannel3 => timer.ccmr2.uncheck(flags::ccmr2::OC3PE),
        OutputCompareChannel::OChannel4 => timer.ccmr2.uncheck(flags::ccmr2::OC4PE),
        _ => { }
	};
}

/*---------------------------------------------------------------------------*/
/** @brief Timer Set the Output Polarity High

The polarity of the channel output is set active high.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] oc_id enum ::OutputCompareChannel OC channel designators
		TIM_OCx where x=1..4, TIM_OCxN where x=1..3  (only for advanced
		timers 1 and 8)
*/

pub fn set_oc_polarity_high(timer: Timer, oc_id: OutputCompareChannel) {
    match oc_id {
        OutputCompareChannel::OChannel1 => timer.ccer.uncheck(flags::ccer::CC1P),
        OutputCompareChannel::OChannel2 => timer.ccer.uncheck(flags::ccer::CC2P),
        OutputCompareChannel::OChannel3 => timer.ccer.uncheck(flags::ccer::CC3P),
        OutputCompareChannel::OChannel4 => timer.ccer.uncheck(flags::ccer::CC4P),
        _ => { }
	};

    if !timer.is_advanced() {
        return;
    }

    match oc_id {
        OutputCompareChannel::OChannel1 => timer.ccer.uncheck(flags::ccer::CC1NP),
        OutputCompareChannel::OChannel2 => timer.ccer.uncheck(flags::ccer::CC2NP),
        OutputCompareChannel::OChannel3 => timer.ccer.uncheck(flags::ccer::CC3NP),
        OutputCompareChannel::OChannel4 => timer.ccer.uncheck(flags::ccer::CC4NP),
        _ => { }
	};
}

/*---------------------------------------------------------------------------*/
/** @brief Timer Set the Output Polarity Low

The polarity of the channel output is set active low.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] oc_id enum ::OutputCompareChannel OC channel designators
		TIM_OCx where x=1..4, TIM_OCxN where x=1..3 (only for advanced
		timers 1 and 8)
*/

pub fn set_oc_polarity_low(timer: Timer, oc_id: OutputCompareChannel) {
    match oc_id {
        OutputCompareChannel::OChannel1 => timer.ccer.check(flags::ccer::CC1P),
        OutputCompareChannel::OChannel2 => timer.ccer.check(flags::ccer::CC2P),
        OutputCompareChannel::OChannel3 => timer.ccer.check(flags::ccer::CC3P),
        OutputCompareChannel::OChannel4 => timer.ccer.check(flags::ccer::CC4P),
        _ => { }
	};

    if !timer.is_advanced() {
        return;
    }

    match oc_id {
        OutputCompareChannel::OChannel1 => timer.ccer.check(flags::ccer::CC1NP),
        OutputCompareChannel::OChannel2 => timer.ccer.check(flags::ccer::CC2NP),
        OutputCompareChannel::OChannel3 => timer.ccer.check(flags::ccer::CC3NP),
        OutputCompareChannel::OChannel4 => timer.ccer.check(flags::ccer::CC4NP),
        _ => { }
	};
}

/*---------------------------------------------------------------------------*/
/** @brief Timer Enable the Output Compare

The channel output compare functionality is enabled.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] oc_id enum ::OutputCompareChannel OC channel designators
		TIM_OCx where x=1..4, TIM_OCxN where x=1..3 (only for advanced
		timers 1 and 8)
*/

pub fn enable_oc_output(timer: Timer, oc_id: OutputCompareChannel) {
    match oc_id {
        OutputCompareChannel::OChannel1 => timer.ccer.check(flags::ccer::CC1E),
        OutputCompareChannel::OChannel2 => timer.ccer.check(flags::ccer::CC2E),
        OutputCompareChannel::OChannel3 => timer.ccer.check(flags::ccer::CC3E),
        OutputCompareChannel::OChannel4 => timer.ccer.check(flags::ccer::CC4E),
        _ => { }
	};

    if !timer.is_advanced() {
        return;
    }

    match oc_id {
        OutputCompareChannel::OChannel1 => timer.ccer.check(flags::ccer::CC1NE),
        OutputCompareChannel::OChannel2 => timer.ccer.check(flags::ccer::CC2NE),
        OutputCompareChannel::OChannel3 => timer.ccer.check(flags::ccer::CC3NE),
        OutputCompareChannel::OChannel4 => timer.ccer.check(flags::ccer::CC4NE),
        _ => { }
	};
}

/*---------------------------------------------------------------------------*/
/** @brief Timer Disable the Output Compare

The channel output compare functionality is disabled.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] oc_id enum ::OutputCompareChannel OC channel designators
		TIM_OCx where x=1..4, TIM_OCxN where x=1..3 (only for advanced
		timers 1 and 8)
*/

pub fn disable_oc_output(timer: Timer, oc_id: OutputCompareChannel) {
    match oc_id {
        OutputCompareChannel::OChannel1 => timer.ccer.uncheck(flags::ccer::CC1E),
        OutputCompareChannel::OChannel2 => timer.ccer.uncheck(flags::ccer::CC2E),
        OutputCompareChannel::OChannel3 => timer.ccer.uncheck(flags::ccer::CC3E),
        OutputCompareChannel::OChannel4 => timer.ccer.uncheck(flags::ccer::CC4E),
        _ => { }
	};

    if !timer.is_advanced() {
        return;
    }

    match oc_id {
        OutputCompareChannel::OChannel1 => timer.ccer.uncheck(flags::ccer::CC1NE),
        OutputCompareChannel::OChannel2 => timer.ccer.uncheck(flags::ccer::CC2NE),
        OutputCompareChannel::OChannel3 => timer.ccer.uncheck(flags::ccer::CC3NE),
        OutputCompareChannel::OChannel4 => timer.ccer.uncheck(flags::ccer::CC4NE),
        _ => { }
	};
}

/*---------------------------------------------------------------------------*/
/** @brief Timer set Output Compare Idle State High

@sa Similar function suitable for multiple OC idle state settings
@ref timer_set_output_idle_state

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] oc_id enum ::OutputCompareChannel OC channel designators
		TIM_OCx where x=1..4, TIM_OCxN where x=1..3 (only for advanced
		timers 1 and 8)
*/

pub fn set_oc_idle_state_set(timer: Timer, oc_id: OutputCompareChannel) {
    if timer.is_advanced() {
        match oc_id {
            OutputCompareChannel::OChannel1  => timer.cr2.check(flags::cr2::OIS1),
            OutputCompareChannel::OChannel1N => timer.cr2.check(flags::cr2::OIS1N),
            OutputCompareChannel::OChannel2  => timer.cr2.check(flags::cr2::OIS2),
            OutputCompareChannel::OChannel2N => timer.cr2.check(flags::cr2::OIS2N),
            OutputCompareChannel::OChannel3  => timer.cr2.check(flags::cr2::OIS3),
            OutputCompareChannel::OChannel3N => timer.cr2.check(flags::cr2::OIS3N),
            OutputCompareChannel::OChannel4  => timer.cr2.check(flags::cr2::OIS4),
        }
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Timer Set Output Compare Idle State Low

@sa Similar function suitable for multiple OC idle state settings
@ref timer_reset_output_idle_state

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base @ref
tim_reg_base
@param[in] oc_id enum ::OutputCompareChannel OC channel designators
		TIM_OCx where x=1..4, TIM_OCxN where x=1..3 (only for advanced
		timers 1 and 8)
*/

pub fn set_oc_idle_state_unset(timer: Timer, oc_id: OutputCompareChannel) {
    if timer.is_advanced() {
        match oc_id {
            OutputCompareChannel::OChannel1  => timer.cr2.uncheck(flags::cr2::OIS1),
            OutputCompareChannel::OChannel1N => timer.cr2.uncheck(flags::cr2::OIS1N),
            OutputCompareChannel::OChannel2  => timer.cr2.uncheck(flags::cr2::OIS2),
            OutputCompareChannel::OChannel2N => timer.cr2.uncheck(flags::cr2::OIS2N),
            OutputCompareChannel::OChannel3  => timer.cr2.uncheck(flags::cr2::OIS3),
            OutputCompareChannel::OChannel3N => timer.cr2.uncheck(flags::cr2::OIS3N),
            OutputCompareChannel::OChannel4  => timer.cr2.uncheck(flags::cr2::OIS4),
        }
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Timer Set Output Compare Value

This is a convenience function to set the OC preload register value for loading
to the compare register.

@param[in] Timer struct. Timer register address base @ref
		tim_reg_base (TIM9 .. TIM14 not yet supported here).
@param[in] oc_id enum ::OutputCompareChannel OC channel designators
		TIM_OCx where x=1..4, TIM_OCxN where x=1..3 (no action taken)
@param[in] value Unsigned int32. Compare value.
*/

pub fn set_oc_value(timer: Timer, oc_id: OutputCompareChannel, value: u32) {
    match oc_id {
        OutputCompareChannel::OChannel1 => timer.ccr1.set(value),
        OutputCompareChannel::OChannel2 => timer.ccr2.set(value),
        OutputCompareChannel::OChannel3 => timer.ccr3.set(value),
        OutputCompareChannel::OChannel4 => timer.ccr4.set(value),
        _ => { }
	};
}

/*---------------------------------------------------------------------------*/
/** @brief Enable Output in Break

Enables the output in the Break feature of an advanced timer. This does not
enable the break functionality itself but only sets the Master Output Enable in
the Break and Deadtime Register.

@note This setting is only valid for the advanced timers.

@note It is necessary to call this function to enable the output on an advanced
timer <b>even if break or deadtime features are not being used</b>.

@param[in] Timer struct. Timer register address base TIM1 or
TIM8
*/

pub fn enable_break_main_output(timer: Timer) {
    if timer.is_advanced() {
        timer.bdtr.check(flags::bdtr::MOE);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Disable Output in Break

Disables the output in the Break feature of an advanced timer. This clears
the Master Output Enable in the Break and Deadtime Register.

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base TIM1 or
TIM8
*/

pub fn disable_break_main_output(timer: Timer) {
    if timer.is_advanced() {
        timer.bdtr.uncheck(flags::bdtr::MOE);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Enable Automatic Output in Break

Enables the automatic output feature of the Break function of an advanced
timer so that the output is re-enabled at the next update event following a
break event.

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base TIM1 or
TIM8
*/

pub fn enable_break_automatic_output(timer: Timer) {
    if timer.is_advanced() {
        timer.bdtr.check(flags::bdtr::AOE);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Disable Automatic Output in Break

Disables the automatic output feature of the Break function of an advanced
timer so that the output is re-enabled at the next update event following a
break event.

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base TIM1 or
TIM8
*/

pub fn disable_break_automatic_output(timer: Timer) {
    if timer.is_advanced() {
        timer.bdtr.uncheck(flags::bdtr::AOE);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Activate Break when Input High

Sets the break function to activate when the break input becomes high.

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base TIM1 or
TIM8
*/

pub fn set_break_polarity_high(timer: Timer) {
    if timer.is_advanced() {
        timer.bdtr.check(flags::bdtr::BKP);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Activate Break when Input Low

Sets the break function to activate when the break input becomes low.

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base TIM1 or
TIM8
*/

pub fn set_break_polarity_low(timer: Timer) {
    if timer.is_advanced() {
        timer.bdtr.uncheck(flags::bdtr::BKP);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Enable Break

Enables the break function of an advanced timer.

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base TIM1 or
TIM8
*/

pub fn enable_break(timer: Timer) {
    if timer.is_advanced() {
        timer.bdtr.check(flags::bdtr::BKE);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Disable Break

Disables the break function of an advanced timer.

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base TIM1 or
TIM8
*/

pub fn disable_break(timer: Timer) {
    if timer.is_advanced() {
        timer.bdtr.uncheck(flags::bdtr::BKE);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Enable Off-State in Run Mode

Enables the off-state in run mode for the break function of an advanced
timer in which the complementary outputs have been configured. It has no effect
if no complementary output is present. When the capture-compare output is
disabled while the complementary output is enabled, the output is set to its
inactive level as defined by the output polarity.

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base TIM1 or
TIM8
*/

pub fn set_enabled_off_state_in_run_mode(timer: Timer) {
    if timer.is_advanced() {
        timer.bdtr.check(flags::bdtr::OSSR);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Disable Off-State in Run Mode

Disables the off-state in run mode for the break function of an advanced
timer in which the complementary outputs have been configured. It has no effect
if no complementary output is present. When the capture-compare output is
disabled, the output is also disabled.

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base TIM1 or
TIM8
*/

pub fn set_disabled_off_state_in_run_mode(timer: Timer) {
    if timer.is_advanced() {
        timer.bdtr.uncheck(flags::bdtr::OSSR);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Enable Off-State in Idle Mode

Enables the off-state in idle mode for the break function of an advanced
timer. When the master output is disabled the output is set to its
inactive level as defined by the output polarity.

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base TIM1 or
TIM8
*/

pub fn set_enabled_off_state_in_idle_mode(timer: Timer) {
    if timer.is_advanced() {
        timer.bdtr.check(flags::bdtr::OSSI);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Disable Off-State in Idle Mode

Disables the off-state in idle mode for the break function of an advanced
timer. When the master output is disabled the output is also disabled.

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base TIM1 or
TIM8
*/

pub fn set_disabled_off_state_in_idle_mode(timer: Timer) {
    if timer.is_advanced() {
        timer.bdtr.uncheck(flags::bdtr::OSSI);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Set Lock Bits

Set the lock bits for an advanced timer. Three levels of lock providing
protection against software errors. Once written they cannot be changed until a
timer reset has occurred.

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base TIM1 or
TIM8
@param[in] lock Unsigned int32. Lock specification @ref tim_lock
*/

pub fn set_break_lock(timer: Timer, lock: u32) {
    if timer.is_advanced() {
        timer.bdtr.check(lock);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Set Deadtime

The deadtime and sampling clock (DTSC) is set in the clock division ratio part
of the timer mode settings. The deadtime count is an 8 bit value defined in
terms of the number of DTSC cycles:

@li Bit 7 = 0, deadtime = bits(6:0)
@li Bits 7:6 = 10, deadtime = 2x(64+bits(5:0))
@li Bits 7:5 = 110, deadtime = 8x(32+bits(5:0))
@li Bits 7:5 = 111, deadtime = 16x(32+bits(5:0))

@note This setting is only valid for the advanced timers.

@param[in] Timer struct. Timer register address base TIM1 or
TIM8
@param[in] deadtime Unsigned int32. Deadtime count specification as defined
above.
*/

pub fn set_deadtime(timer: Timer, deadtime: u32) {
    if timer.is_advanced() {
        timer.bdtr.check(deadtime);
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Force generate a timer event.

The event specification consists of 8 possible events that can be forced on the
timer. The forced events are automatically cleared by hardware. The UG event is
useful to cause shadow registers to be preloaded before the timer is started to
avoid uncertainties in the first cycle in case an update event may never be
generated.

@param[in] Timer struct. Timer register address base
@param[in] event Unsigned int32. Event specification @ref tim_event_gen
*/

pub fn generate_event(timer: Timer, event: u32) {
    timer.egr.check(event);
}

/*---------------------------------------------------------------------------*/
/** @brief Read Counter

Read back the value of a timer's counter register contents

@param[in] Timer struct. Timer register address base
@returns Unsigned int32. Counter value.
*/

pub fn get_counter(timer: Timer) -> u32 {
    timer.cnt.get()
}

/*---------------------------------------------------------------------------*/
/** @brief Set Counter

Set the value of a timer's counter register contents.

@param[in] Timer struct. Timer register address base
@param[in] count Unsigned int32. Counter value.
*/

pub fn set_counter(timer: Timer, count: u32) {
    timer.cnt.set(count);
}

/*---------------------------------------------------------------------------*/
/** @brief Set Input Capture Filter Parameters

Set the input filter parameters for an input channel, specifying:
@li the frequency of sampling from the Deadtime and Sampling clock
(@see @ref timer_set_clock_division)
@li the number of events that must occur before a transition is considered
valid.

@param[in] Timer struct. Timer register address base
@param[in] ic ::InputCaptureChannel. Input Capture channel designator.
@param[in] flt ::InputCaptureFilter. Input Capture Filter identifier.
*/

pub fn ic_set_filter(timer: Timer, ic: InputCaptureChannel, flt: InputCaptureFilter) {
    match ic {
        InputCaptureChannel::Channel1 => timer.ccmr1.mask_set(flags::ccmr1::IC1F_MASK, 4,  flt as u32),
        InputCaptureChannel::Channel2 => timer.ccmr1.mask_set(flags::ccmr1::IC2F_MASK, 12, flt as u32),
        InputCaptureChannel::Channel3 => timer.ccmr2.mask_set(flags::ccmr2::IC3F_MASK, 4,  flt as u32),
        InputCaptureChannel::Channel4 => timer.ccmr2.mask_set(flags::ccmr2::IC4F_MASK, 12, flt as u32),
    }
}

/*---------------------------------------------------------------------------*/
/** @brief Set Input Capture Prescaler

Set the number of events between each capture.

@param[in] Timer struct. Timer register address base
@param[in] ic ::InputCaptureChannel. Input Capture channel designator.
@param[in] psc ::InputCapturePrescaler. Input Capture sample clock prescaler.
*/

pub fn ic_set_prescaler(timer: Timer, ic: InputCaptureChannel, psc: InputCapturePrescaler) {
    match ic {
        InputCaptureChannel::Channel1 => timer.ccmr1.mask_set(flags::ccmr1::IC1PSC_MASK, 2,  psc as u32),
        InputCaptureChannel::Channel2 => timer.ccmr1.mask_set(flags::ccmr1::IC2PSC_MASK, 10, psc as u32),
        InputCaptureChannel::Channel3 => timer.ccmr2.mask_set(flags::ccmr2::IC3PSC_MASK, 2,  psc as u32),
        InputCaptureChannel::Channel4 => timer.ccmr2.mask_set(flags::ccmr2::IC4PSC_MASK, 10, psc as u32),
    };
}

/*---------------------------------------------------------------------------*/
/** @brief Set Capture/Compare Channel Direction/Input

The Capture/Compare channel is defined as output (compare) or input with the
input mapping specified:

@li channel is configured as output
@li channel is configured as input and mapped on corresponding input
@li channel is configured as input and mapped on alternate input
(TI2 for channel 1, TI1 for channel 2, TI4 for channel 3, TI3 for channel 4)
@li channel is configured as input and is mapped on TRC (requires an
internal trigger input selected through TS bit

@note not all combinations of the input and channel are valid, see datasheets.
@note these parameters are writable only when the channel is off.

@param[in] Timer struct. Timer register address base
@param[in] ic ::InputCaptureChannel. Input Capture channel designator.
@param[in] in ::InputCaptureSource. Input Capture channel direction and source input.
*/

pub fn ic_set_input(timer: Timer, ic: InputCaptureChannel, ics: InputCaptureSource) {
	let mut input = (ics as u32) & 3;

	if ((ic == InputCaptureChannel::Channel2) || (ic == InputCaptureChannel::Channel4)) &&
	   ((ics == InputCaptureSource::InTI1) || (ics == InputCaptureSource::InTI2)) {
		/* Input select bits are flipped for these combinations */
		input ^= 3;
	}

    match ic {
        InputCaptureChannel::Channel1 => timer.ccmr1.mask_set(flags::ccmr1::CC1S_MASK, 0, input),
        InputCaptureChannel::Channel2 => timer.ccmr1.mask_set(flags::ccmr1::CC2S_MASK, 8, input),
        InputCaptureChannel::Channel3 => timer.ccmr2.mask_set(flags::ccmr2::CC3S_MASK, 0, input),
        InputCaptureChannel::Channel4 => timer.ccmr2.mask_set(flags::ccmr2::CC4S_MASK, 8, input),
    };
}

/*---------------------------------------------------------------------------*/
/** @brief Enable Timer Input Capture

@param[in] Timer struct. Timer register address base
@param[in] ic ::InputCaptureChannel. Input Capture channel designator.
*/

pub fn ic_enable(timer: Timer, ic: InputCaptureChannel) {
	timer.ccer.check(0x1 << ((ic as u32) * 4));
}

/*---------------------------------------------------------------------------*/
/** @brief Disable Timer Input Capture

@param[in] Timer struct. Timer register address base
@param[in] ic ::InputCaptureChannel. Input Capture channel designator.
*/

pub fn ic_disable(timer: Timer, ic: InputCaptureChannel) {
    timer.ccer.uncheck(0x1 << ((ic as u32) * 4));
}

/*---------------------------------------------------------------------------*/
/** @brief Set External Trigger Filter Parameters for Slave

Set the input filter parameters for the external trigger, specifying:
@li the frequency of sampling from the Deadtime and Sampling clock
(@see @ref timer_set_clock_division)
@li the number of events that must occur before a transition is considered
valid.

@param[in] Timer struct. Timer register address base
@param[in] flt ::InputCaptureFilter. Input Capture Filter identifier.
*/

pub fn slave_set_filter(timer: Timer, flt: InputCaptureFilter) {
    timer.smcr.uncheck(flags::smcr::ETF_MASK);
    timer.smcr.check((flt as u32) << 8);
}

/*---------------------------------------------------------------------------*/
/** @brief Set External Trigger Prescaler for Slave

Set the external trigger frequency division ratio.

@param[in] Timer struct. Timer register address base
@param[in] psc ::InputCapturePrescaler. Input Capture sample clock prescaler.
*/

pub fn slave_set_prescaler(timer: Timer, psc: InputCapturePrescaler) {
    timer.smcr.uncheck(flags::smcr::ETPS_MASK);
    timer.smcr.check((psc as u32) << 12);
}

/*---------------------------------------------------------------------------*/
/** @brief Set External Trigger Polarity for Slave

@param[in] Timer struct. Timer register address base
@param[in] pol ::ExternalTriggerPolarity. Slave External Trigger polarity.
*/

pub fn slave_set_polarity(timer: Timer, pol: ExternalTriggerPolarity) {
    match pol {
        ExternalTriggerPolarity::Rising => timer.smcr.uncheck(flags::smcr::ETP),
        ExternalTriggerPolarity::Falling => timer.smcr.check(flags::smcr::ETP),
    };
}

/*---------------------------------------------------------------------------*/
/** @brief Set Slave Mode

@param[in] Timer struct. Timer register address base
@param[in] mode Unsigned int8. Slave mode @ref tim_sms
*/

pub fn slave_set_mode(timer: Timer, mode: u8) {
    timer.smcr.uncheck(flags::smcr::SMS_MASK);
    timer.smcr.check(mode as u32);
}

/*---------------------------------------------------------------------------*/
/** @brief Set Slave Trigger Source

@param[in] Timer struct. Timer register address base
@param[in] trigger Unsigned int8. Slave trigger source @ref tim_ts
*/

pub fn slave_set_trigger(timer: Timer, trigger: u8) {
    timer.smcr.uncheck(flags::smcr::TS_MASK);
    timer.smcr.check(trigger as u32);
}

/* TODO Timer DMA burst */

/**@}*/

pub fn set_option(timer: Timer, option: u32) {
    if timer == TIM2 {
        timer.or.uncheck(flags::or::ITR1_RMP_MASK);
        timer.or.check(option);
    } else if timer == TIM5 {
        timer.or.uncheck(flags::or::ITR4_RMP_MASK);
        timer.or.check(option);
    }
}

pub fn ic_set_polarity(timer: Timer, ic: InputCaptureChannel, pol: InputCapturePolarity) {
	/* Clear CCxP and CCxNP to zero. For both edge trigger both fields are
	 * set. Case 10 is invalid.
	 */

    timer.ccer.uncheck(0xa << ((ic as u32) * 4));

	match pol {
	    /* 01 */ InputCapturePolarity::Falling => timer.ccer.check(0x2 << ((ic as u32) * 4)),
	    /* 11 */ InputCapturePolarity::Both => timer.ccer.check(0xa << ((ic as u32) * 4)),
        _ => {}
	}
}