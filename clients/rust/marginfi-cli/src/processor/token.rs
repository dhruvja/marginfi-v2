use {
    crate::{config::Config, utils::process_transaction},
    anyhow::{anyhow, Result},
    solana_sdk::{
        message::Message,
        program_pack::Pack,
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
        system_instruction,
        transaction::Transaction,
    },
    spl_associated_token_account::{
        get_associated_token_address, instruction::create_associated_token_account_idempotent,
    },
};

/// Create a new SPL token mint
pub fn create_token(
    config: &Config,
    decimals: u8,
    mint_authority: Option<Pubkey>,
    freeze_authority: Option<Pubkey>,
) -> Result<()> {
    let rpc_client = config.mfi_program.rpc();
    let payer = &config.fee_payer;

    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();

    let mint_authority = mint_authority.unwrap_or_else(|| payer.pubkey());
    let freeze_authority_ref = freeze_authority.as_ref();

    // Calculate rent for mint account
    let mint_rent = rpc_client.get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)?;

    // Create mint account instruction
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint_pubkey,
        mint_rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );

    // Initialize mint instruction
    let init_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &mint_authority,
        freeze_authority_ref,
        decimals,
    )?;

    let instructions = vec![create_account_ix, init_mint_ix];
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let message = Message::new(&instructions, Some(&payer.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.partial_sign(&[payer, &mint_keypair], recent_blockhash);

    match process_transaction(&transaction, &rpc_client, config.get_tx_mode()) {
        Ok(sig) => {
            println!("Token mint created successfully!");
            println!("Mint address: {}", mint_pubkey);
            println!("Decimals: {}", decimals);
            println!("Mint authority: {}", mint_authority);
            if let Some(freeze_auth) = freeze_authority {
                println!("Freeze authority: {}", freeze_auth);
            }
            println!("Signature: {}", sig);
            Ok(())
        }
        Err(err) => {
            println!("Error creating token mint:\n{:#?}", err);
            Err(anyhow!("Error creating token mint"))
        }
    }
}

/// Mint tokens to a destination account
pub fn mint_tokens(config: &Config, mint: Pubkey, ui_amount: f64, to: Pubkey) -> Result<()> {
    let rpc_client = config.mfi_program.rpc();
    let payer = &config.fee_payer;

    // Get mint info to determine decimals
    let mint_account = rpc_client.get_account(&mint)?;
    let mint_data = spl_token::state::Mint::unpack(&mint_account.data)?;
    let decimals = mint_data.decimals;

    // Convert UI amount to native amount
    let amount = spl_token::ui_amount_to_amount(ui_amount, decimals);

    // Get or create associated token account for destination
    let destination_ata = get_associated_token_address(&to, &mint);

    let mut instructions = vec![];

    // Create ATA if it doesn't exist
    let create_ata_ix =
        create_associated_token_account_idempotent(&payer.pubkey(), &to, &mint, &spl_token::id());
    instructions.push(create_ata_ix);

    // Mint tokens instruction
    let mint_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint,
        &destination_ata,
        &payer.pubkey(), // Mint authority
        &[],
        amount,
    )?;
    instructions.push(mint_ix);

    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let message = Message::new(&instructions, Some(&payer.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.partial_sign(&[payer], recent_blockhash);

    match process_transaction(&transaction, &rpc_client, config.get_tx_mode()) {
        Ok(sig) => {
            println!("Tokens minted successfully!");
            println!("Mint: {}", mint);
            println!("Amount: {} (native: {})", ui_amount, amount);
            println!("Destination wallet: {}", to);
            println!("Destination ATA: {}", destination_ata);
            println!("Signature: {}", sig);
            Ok(())
        }
        Err(err) => {
            println!("Error minting tokens:\n{:#?}", err);
            Err(anyhow!("Error minting tokens"))
        }
    }
}

/// Transfer tokens to another account
pub fn transfer_tokens(config: &Config, mint: Pubkey, ui_amount: f64, to: Pubkey) -> Result<()> {
    let rpc_client = config.mfi_program.rpc();
    let payer = &config.fee_payer;

    // Get mint info to determine decimals
    let mint_account = rpc_client.get_account(&mint)?;
    let mint_data = spl_token::state::Mint::unpack(&mint_account.data)?;
    let decimals = mint_data.decimals;

    // Convert UI amount to native amount
    let amount = spl_token::ui_amount_to_amount(ui_amount, decimals);

    // Get source and destination ATAs
    let source_ata = get_associated_token_address(&payer.pubkey(), &mint);
    let destination_ata = get_associated_token_address(&to, &mint);

    let mut instructions = vec![];

    // Create destination ATA if it doesn't exist
    let create_ata_ix =
        create_associated_token_account_idempotent(&payer.pubkey(), &to, &mint, &spl_token::id());
    instructions.push(create_ata_ix);

    // Transfer tokens instruction
    let transfer_ix = spl_token::instruction::transfer(
        &spl_token::id(),
        &source_ata,
        &destination_ata,
        &payer.pubkey(),
        &[],
        amount,
    )?;
    instructions.push(transfer_ix);

    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let message = Message::new(&instructions, Some(&payer.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.partial_sign(&[payer], recent_blockhash);

    match process_transaction(&transaction, &rpc_client, config.get_tx_mode()) {
        Ok(sig) => {
            println!("Tokens transferred successfully!");
            println!("Mint: {}", mint);
            println!("Amount: {} (native: {})", ui_amount, amount);
            println!("From: {} (ATA: {})", payer.pubkey(), source_ata);
            println!("To: {} (ATA: {})", to, destination_ata);
            println!("Signature: {}", sig);
            Ok(())
        }
        Err(err) => {
            println!("Error transferring tokens:\n{:#?}", err);
            Err(anyhow!("Error transferring tokens"))
        }
    }
}

/// Get token balance for an account
pub fn get_balance(config: &Config, mint: Pubkey, owner: Option<Pubkey>) -> Result<()> {
    let rpc_client = config.mfi_program.rpc();
    let payer = &config.fee_payer;

    let owner = owner.unwrap_or_else(|| payer.pubkey());

    // Get mint info to determine decimals
    let mint_account = rpc_client.get_account(&mint)?;
    let mint_data = spl_token::state::Mint::unpack(&mint_account.data)?;
    let decimals = mint_data.decimals;

    // Get associated token account
    let ata = get_associated_token_address(&owner, &mint);

    // Try to get the token account
    match rpc_client.get_account(&ata) {
        Ok(account) => {
            let token_account = spl_token::state::Account::unpack(&account.data)?;
            let ui_amount = spl_token::amount_to_ui_amount(token_account.amount, decimals);

            println!("Token Balance");
            println!("=============");
            println!("Mint: {}", mint);
            println!("Owner: {}", owner);
            println!("Token Account (ATA): {}", ata);
            println!("Balance: {} (native: {})", ui_amount, token_account.amount);
            println!("Decimals: {}", decimals);
        }
        Err(_) => {
            println!("Token Balance");
            println!("=============");
            println!("Mint: {}", mint);
            println!("Owner: {}", owner);
            println!("Token Account (ATA): {} (not found)", ata);
            println!("Balance: 0");
            println!("Decimals: {}", decimals);
        }
    }

    Ok(())
}
