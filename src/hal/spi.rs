use core::ops::Deref;

use rcc;
use common::asm;
use common::VolatileCell;

pub const SPI1: Spi = Spi { addr: 0x40013000 };
pub const SPI2: Spi = Spi { addr: 0x40003800 };
pub const SPI3: Spi = Spi { addr: 0x40003c00 };
pub const SPI4: Spi = Spi { addr: 0x40013400 };
pub const SPI5: Spi = Spi { addr: 0x40015000 };
pub const SPI6: Spi = Spi { addr: 0x40015400 };

mod flags {
    pub mod cr1 {
        /* BIDIMODE: Bidirectional data mode enable */
        pub const BIDIMODE_2LINE_UNIDIR: u32 = 0 << 15;
        pub const BIDIMODE_1LINE_BIDIR: u32 = 1 << 15;
        pub const BIDIMODE: u32 = 1 << 15;

        /* BIDIOE: Output enable in bidirectional mode */
        pub const BIDIOE: u32 = 1 << 14;

        /* CRCEN: Hardware CRC calculation enable */
        pub const CRCEN: u32 = 1 << 13;

        /* CRCNEXT: Transmit CRC next */
        pub const CRCNEXT: u32 = 1 << 12;

        pub const CRCL_8BIT: u32 = 0 << 11;
        pub const CRCL_16BIT: u32 = 1 << 11;
        /**@}*/
        pub const CRCL: u32 = 1 << 11;

        pub const DFF_8BIT: u32 = 0 << 11;
        pub const DFF_16BIT: u32 = 1 << 11;
        /**@}*/
        pub const DFF: u32 = 1 << 11;

        /* RXONLY: Receive only */
        pub const RXONLY: u32 = 1 << 10;

        /* SSM: Software slave management */
        pub const SSM: u32 = 1 << 9;

        /* SSI: Internal slave select */
        pub const SSI: u32 = 1 << 8;

        /* LSBFIRST: Frame format */
        /****************************************************************************/
        /** @defgroup spi_lsbfirst SPI lsb/msb first
        @ingroup spi_defines

        @{*/
        pub const MSBFIRST: u32 = 0 << 7;
        pub const LSBFIRST: u32 = 1 << 7;
        /**@}*/

        /* SPE: SPI enable */
        pub const SPE: u32 = 1 << 6;

        /* BR[2:0]: Baud rate control */
        /****************************************************************************/
        /** @defgroup spi_baudrate SPI peripheral baud rates
        @ingroup spi_defines

        @{*/
        pub const BAUDRATE_FPCLK_DIV_2: u32 = 0x00 << 3;
        pub const BAUDRATE_FPCLK_DIV_4: u32 = 0x01 << 3;
        pub const BAUDRATE_FPCLK_DIV_8: u32 = 0x02 << 3;
        pub const BAUDRATE_FPCLK_DIV_16: u32 = 0x03 << 3;
        pub const BAUDRATE_FPCLK_DIV_32: u32 = 0x04 << 3;
        pub const BAUDRATE_FPCLK_DIV_64: u32 = 0x05 << 3;
        pub const BAUDRATE_FPCLK_DIV_128: u32 = 0x06 << 3;
        pub const BAUDRATE_FPCLK_DIV_256: u32 = 0x07 << 3;
        /**@}*/
        /****************************************************************************/
        /** @defgroup spi_br_pre SPI peripheral baud rate prescale values
        @ingroup spi_defines

        @{*/
        pub const BR_FPCLK_DIV_2: u32 = 0x0;
        pub const BR_FPCLK_DIV_4: u32 = 0x1;
        pub const BR_FPCLK_DIV_8: u32 = 0x2;
        pub const BR_FPCLK_DIV_16: u32 = 0x3;
        pub const BR_FPCLK_DIV_32: u32 = 0x4;
        pub const BR_FPCLK_DIV_64: u32 = 0x5;
        pub const BR_FPCLK_DIV_128: u32 = 0x6;
        pub const BR_FPCLK_DIV_256: u32 = 0x7;
        /**@}*/

        /* MSTR: Master selection */
        pub const MSTR: u32 = 1 << 2;

        /* CPOL: Clock polarity */
        /****************************************************************************/
        /** @defgroup spi_cpol SPI clock polarity
        @ingroup spi_defines

        @{*/
        pub const CPOL_CLK_TO_0_WHEN_IDLE: u32 = 0 << 1;
        pub const CPOL_CLK_TO_1_WHEN_IDLE: u32 = 1 << 1;
        /**@}*/
        pub const CPOL: u32 = 1 << 1;

        /* CPHA: Clock phase */
        /****************************************************************************/
        /** @defgroup spi_cpha SPI clock phase
        @ingroup spi_defines

        @{*/
        pub const CPHA_CLK_TRANSITION_1: u32 = 0 << 0;
        pub const CPHA_CLK_TRANSITION_2: u32 = 1 << 0;
        /**@}*/
        pub const CPHA: u32 = 1 << 0;
    }

    pub mod cr2 {
        /* LDMA_TX: Last DMA transfer for transmission */
        pub const LDMA_TX: u32 = 1 << 14;

        /* LDMA_RX: Last DMA transfer for reception */
        pub const LDMA_RX: u32 = 1 << 13;

        /* FRXTH: FIFO reception threshold */
        pub const FRXTH: u32 = 1 << 12;

        /* DS [3:0]: Data size */
        /* 0x0 - 0x2 NOT USED */
        pub const DS_4BIT: u32 = 0x3 << 8;
        pub const DS_5BIT: u32 = 0x4 << 8;
        pub const DS_6BIT: u32 = 0x5 << 8;
        pub const DS_7BIT: u32 = 0x6 << 8;
        pub const DS_8BIT: u32 = 0x7 << 8;
        pub const DS_9BIT: u32 = 0x8 << 8;
        pub const DS_10BIT: u32 = 0x9 << 8;
        pub const DS_11BIT: u32 = 0xA << 8;
        pub const DS_12BIT: u32 = 0xB << 8;
        pub const DS_13BIT: u32 = 0xC << 8;
        pub const DS_14BIT: u32 = 0xD << 8;
        pub const DS_15BIT: u32 = 0xE << 8;
        pub const DS_16BIT: u32 = 0xF << 8;
        pub const DS_MASK: u32 = 0xF << 8;

        /* Bits [15:8]: Reserved. Forced to 0 by hardware. Used on F3. */

        /* TXEIE: Tx buffer empty interrupt enable */
        pub const TXEIE: u32 = 1 << 7;

        /* RXNEIE: Rx buffer not empty interrupt enable */
        pub const RXNEIE: u32 = 1 << 6;

        /* ERRIE: Error interrupt enable */
        pub const ERRIE: u32 = 1 << 5;

        /* FRF: Frame format */
        /* Note: Not used in I2S mode. */
        pub const FRF: u32 = 1 << 4;
        pub const FRF_MOTOROLA_MODE: u32 = 0 << 4;
        pub const FRF_TI_MODE: u32 = 1 << 4;

        /* NSSP: NSS pulse management */
        pub const NSSP: u32 = 1 << 3;

        /* SSOE: SS output enable */
        /* Note: Not used in I2S mode. */
        pub const SSOE: u32 = 1 << 2;

        /* TXDMAEN: Tx buffer DMA enable */
        pub const TXDMAEN: u32 = 1 << 1;

        /* RXDMAEN: Rx buffer DMA enable */
        pub const RXDMAEN: u32 = 1 << 0;
    }

    pub mod sr {

        /* FTLVL[1:0]: FIFO Transmission Level */
        pub const FTLVL_FIFO_EMPTY: u32 = 0x0 << 11;
        pub const FTLVL_QUARTER_FIFO: u32 = 0x1 << 11;
        pub const FTLVL_HALF_FIFO: u32 = 0x2 << 11;
        pub const FTLVL_FIFO_FULL: u32 = 0x3 << 11;

        /* FRLVL[1:0]: FIFO Reception Level */
        pub const FRLVL_FIFO_EMPTY: u32 = 0x0 << 9;
        pub const FRLVL_QUARTER_FIFO: u32 = 0x1 << 9;
        pub const FRLVL_HALF_FIFO: u32 = 0x2 << 9;
        pub const FRLVL_FIFO_FULL: u32 = 0x3 << 9;

        /* TIFRFE: TI frame format error */
        pub const TIFRFE: u32 = 1 << 8;

        /* BSY: Busy flag */
        pub const BSY: u32 = 1 << 7;

        /* OVR: Overrun flag */
        pub const OVR: u32 = 1 << 6;

        /* MODF: Mode fault */
        /* Note: Not used in I2S mode. */
        pub const MODF: u32 = 1 << 5;

        /* CRCERR: CRC error flag */
        /* Note: Not used in I2S mode. */
        pub const CRCERR: u32 = 1 << 4;

        /* UDR: Underrun flag */
        /* Note: Not used in SPI mode. */
        pub const UDR: u32 = 1 << 3;

        /* CHSIDE: Channel side */
        /* Note: Not used in SPI mode. No meaning in PCM mode. */
        pub const CHSIDE: u32 = 1 << 2;

        /* TXE: Transmit buffer empty */
        pub const TXE: u32 = 1 << 1;

        /* RXNE: Receive buffer not empty */
        pub const RXNE: u32 = 1 << 0;
    }

    pub mod i2scfgr {
        /* I2SMOD: I2S mode selection */
        pub const I2SMOD: u32 = 1 << 11;

        /* I2SE: I2S enable */
        pub const I2SE: u32 = 1 << 10;

        /* I2SCFG[9:8]: I2S configuration mode */
        pub const I2SCFG_LSB: u32 = 8;
        pub const I2SCFG_SLAVE_TRANSMIT: u32 = 0x0;
        pub const I2SCFG_SLAVE_RECEIVE: u32 = 0x1;
        pub const I2SCFG_MASTER_TRANSMIT: u32 = 0x2;
        pub const I2SCFG_MASTER_RECEIVE: u32 = 0x3;

        /* PCMSYNC: PCM frame synchronization */
        pub const PCMSYNC: u32 = 1 << 7;

        /* Bit 6: Reserved. Forced to 0 by hardware. */

        /* I2SSTD[5:4]: I2S standard selection */
        pub const I2SSTD_LSB: u32 = 4;
        pub const I2SSTD_I2S_PHILIPS: u32 = 0x0;
        pub const I2SSTD_MSB_JUSTIFIED: u32 = 0x1;
        pub const I2SSTD_LSB_JUSTIFIED: u32 = 0x2;
        pub const I2SSTD_PCM: u32 = 0x3;

        /* CKPOL: Steady state clock polarity */
        pub const CKPOL: u32 = 1 << 3;

        /* DATLEN[2:1]: Data length to be transferred */
        pub const DATLEN_LSB: u32 = 1;
        pub const DATLEN_16BIT: u32 = 0x0;
        pub const DATLEN_24BIT: u32 = 0x1;
        pub const DATLEN_32BIT: u32 = 0x2;

        /* CHLEN: Channel length */
        pub const CHLEN: u32 = 1 << 0;
    }
    pub mod i2spr {
        /* Note: None of these bits are used in SPI mode. */

        /* Bits [15:10]: Reserved. Forced to 0 by hardware. */

        /* MCKOE: Master clock output enable */
        pub const MCKOE: u32 = 1 << 9;

        /* ODD: Odd factor for the prescaler */
        pub const ODD: u32 = 1 << 8;
    }
}

#[repr(C)]
pub struct Registers {
    pub cr1: VolatileCell<u32>,
    pub cr2: VolatileCell<u32>,
    pub sr: VolatileCell<u32>,
    pub dr: VolatileCell<u32>,
    pub crcpr: VolatileCell<u32>,
    pub rxcrcr: VolatileCell<u32>,
    pub txcrcr: VolatileCell<u32>,
    pub i2scfgr: VolatileCell<u32>,
    pub i2spr: VolatileCell<u32>,
}

#[derive(PartialEq, Eq)]
pub struct Spi {
    addr: usize
}

impl Deref for Spi {
    type Target = Registers;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.addr as *const Registers) }
    }
}

impl Spi {

    /** @brief SPI Reset.

    The SPI peripheral and all its associated configuration registers are placed in
    the reset condition. The reset is effected via the RCC peripheral reset system.

    @param[in] spi_peripheral Unsigned int32. SPI peripheral identifier @ref
    spi_reg_base.
    */
    pub fn reset(&self) {	
        match self {
            &SPI1 => rcc::reset_pulse(rcc::Peripheral::SPI1),
            &SPI2 => rcc::reset_pulse(rcc::Peripheral::SPI2),
            &SPI3 => rcc::reset_pulse(rcc::Peripheral::SPI3),
            &SPI4 => rcc::reset_pulse(rcc::Peripheral::SPI4),
            &SPI5 => rcc::reset_pulse(rcc::Peripheral::SPI5),
            &SPI6 => rcc::reset_pulse(rcc::Peripheral::SPI6),
            _ => {}
        }
    }

    /* TODO: Error handling? */
    /*---------------------------------------------------------------------------*/
    /** @brief SPI Enable.

    The SPI peripheral is enabled.

    @todo Error handling?

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn enable(&self) {
        self.cr1.check(flags::cr1::SPE); /* Enable SPI. */
    }

    /* TODO: Error handling? */
    /*---------------------------------------------------------------------------*/
    /** @brief SPI Disable.

    The SPI peripheral is disabled.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn disable(&self) {
        self.cr1.uncheck(flags::cr1::SPE); /* Disable SPI. */
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Clean Disable.

    Disable the SPI peripheral according to the procedure in section 23.3.8 of the
    reference manual.  This prevents corruption of any ongoing transfers and
    prevents the BSY flag from becoming unreliable.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    @returns data Unsigned int16. 8 or 16 bit data from final read.
    */

    pub fn clean_disable(&self) -> u16 {
        /* Wait to receive last data */
        while !self.sr.test(flags::sr::RXNE) { asm::nop(); }

        let data = self.dr.get();

        /* Wait to transmit last data */
        while !self.sr.test(flags::sr::TXE) { asm::nop(); };

        /* Wait until not busy */
        while self.sr.test(flags::sr::BSY) { asm::nop(); };

        self.cr1.uncheck(flags::cr1::SPE);

        return data as u16;
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Data Write.

    Data is written to the SPI interface.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    @param[in] data Unsigned int16. 8 or 16 bit data to be written.
    */

    pub fn write(&self, data: u16) {
        /* Write data (8 or 16 bits, depending on DFF) into DR. */
        self.dr.set(data as u32);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Data Write with Blocking.

    Data is written to the SPI interface after the previous write transfer has
    finished.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    @param[in] data Unsigned int16. 8 or 16 bit data to be written.
    */

    pub fn send(&self, data: u16) {
        /* Wait for transfer finished. */
        while !self.sr.test(flags::sr::TXE) { asm::nop(); };

        /* Write data (8 or 16 bits, depending on DFF) into DR. */
        self.dr.set(data as u32);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Data Read.

    Data is read from the SPI interface after the incoming transfer has finished.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    @returns data Unsigned int16. 8 or 16 bit data.
    */

    pub fn read(&self) -> u16 {
        /* Wait for transfer finished. */
        while !self.sr.test(flags::sr::RXNE) { asm::nop(); }

        /* Read the data (8 or 16 bits, depending on DFF bit) from DR. */
        return self.dr.get() as u16;
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Data Write and Read Exchange.

    Data is written to the SPI interface, then a read is done after the incoming
    transfer has finished.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    @param[in] data Unsigned int16. 8 or 16 bit data to be written.
    @returns data Unsigned int16. 8 or 16 bit data.
    */

    pub fn xfer(&self, data: u16) -> u16 {
        self.write(data);

        /* Wait for transfer finished. */
        while !self.sr.test(flags::sr::RXNE) { asm::nop(); }

        /* Read the data (8 or 16 bits, depending on DFF bit) from DR. */
        return self.dr.get() as u16;
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set Bidirectional Simplex Mode.

    The SPI peripheral is set for bidirectional transfers in two-wire simplex mode
    (using a clock wire and a bidirectional data wire).

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_bidirectional_mode(&self) {
        self.cr1.check(flags::cr1::BIDIMODE);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set Unidirectional Mode.

    The SPI peripheral is set for unidirectional transfers. This is used in full
    duplex mode or when the SPI is placed in two-wire simplex mode that uses a
    clock wire and a unidirectional data wire.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_unidirectional_mode(&self) {
        self.cr1.uncheck(flags::cr1::BIDIMODE);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set Bidirectional Simplex Receive Only Mode.

    The SPI peripheral is set for bidirectional transfers in two-wire simplex mode
    (using a clock wire and a bidirectional data wire), and is placed in a receive
    state.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_bidirectional_receive_only_mode(&self) {
        self.cr1.check(flags::cr1::BIDIMODE);
        self.cr1.uncheck(flags::cr1::BIDIOE);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set Bidirectional Simplex Receive Only Mode.

    The SPI peripheral is set for bidirectional transfers in two-wire simplex mode
    (using a clock wire and a bidirectional data wire), and is placed in a transmit
    state.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_bidirectional_transmit_only_mode(&self) {
        self.cr1.check(flags::cr1::BIDIMODE);
        self.cr1.check(flags::cr1::BIDIOE);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Enable the CRC.

    The SPI peripheral is set to use a CRC field for transmit and receive.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn enable_crc(&self) {
        self.cr1.check(flags::cr1::CRCEN);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Disable the CRC.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn disable_crc(&self) {
        self.cr1.uncheck(flags::cr1::CRCEN);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Next Transmit is a Data Word

    The next transmission to take place is a data word from the transmit buffer.
    This must be called before transmission to distinguish between sending
    of a data or CRC word.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_next_tx_from_buffer(&self) {
        self.cr1.uncheck(flags::cr1::CRCNEXT);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Next Transmit is a CRC Word

    The next transmission to take place is a crc word from the hardware crc unit.
    This must be called before transmission to distinguish between sending
    of a data or CRC word.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_next_tx_from_crc(&self) {
        self.cr1.check(flags::cr1::CRCNEXT);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set Full Duplex (3-wire) Mode

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_full_duplex_mode(&self) {
        self.cr1.uncheck(flags::cr1::RXONLY);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set Receive Only Mode for Simplex (2-wire) Unidirectional
    Transfers

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_receive_only_mode(&self) {
        self.cr1.check(flags::cr1::RXONLY);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Disable Slave Management by Hardware

    In slave mode the NSS hardware input is used as a select enable for the slave.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn disable_software_slave_management(&self) {
        self.cr1.uncheck(flags::cr1::SSM);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Enable Slave Management by Software

    In slave mode the NSS hardware input is replaced by an internal software
    enable/disable of the slave (@ref spi_set_nss_high).

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn enable_software_slave_management(&self) {
        self.cr1.check(flags::cr1::SSM);
        /* allow slave select to be an input */
        self.cr2.uncheck(flags::cr2::SSOE);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set the Software NSS Signal High

    In slave mode, and only when software slave management is used, this replaces
    the NSS signal with a slave select enable signal.

    @todo these should perhaps be combined with an SSM enable as it is meaningless
    otherwise

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_nss_high(&self) {
        self.cr1.check(flags::cr1::SSI);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set the Software NSS Signal Low

    In slave mode, and only when software slave management is used, this replaces
    the NSS signal with a slave select disable signal.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_nss_low(&self) {
        self.cr1.uncheck(flags::cr1::SSI);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set to Send LSB First

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn send_lsb_first(&self) {
        self.cr1.check(flags::cr1::LSBFIRST);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set to Send MSB First

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn send_msb_first(&self) {
        self.cr1.uncheck(flags::cr1::LSBFIRST);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set the Baudrate Prescaler

    @todo Why is this specification different to the spi_init_master baudrate
    values?

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    @param[in] baudrate Unsigned int8. Baudrate prescale value @ref spi_br_pre.
    */

    pub fn set_baudrate_prescaler(&self, baudrate: u8) {
        if baudrate > 7 {
            return;
        }

        let mut reg32 = self.cr1.get() & 0xffc7; /* Clear bits [5:3]. */
        reg32 |= (baudrate as u32) << 3;
        self.cr1.set(reg32);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set to Master Mode

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_master_mode(&self) {
        self.cr1.check(flags::cr1::MSTR);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set to Slave Mode

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_slave_mode(&self) {
        self.cr1.uncheck(flags::cr1::MSTR);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set the Clock Polarity to High when Idle

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    @sa spi_set_clock_polarity_0
    */

    pub fn set_clock_polarity_1(&self) {
        self.cr1.check(flags::cr1::CPOL);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set the Clock Polarity to Low when Idle

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    @sa spi_set_clock_polarity_1
    */

    pub fn set_clock_polarity_0(&self) {
        self.cr1.uncheck(flags::cr1::CPOL);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set the Clock Phase to Capture on Trailing Edge

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    @sa spi_set_clock_phase_0
    */

    pub fn set_clock_phase_1(&self) {
        self.cr1.check(flags::cr1::CPHA);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set the Clock Phase to Capture on Leading Edge

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    @sa spi_set_clock_phase_1
    */

    pub fn set_clock_phase_0(&self) {
        self.cr1.uncheck(flags::cr1::CPHA);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Enable the Transmit Buffer Empty Interrupt

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn enable_tx_buffer_empty_interrupt(&self) {
        self.cr2.check(flags::cr2::TXEIE);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Disable the Transmit Buffer Empty Interrupt

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn disable_tx_buffer_empty_interrupt(&self) {
        self.cr2.uncheck(flags::cr2::TXEIE);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Enable the Receive Buffer Ready Interrupt

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn enable_rx_buffer_not_empty_interrupt(&self) {
        self.cr2.check(flags::cr2::RXNEIE);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Disable the Receive Buffer Ready Interrupt

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn disable_rx_buffer_not_empty_interrupt(&self) {
        self.cr2.uncheck(flags::cr2::RXNEIE);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Enable the Error Interrupt

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn enable_error_interrupt(&self) {
        self.cr2.check(flags::cr2::ERRIE);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Disable the Error Interrupt

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn disable_error_interrupt(&self) {
        self.cr2.uncheck(flags::cr2::ERRIE);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set the NSS Pin as an Output

    Normally used in master mode to allows the master to place all devices on the
    SPI bus into slave mode. Multimaster mode is not possible.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn enable_ss_output(&self) {
        self.cr2.check(flags::cr2::SSOE);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set the NSS Pin as an Input

    In master mode this allows the master to sense the presence of other masters. If
    NSS is then pulled low the master is placed into slave mode. In slave mode NSS
    becomes a slave enable.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn disable_ss_output(&self) {
        self.cr2.uncheck(flags::cr2::SSOE);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Enable Transmit Transfers via DMA

    This allows transmissions to proceed unattended using DMA to move data to the
    transmit buffer as it becomes available. The DMA channels provided for each
    SPI peripheral are given in the Technical Manual DMA section.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn enable_tx_dma(&self) {
        self.cr2.check(flags::cr2::TXDMAEN);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Disable Transmit Transfers via DMA

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn disable_tx_dma(&self) {
        self.cr2.uncheck(flags::cr2::TXDMAEN);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Enable Receive Transfers via DMA

    This allows received data streams to proceed unattended using DMA to move data
    from the receive buffer as data becomes available. The DMA channels provided
    for each SPI peripheral are given in the Technical Manual DMA section.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn enable_rx_dma(&self) {
        self.cr2.check(flags::cr2::RXDMAEN);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Disable Receive Transfers via DMA

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn disable_rx_dma(&self) {
        self.cr2.uncheck(flags::cr2::RXDMAEN);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Standard Mode selection
    @details Set SPI standard Modes
    Mode | CPOL | CPHA
    ---- | ---- | ----
    0   |  0   |  0
    1   |  0   |  1
    2   |  1   |  0
    3   |  1   |  1
    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    @param[in] mode Unsigned int8. Standard SPI mode (0, 1, 2, 3)
    @sa spi_set_clock_phase_0 spi_set_clock_phase_1
    @sa spi_set_clock_polarity_0 spi_set_clock_polarity_1
    */

    pub fn set_standard_mode(&self, mode: u8) {
        if mode > 3 {
            return;
        }

        let reg32 = self.cr1.get() & !(flags::cr1::CPOL | flags::cr1::CPHA);
        self.cr1.set(reg32 | mode as u32);
    }


    /*---------------------------------------------------------------------------*/
    /** @brief Configure the SPI as Master.

    The SPI peripheral is configured as a master with communication parameters
    baudrate, crc length 8/16 bits, frame format lsb/msb first, clock polarity
    and phase. The SPI enable, CRC enable and CRC next controls are not affected.
    These must be controlled separately.

    @todo NSS pin handling.

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    @param[in] br Unsigned int32. Baudrate @ref spi_baudrate.
    @param[in] cpol Unsigned int32. Clock polarity @ref spi_cpol.
    @param[in] cpha Unsigned int32. Clock Phase @ref spi_cpha.
    @param[in] crcl Unsigned int32. CRC length 8/16 bits @ref spi_crcl.
    @param[in] lsbfirst Unsigned int32. Frame format lsb/msb first @ref
    spi_lsbfirst.
    @returns int. Error code.
    */

    pub fn init_master(&self, br: u32, cpol: u32, cpha: u32, crcl: u32, lsbfirst: u32) {
        let mut reg32 = self.cr1.get();

        /* Reset all bits omitting SPE, CRCEN and CRCNEXT bits. */
        reg32 &= flags::cr1::SPE | flags::cr1::CRCEN | flags::cr1::CRCNEXT;
        reg32 |= flags::cr1::MSTR;	/* Configure SPI as master. */

        reg32 |= br;		/* Set baud rate bits. */
        reg32 |= cpol;		/* Set CPOL value. */
        reg32 |= cpha;		/* Set CPHA value. */
        reg32 |= crcl;		/* Set crc length (8 or 16 bits). */
        reg32 |= lsbfirst;	/* Set frame format (LSB- or MSB-first). */

        /* TODO: NSS pin handling. */

        self.cr1.set(reg32);
    }

    pub fn send8(&self, data: u8) {
        /* Wait for transfer finished. */
        while !self.sr.test(flags::sr::TXE) { asm::nop(); };

        /* Write data (8 or 16 bits, depending on DFF) into DR. */
        self.dr.set(data as u32);
    }

    fn read8(&self) -> u8 {
        /* Wait for transfer finished. */
        while !self.sr.test(flags::sr::RXNE) { asm::nop(); };

        /* Read the data (8 or 16 bits, depending on DFF bit) from DR. */
        self.dr.get() as u8
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set CRC length to 8 bits

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_crcl_8bit(&self) {
        self.cr1.uncheck(flags::cr1::CRCL);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set CRC length to 16 bits

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_crcl_16bit(&self) {
        self.cr1.check(flags::cr1::CRCL);
    }

    pub fn set_data_size(&self, data_size: u32) {
        let mut reg32 = self.cr2.get();

        reg32 &= !flags::cr2::DS_MASK;
        reg32 |= data_size & flags::cr2::DS_MASK;

        self.cr2.set(reg32);
    }

    pub fn fifo_reception_threshold_8bit(&self) {
        self.cr2.check(flags::cr2::FRXTH);
    }

    pub fn fifo_reception_threshold_16bit(&self) {
        self.cr2.uncheck(flags::cr2::FRXTH);
    }

    pub fn i2s_mode_spi_mode(&self) {
        self.i2scfgr.uncheck(flags::i2scfgr::I2SMOD);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set Data Frame Format to 8 bits

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_dff_8bit(&self) {
        self.cr1.uncheck(flags::cr1::DFF);
    }

    /*---------------------------------------------------------------------------*/
    /** @brief SPI Set Data Frame Format to 16 bits

    @param[in] spi Unsigned int32. SPI peripheral identifier @ref spi_reg_base.
    */

    pub fn set_dff_16bit(&self) {
        self.cr1.check(flags::cr1::DFF);
    }
}