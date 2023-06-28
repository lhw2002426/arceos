pub const BLOCK_SIZE: usize = 512;

/*
mmc.h
 */
pub const MMC_GO_IDLE_STATE: u32 = 0;
pub const MMC_SEND_OP_COND: u32 = 1;
pub const MMC_ALL_SEND_CID: u32 = 2;
pub const MMC_SET_RELATIVE_ADDR: u32 = 3;
pub const MMC_SET_DSR: u32 = 4;
pub const MMC_SLEEP_AWAKE: u32 = 5;
pub const MMC_SWITCH: u32 = 6;
pub const MMC_SELECT_CARD: u32 = 7;
pub const MMC_SEND_EXT_CSD: u32 = 8;
pub const MMC_SEND_CSD: u32 = 9;
pub const MMC_SEND_CID: u32 = 10;
pub const MMC_READ_DAT_UNTIL_STOP: u32 = 11;
pub const MMC_STOP_TRANSMISSION: u32 = 12;
pub const MMC_SEND_STATUS: u32 = 13;
pub const MMC_BUS_TEST_R: u32 = 14;
pub const MMC_GO_INACTIVE_STATE: u32 = 15;
pub const MMC_BUS_TEST_W: u32 = 19;
pub const MMC_SPI_READ_OCR: u32 = 58;
pub const MMC_SPI_CRC_ON_OFF: u32 = 59;

  /* class 2 */
pub const MMC_SET_BLOCKLEN         :u32= 16;   /* ac   [31:0] block len   R1  */
pub const MMC_READ_SINGLE_BLOCK    :u32= 17;   /* adtc [31:0] data addr   R1  */
pub const MMC_READ_MULTIPLE_BLOCK  :u32= 18;   /* adtc [31:0] data addr   R1  */
pub const MMC_SEND_TUNING_BLOCK    :u32= 19;   /* adtc                    R1  */
pub const MMC_SEND_TUNING_BLOCK_HS200	:u32= 21;	/* adtc R1  */
  
    /* class 3 */
pub const MMC_WRITE_DAT_UNTIL_STOP :u32= 20;   /* adtc [31:0] data addr   R1  */
  
    /* class 4 */
pub const MMC_SET_BLOCK_COUNT      :u32= 23;   /* adtc [31:0] data addr   R1  */
pub const MMC_WRITE_BLOCK          :u32= 24;   /* adtc [31:0] data addr   R1  */
pub const MMC_WRITE_MULTIPLE_BLOCK :u32= 25;   /* adtc                    R1  */
pub const MMC_PROGRAM_CID          :u32= 26;   /* adtc                    R1  */
pub const MMC_PROGRAM_CSD          :u32= 27;   /* adtc                    R1  */

/*
core.h
 */
pub const MMC_CMD23_ARG_REL_WR	:u32= (1 << 31);
pub const MMC_CMD23_ARG_PACKED	:u32= ((0 << 31) | (1 << 30));
pub const MMC_CMD23_ARG_TAG_REQ	:u32= (1 << 29);

pub const MMC_RSP_PRESENT	:u32= (1 << 0);
pub const MMC_RSP_136	:u32= (1 << 1);	/* 136 bit response */
pub const MMC_RSP_CRC	:u32= (1 << 2);		/* expect valid crc */
pub const MMC_RSP_BUSY	:u32= (1 << 3);		/* card may send busy */
pub const MMC_RSP_OPCODE	:u32= (1 << 4);		/* response contains opcode */

pub const MMC_CMD_MASK	:u32= (3 << 5);		/* non-SPI command type */
pub const MMC_CMD_AC	:u32= (0 << 5);
pub const MMC_CMD_ADTC	:u32= (1 << 5);
pub const MMC_CMD_BC	:u32= (2 << 5);
pub const MMC_CMD_BCR	:u32= (3 << 5);

pub const MMC_RSP_SPI_S1	:u32= (1 << 7);		/* one status byte */
pub const MMC_RSP_SPI_S2	:u32= (1 << 8);		/* second byte */
pub const MMC_RSP_SPI_B4	:u32= (1 << 9);		/* four data bytes */
pub const MMC_RSP_SPI_BUSY :u32= (1 << 10);		/* card may send busy */

pub const MMC_RSP_NONE	:u32= (0);
pub const MMC_RSP_R1	:u32= (MMC_RSP_PRESENT|MMC_RSP_CRC|MMC_RSP_OPCODE);
pub const MMC_RSP_R1B	:u32= (MMC_RSP_PRESENT|MMC_RSP_CRC|MMC_RSP_OPCODE|MMC_RSP_BUSY);
pub const MMC_RSP_R2	:u32= (MMC_RSP_PRESENT|MMC_RSP_136|MMC_RSP_CRC);
pub const MMC_RSP_R3	:u32= (MMC_RSP_PRESENT);
pub const MMC_RSP_R4	:u32= (MMC_RSP_PRESENT);
pub const MMC_RSP_R5	:u32= (MMC_RSP_PRESENT|MMC_RSP_CRC|MMC_RSP_OPCODE);
pub const MMC_RSP_R6	:u32= (MMC_RSP_PRESENT|MMC_RSP_CRC|MMC_RSP_OPCODE);
pub const MMC_RSP_R7	:u32= (MMC_RSP_PRESENT|MMC_RSP_CRC|MMC_RSP_OPCODE);

pub const MMC_DATA_WRITE: u32 = 1 << 8;
pub const MMC_DATA_READ: u32 = 1 << 9;
pub const MMC_DATA_QBR: u32 = 1 << 10;
pub const MMC_DATA_PRIO: u32 = 1 << 11;
pub const MMC_DATA_REL_WR: u32 = 1 << 12;
pub const MMC_DATA_DAT_TAG: u32 = 1 << 13;
pub const MMC_DATA_FORCED_PRG: u32 = 1 << 14;

/*
param.h
 */
pub const HZ :u32 = 1024;
/* 
sdhci.h
*/
pub const SDHCI_DMA_ADDRESS: u32 = 0x00;
pub const SDHCI_ARGUMENT2: u32 = SDHCI_DMA_ADDRESS;
pub const SDHCI_32BIT_BLK_CNT: u32 = SDHCI_DMA_ADDRESS;

pub const SDHCI_BLOCK_SIZE: u32 = 0x04;
pub fn SDHCI_MAKE_BLKSZ(dma: u32, blksz: u32) -> u32 {
    (((dma & 0x7) << 12) | (blksz & 0xFFF))
}

pub const SDHCI_BLOCK_COUNT: u32 = 0x06;

pub const SDHCI_ARGUMENT: u32 = 0x08;

pub const SDHCI_TRANSFER_MODE: u32 = 0x0C;
pub const SDHCI_TRNS_DMA: u32 = 0x01;
pub const SDHCI_TRNS_BLK_CNT_EN: u32 = 0x02;
pub const SDHCI_TRNS_AUTO_CMD12: u32 = 0x04;
pub const SDHCI_TRNS_AUTO_CMD23: u32 = 0x08;
pub const SDHCI_TRNS_AUTO_SEL: u32 = 0x0C;
pub const SDHCI_TRNS_READ: u32 = 0x10;
pub const SDHCI_TRNS_MULTI: u32 = 0x20;

pub const SDHCI_COMMAND: u32 = 0x0E;
pub const SDHCI_CMD_RESP_MASK: u32 = 0x03;
pub const SDHCI_CMD_CRC: u32 = 0x08;
pub const SDHCI_CMD_INDEX: u32 = 0x10;
pub const SDHCI_CMD_DATA: u32 = 0x20;
pub const SDHCI_CMD_ABORTCMD: u32 = 0xC0;

pub const SDHCI_CMD_RESP_NONE: u32 = 0x00;
pub const SDHCI_CMD_RESP_LONG: u32 = 0x01;
pub const SDHCI_CMD_RESP_SHORT: u32 = 0x02;
pub const SDHCI_CMD_RESP_SHORT_BUSY: u32 = 0x03;

pub fn SDHCI_MAKE_CMD(c: u32, f: u32) -> u32 {
    (((c & 0xff) << 8) | (f & 0xff))
}
pub fn SDHCI_GET_CMD(c: u32) -> u32 {
    ((c >> 8) & 0x3f)
}

pub const SDHCI_RESPONSE: u32 = 0x10;

pub const SDHCI_BUFFER: u32 = 0x20;

pub const SDHCI_PRESENT_STATE: u32 = 0x24;
pub const SDHCI_CMD_INHIBIT: u32 = 0x00000001;
pub const SDHCI_DATA_INHIBIT: u32 = 0x00000002;
pub const SDHCI_DOING_WRITE: u32 = 0x00000100;
pub const SDHCI_DOING_READ: u32 = 0x00000200;
pub const SDHCI_SPACE_AVAILABLE: u32 = 0x00000400;
pub const SDHCI_DATA_AVAILABLE: u32 = 0x00000800;
pub const SDHCI_CARD_PRESENT: u32 = 0x00010000;
pub const SDHCI_CARD_PRES_SHIFT: u32 = 16;
pub const SDHCI_CD_STABLE: u32 = 0x00020000;
pub const SDHCI_CD_LVL: u32 = 0x00040000;
pub const SDHCI_CD_LVL_SHIFT: u32 = 18;
pub const SDHCI_DATA_0_LVL_MASK: u32 =	0x00100000;
pub const SDHCI_CMD_LVL:	u32 =	0x01000000;
