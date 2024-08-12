pub enum Address {
    DevNet,
    TestNet,
    MainNet,
    CustomPpc
}

impl From<Address> for String {
    // https://api.devnet.solana.com 开发网
    // https://api.testnet.solana.com 测试网
    // http://api.mainnet-beta.solana.com 主网
    fn from(platform: Address) -> Self {
        match platform {
            Address::DevNet => "https://api.devnet.solana.com".into(),
            Address::TestNet => "https://api.testnet.solana.com".into(),
            Address::MainNet => "http://api.mainnet-beta.solana.com".into(),
            Address::CustomPpc => platform.into(),
        }
    }
}