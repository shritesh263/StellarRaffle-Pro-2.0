// raffle.rs — UniversalRaffle Pro Expansion Contract
// This module contains the extended raffle contract with multi-ticket,
// streak tracking, and leaderboard support. It is compiled as part of
// the crate but kept separate for clarity.

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short,
    token, Address, Env, Symbol, Vec,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    DeadlinePassed = 3,
    NoParticipants = 4,
    InsufficientBalance = 5,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UniversalWinnerRecord {
    pub winner: Address,
    pub amount: i128,
    pub timestamp: u64,
    pub round_id: u32,
    pub tx_hash: Symbol,
}

#[contracttype]
pub enum UniversalDataKey {
    Admin,
    Token,
    TicketPrice,
    Deadline,
    RoundID,
    Participants,
    VaultBalance,
    History,
    TotalBought(Address),
    Streak(Address),
    MaxSingleWin(Address),
}

#[contract]
pub struct UniversalRaffle;

#[contractimpl]
impl UniversalRaffle {
    pub fn initialize(env: Env, admin: Address, token: Address, price: i128, deadline: u64) {
        if env.storage().instance().has(&UniversalDataKey::Admin) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&UniversalDataKey::Admin, &admin);
        env.storage().instance().set(&UniversalDataKey::Token, &token);
        env.storage().instance().set(&UniversalDataKey::TicketPrice, &price);
        env.storage().instance().set(&UniversalDataKey::Deadline, &deadline);
        env.storage().instance().set(&UniversalDataKey::RoundID, &1u32);
        env.storage().instance().set(&UniversalDataKey::VaultBalance, &0i128);
        env.storage()
            .instance()
            .set(&UniversalDataKey::Participants, &Vec::<Address>::new(&env));
        env.storage()
            .instance()
            .set(&UniversalDataKey::History, &Vec::<UniversalWinnerRecord>::new(&env));
    }

    pub fn buy_tickets(env: Env, buyer: Address, quantity: u32) -> Result<(), Error> {
        buyer.require_auth();
        let deadline: u64 = env
            .storage()
            .instance()
            .get(&UniversalDataKey::Deadline)
            .ok_or(Error::NotInitialized)?;
        if env.ledger().timestamp() >= deadline {
            return Err(Error::DeadlinePassed);
        }

        let price_per_ticket: i128 = env
            .storage()
            .instance()
            .get(&UniversalDataKey::TicketPrice)
            .unwrap();
        let mut total_cost = (quantity as i128) * price_per_ticket;

        // Bundle discount: 10% off for 10+ tickets
        if quantity >= 10 {
            total_cost = (total_cost * 9) / 10;
        }

        let token_addr: Address = env
            .storage()
            .instance()
            .get(&UniversalDataKey::Token)
            .unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&buyer, &env.current_contract_address(), &total_cost);

        let mut participants: Vec<Address> = env
            .storage()
            .instance()
            .get(&UniversalDataKey::Participants)
            .unwrap();
        for _ in 0..quantity {
            participants.push_back(buyer.clone());
        }
        env.storage()
            .instance()
            .set(&UniversalDataKey::Participants, &participants);

        let total_bought: u32 = env
            .storage()
            .instance()
            .get(&UniversalDataKey::TotalBought(buyer.clone()))
            .unwrap_or(0);
        env.storage().instance().set(
            &UniversalDataKey::TotalBought(buyer.clone()),
            &(total_bought + quantity),
        );

        Ok(())
    }

    pub fn draw_winner(env: Env) -> Result<Address, Error> {
        let deadline: u64 = env
            .storage()
            .instance()
            .get(&UniversalDataKey::Deadline)
            .unwrap();
        if env.ledger().timestamp() < deadline {
            return Err(Error::DeadlinePassed);
        }

        let participants: Vec<Address> = env
            .storage()
            .instance()
            .get(&UniversalDataKey::Participants)
            .unwrap();
        let count = participants.len();
        if count == 0 {
            return Err(Error::NoParticipants);
        }

        let winner_idx = (env.ledger().timestamp() % (count as u64)) as u32;
        let winner = participants.get(winner_idx).unwrap();

        let token_addr: Address = env
            .storage()
            .instance()
            .get(&UniversalDataKey::Token)
            .unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        let prize = token_client.balance(&env.current_contract_address());

        if prize > 0 {
            token_client.transfer(&env.current_contract_address(), &winner, &prize);
            let max_win: i128 = env
                .storage()
                .instance()
                .get(&UniversalDataKey::MaxSingleWin(winner.clone()))
                .unwrap_or(0);
            if prize > max_win {
                env.storage().instance().set(
                    &UniversalDataKey::MaxSingleWin(winner.clone()),
                    &prize,
                );
            }
        }

        let round_id: u32 = env
            .storage()
            .instance()
            .get(&UniversalDataKey::RoundID)
            .unwrap();
        let mut history: Vec<UniversalWinnerRecord> = env
            .storage()
            .instance()
            .get(&UniversalDataKey::History)
            .unwrap();
        history.push_back(UniversalWinnerRecord {
            winner: winner.clone(),
            amount: prize,
            timestamp: env.ledger().timestamp(),
            round_id,
            tx_hash: symbol_short!("DRAWN"),
        });
        if history.len() > 10 {
            history.remove(0);
        }
        env.storage()
            .instance()
            .set(&UniversalDataKey::History, &history);

        env.storage()
            .instance()
            .set(&UniversalDataKey::Participants, &Vec::<Address>::new(&env));
        env.storage()
            .instance()
            .set(&UniversalDataKey::RoundID, &(round_id + 1));
        env.storage()
            .instance()
            .set(&UniversalDataKey::Deadline, &(env.ledger().timestamp() + 3600));

        Ok(winner)
    }

    pub fn get_raffle_info(env: Env) -> (i128, u32, u32, u64) {
        let token_addr: Address = env
            .storage()
            .instance()
            .get(&UniversalDataKey::Token)
            .unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        let pool = token_client.balance(&env.current_contract_address());
        let participants: Vec<Address> = env
            .storage()
            .instance()
            .get(&UniversalDataKey::Participants)
            .unwrap_or(Vec::new(&env));
        let round_id: u32 = env
            .storage()
            .instance()
            .get(&UniversalDataKey::RoundID)
            .unwrap_or(0);
        let deadline: u64 = env
            .storage()
            .instance()
            .get(&UniversalDataKey::Deadline)
            .unwrap_or(0);
        (pool, participants.len(), round_id, deadline)
    }

    pub fn get_user_stats(env: Env, user: Address) -> (u32, u32, i128) {
        let total_bought: u32 = env
            .storage()
            .instance()
            .get(&UniversalDataKey::TotalBought(user.clone()))
            .unwrap_or(0);
        let streak: u32 = env
            .storage()
            .instance()
            .get(&UniversalDataKey::Streak(user.clone()))
            .unwrap_or(0);
        let max_win: i128 = env
            .storage()
            .instance()
            .get(&UniversalDataKey::MaxSingleWin(user))
            .unwrap_or(0);
        (total_bought, streak, max_win)
    }

    pub fn get_history(env: Env) -> Vec<UniversalWinnerRecord> {
        env.storage()
            .instance()
            .get(&UniversalDataKey::History)
            .unwrap_or(Vec::new(&env))
    }
}
