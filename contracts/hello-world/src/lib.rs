#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, String, Symbol, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Content {
    pub owner: Address,
    pub title: String,
    pub uri: String,
}

#[contracttype]
pub enum ContentKey {
    Content(u64),
    Count,
}

#[contract]
pub struct DigitalRightsContract;

#[contractimpl]
impl DigitalRightsContract {
    pub fn register_content(env: Env, owner: Address, title: String, uri: String) -> u64 {
        owner.require_auth();

        let mut count: u64 = env.storage().instance().get(&ContentKey::Count).unwrap_or(0);
        count += 1;

        let content = Content {
            owner: owner.clone(),
            title,
            uri,
        };

        env.storage().instance().set(&ContentKey::Content(count), &content);
        env.storage().instance().set(&ContentKey::Count, &count);

        count
    }

    pub fn transfer_ownership(env: Env, content_id: u64, new_owner: Address, sender: Address) {
        sender.require_auth();

        let mut content: Content = env
            .storage()
            .instance()
            .get(&ContentKey::Content(content_id))
            .expect("Content not found");

        if content.owner != sender {
            panic!("Only the current owner can transfer ownership");
        }

        content.owner = new_owner;
        env.storage().instance().set(&ContentKey::Content(content_id), &content);
    }

    pub fn get_content(env: Env, content_id: u64) -> Content {
        env.storage()
            .instance()
            .get(&ContentKey::Content(content_id))
            .expect("Content not found")
    }

    pub fn total_registered(env: Env) -> u64 {
        env.storage().instance().get(&ContentKey::Count).unwrap_or(0)
    }
}
