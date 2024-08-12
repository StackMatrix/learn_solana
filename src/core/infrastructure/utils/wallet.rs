// use std::str::FromStr;
// use solana_client::rpc_client::RpcClient;
// use solana_program::native_token::LAMPORTS_PER_SOL;
// use solana_program::pubkey::Pubkey;
//
// /// Struct Account 账号信息
// ///
// /// @Param balance 余额
// /// @Param pub_key 钱包公钥
// pub struct Wallet {
//     pub balance: u64,
//     pub pub_key: String
// }
//
// impl Wallet {
//     pub fn default() -> Self {
//         Self {
//             balance: 0,
//             pub_key: String::new(),
//         }
//     }
//
//     /// @Description 获取钱包余额
//     ///
//     /// @Param rpc_client
//     pub fn get_balance(&mut self, rpc_client: &RpcClient) -> Result<(), String> {
//         let pub_key = Pubkey::from_str(&self.pub_key)
//             .map_err(|e| format!("-[Wallet] Failed to parse pub_key: {}", e))?;
//
//         self.balance = rpc_client
//             .get_balance(&pub_key)
//             .map_err(|e| format!("-[Wallet] Failed to get balance: {}", e))?;
//
//         Ok(())
//     }
//
//     /// @Description 将钱包余额转换为 Solana
//     pub fn balance_convert_sol(&self) -> f64 {
//         self.balance as f64 / LAMPORTS_PER_SOL as f64
//     }
// }
