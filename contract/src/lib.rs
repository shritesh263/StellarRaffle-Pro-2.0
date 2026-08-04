#![no_std]
mod raffle;
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env, Vec, Option,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TicketTier {
    Bronze,
    Gold,
    Diamond,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WinnerRecord {
    pub winner: Address,
    pub amount: i128,
    pub timestamp: u64,
}

#[contracttype]
pub enum DataKey {
    Deadline,
    Participants,
    Token,
    Owner,
    VaultBalance,
    History,
}

// 5% platform fee in basis points
const PLATFORM_FEE_BPS: i128 = 500;
// 1% referral bonus in basis points
const REFERRAL_FEE_BPS: i128 = 100;

#[contract]
pub struct ProfessionalRaffle;

#[contractimpl]
impl ProfessionalRaffle {
    pub fn initialize(env: Env, owner: Address, token: Address, deadline: u64) {
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Deadline, &deadline);
        env.storage().instance().set(&DataKey::VaultBalance, &0i128);
        env.storage().instance().set(&DataKey::Participants, &Vec::<Address>::new(&env));
        env.storage().instance().set(&DataKey::History, &Vec::<WinnerRecord>::new(&env));
    }

    /// Buy a ticket for a raffle tier with optional referral.
    /// - Platform fee: 5% of price goes to vault
    /// - Referral bonus: 1% of price paid to referrer (deducted from vault)
    /// - Self-referral is silently ignored
    pub fn buy_ticket(env: Env, buyer: Address, tier: TicketTier, referrer: Option<Address>) {
        buyer.require_auth();
        let deadline: u64 = env.storage().instance().get(&DataKey::Deadline).unwrap();
        if env.ledger().timestamp() >= deadline {
            panic!("Lottery draw is in progress or ended.");
        }
        let price = match tier {
            TicketTier::Bronze => 5_000_000i128,
            TicketTier::Gold => 20_000_000i128,
            TicketTier::Diamond => 50_000_000i128,
        };
        let entries: u32 = match tier {
            TicketTier::Bronze => 1,
            TicketTier::Gold => 5,
            TicketTier::Diamond => 15,
        };

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&buyer, &env.current_contract_address(), &price);

        // Platform fee (5%) goes to vault
        let platform_fee = (price * PLATFORM_FEE_BPS) / 10_000;
        let current_vault: i128 = env.storage().instance().get(&DataKey::VaultBalance).unwrap();
        let mut new_vault = current_vault + platform_fee;

        // Referral reward (1%) — blocked if same as buyer (self-referral)
        if let Some(ref ref_addr) = referrer {
            if ref_addr != &buyer {
                let referral_reward = (price * REFERRAL_FEE_BPS) / 10_000;
                // Pay referral from contract balance (deduct from vault)
                new_vault -= referral_reward;
                token_client.transfer(&env.current_contract_address(), ref_addr, &referral_reward);
            }
        }

        env.storage().instance().set(&DataKey::VaultBalance, &new_vault);

        let mut participants: Vec<Address> = env.storage().instance().get(&DataKey::Participants).unwrap();
        for _ in 0..entries {
            participants.push_back(buyer.clone());
        }
        env.storage().instance().set(&DataKey::Participants, &participants);
    }

    pub fn draw_winner(env: Env) {
        let deadline: u64 = env.storage().instance().get(&DataKey::Deadline).unwrap();
        if env.ledger().timestamp() < deadline {
            panic!("Ongoing");
        }
        let participants: Vec<Address> = env.storage().instance().get(&DataKey::Participants).unwrap();
        let count = participants.len();
        if count == 0 {
            panic!("No players");
        }

        let winner_idx = (env.ledger().timestamp() % (count as u64)) as u32;
        let winner = participants.get(winner_idx).unwrap();

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        let vault_bal: i128 = env.storage().instance().get(&DataKey::VaultBalance).unwrap();
        let total_bal = token_client.balance(&env.current_contract_address());
        let prize = total_bal - vault_bal;

        if prize > 0 {
            token_client.transfer(&env.current_contract_address(), &winner, &prize);
        }

        let mut history: Vec<WinnerRecord> = env.storage().instance().get(&DataKey::History).unwrap();
        history.push_back(WinnerRecord {
            winner: winner.clone(),
            amount: prize,
            timestamp: env.ledger().timestamp(),
        });
        if history.len() > 10 {
            history.remove(0);
        }
        env.storage().instance().set(&DataKey::History, &history);
        env.storage().instance().set(&DataKey::Participants, &Vec::<Address>::new(&env));
        env.storage().instance().set(&DataKey::Deadline, &(env.ledger().timestamp() + 3600));
    }

    /// Owner withdraws accumulated platform fees from the vault
    pub fn withdraw_fees(env: Env, owner: Address) {
        owner.require_auth();
        let stored_owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        assert!(owner == stored_owner, "Unauthorized");

        let vault_bal: i128 = env.storage().instance().get(&DataKey::VaultBalance).unwrap();
        if vault_bal > 0 {
            let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
            let token_client = token::Client::new(&env, &token_addr);
            token_client.transfer(&env.current_contract_address(), &owner, &vault_bal);
            env.storage().instance().set(&DataKey::VaultBalance, &0i128);
        }
    }

    pub fn get_raffle_info(env: Env) -> (i128, u32, u64, i128) {
        let vault_bal: i128 = env.storage().instance().get(&DataKey::VaultBalance).unwrap_or(0);
        let participants: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Participants)
            .unwrap_or(Vec::new(&env));
        let deadline: u64 = env.storage().instance().get(&DataKey::Deadline).unwrap_or(0);
        // Return (pool_net, participants_count, deadline, vault_balance)
        (0i128, participants.len(), deadline, vault_bal)
    }

    pub fn get_winner_history(env: Env) -> Vec<WinnerRecord> {
        env.storage()
            .instance()
            .get(&DataKey::History)
            .unwrap_or(Vec::new(&env))
    }
}
