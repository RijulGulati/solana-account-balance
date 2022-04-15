//! A very simple library to fetch Account Balance from Solana Clusters.

use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

/// Contains Account balance in Lamports and SOL.
#[derive(Debug)]
pub struct SolanaBalance {
    pub lamports: u64,
    pub sol: f64,
}

/// Contains any error reported.
#[derive(Debug)]
pub struct SolanaError {
    pub error: String,
}

/// Available solana clusters.
pub enum Cluster {
    /// The Testnet Cluster.
    Testnet,
    /// The Devnet Cluster.
    Devnet,
    /// The Mainnet Beta Cluster.
    MainnetBeta,
}

impl Cluster {
    /// method to get endpoint URL for cluster.
    fn endpoint(&self) -> &str {
        match self {
            &Cluster::Devnet => "https://api.devnet.solana.com",
            &Cluster::MainnetBeta => "https://api.mainnet-beta.solana.com",
            &Cluster::Testnet => "https://api.testnet.solana.com",
        }
    }
}

/// Function to get account balance from Solana Cluster
pub fn get_solana_balance(pubkey: &str, cluster: Cluster) -> Result<SolanaBalance, SolanaError> {
    let rpc = RpcClient::new(String::from(cluster.endpoint()));
    let pubkey = match Pubkey::from_str(pubkey) {
        Ok(key) => key,
        Err(err) => {
            return Err(SolanaError {
                error: err.to_string(),
            });
        }
    };

    match rpc.get_account(&pubkey) {
        Ok(acc) => {
            let balance: SolanaBalance = SolanaBalance {
                lamports: acc.lamports,
                sol: (acc.lamports as f64) / 1000000000.0,
            };
            Ok(balance)
        }

        Err(err) => {
            return Err(SolanaError {
                error: err.to_string(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use solana_sdk::pubkey::ParsePubkeyError;

    use super::*;

    const CORRECT_ACC_ADDRESS: &str = "9aavjzd4iAbiJHawgS7kunfCJefSRRVKso61vzAX9Ho5";
    const INCORRECT_ACC_ADDRESS: &str = "wrongaddress";
    const ACCOUNT_NOT_FOUND: &str = "888vjzd4iAbiJHawgS7kunfCJefSRRVKso61vzAX9111";

    #[test]
    fn get_balance() {
        let result = get_solana_balance(CORRECT_ACC_ADDRESS, Cluster::Devnet).unwrap();
        assert_eq!(result.lamports, 599985000);
        assert_eq!(result.sol, 0.599985);
    }

    #[test]
    fn invalid_pubkey() {
        let result = get_solana_balance(INCORRECT_ACC_ADDRESS, Cluster::Devnet)
            .err()
            .unwrap();
        assert_eq!(result.error, ParsePubkeyError::WrongSize.to_string());
    }

    #[test]
    fn acc_not_found() {
        let result = get_solana_balance(ACCOUNT_NOT_FOUND, Cluster::Devnet)
            .err()
            .unwrap();
        assert_eq!(
            result.error,
            format!("AccountNotFound: pubkey={}", ACCOUNT_NOT_FOUND)
        );
    }
}
