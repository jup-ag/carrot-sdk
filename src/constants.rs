use solana_pubkey::Pubkey;

// mints
pub const USDC_MINT: Pubkey =
    Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const USDT_MINT: Pubkey =
    Pubkey::from_str_const("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");
pub const PYUSD_MINT: Pubkey =
    Pubkey::from_str_const("2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo");

// mint atas
pub const USDC_VAULT_ATA: Pubkey =
    Pubkey::from_str_const("Gfedc4JEmMahEMBJXcXfLHWgNs9d7UzLPq1tkba5S11U");
pub const USDT_VAULT_ATA: Pubkey =
    Pubkey::from_str_const("Hpxgqa8dvk2jSfNgTfdYncxSE2YY2c52TTzPaH1V98RW");
pub const PYUSD_VAULT_ATA: Pubkey =
    Pubkey::from_str_const("4cugtfkFydmoPe9CZJ4wFZzDUEmGJFNaThvumYABTFDS");

// mint oracles
pub const USDC_ORACLE: Pubkey =
    Pubkey::from_str_const("Dpw1EAVrSB1ibxiDQyTAW6Zip3J4Btk2x4SgApQCeFbX");
pub const USDT_ORACLE: Pubkey =
    Pubkey::from_str_const("HT2PLQBcG5EiCcNSaMHAjSgd9F98ecpATbk4Sk5oYuM");
pub const PYUSD_ORACLE: Pubkey =
    Pubkey::from_str_const("9zXQxpYH3kYhtoybmZfUNNCRVuud7fY9jswTg1hLyT8k");

// crt vault
pub const CRT_MINT: Pubkey = Pubkey::from_str_const("CRTx1JouZhzSU6XytsE42UQraoGqiHgxabocVfARTy2s");
pub const CRT_VAULT: Pubkey =
    Pubkey::from_str_const("FfCRL34rkJiMiX5emNDrYp3MdWH2mES3FvDQyFppqgpJ");

// carrot protocol
pub const CARROT_PROGRAM: Pubkey =
    Pubkey::from_str_const("CarrotwivhMpDnm27EHmRLeQ683Z1PufuqEmBZvD282s");
pub const CARROT_LOG_PROGRAM: Pubkey =
    Pubkey::from_str_const("7Mc3vSdRWoThArpni6t5W4XjvQf4BuMny1uC8b6VBn48");

// other programs
pub const TOKEN_PROGRAM: Pubkey =
    Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const TOKEN_22_PROGRAM: Pubkey =
    Pubkey::from_str_const("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

// amm label
pub const AMM_LABEL: &str = "Carrot";
