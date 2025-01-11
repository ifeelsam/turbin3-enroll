mod programs;
#[cfg(test)]
mod tests {
    use crate::programs::turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs, UpdateArgs};
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer, read_keypair_file},
    };
    use solana_sdk::system_program;
    use solana_sdk::bs58;
    use std::io::{self, BufRead};
    use solana_client::rpc_client::RpcClient;

    const RPC_URL: &str = "https://api.devnet.solana.com";
    #[test]
    fn keygen() { 
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        
        println!("{:?}", kp.to_bytes());
    }
    #[test]
    fn airdrop() {
        // let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");     

    }
    #[test]
    fn transfer_sol() {}
    #[test]
    fn base58_to_wallet() {

        println!("Input your private key as base58:");

        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap(); println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap(); println!("{:?}", wallet);
        
    }
    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin3-wallet.json").expect("cant find wallet");
        let prereq = Turbin3PrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);
        let args = CompleteArgs {
            github: b"ifeelsam".to_vec()
        };
        let blockhash = rpc_client.get_latest_blockhash().expect("cant get block hash");
        let transaction = Turbin3PrereqProgram::complete(&[&signer.pubkey(), &prereq, &system_program::id()], &args, Some(&signer.pubkey()), &[&signer], blockhash );
        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("failed to send the txn");
        print!("sucess!! https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }
    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:"); 
        let stdin = io::stdin(); 
        let wallet = stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').split(',') .map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();

        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string(); println!("{:?}", base58);

    }
}