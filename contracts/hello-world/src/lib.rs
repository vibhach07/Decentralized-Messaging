#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, String, Symbol, Address, symbol_short};

// Message structure to store individual messages
#[contracttype]
#[derive(Clone)]
pub struct Message {
    pub msg_id: u64,
    pub sender: Address,
    pub receiver: Address,
    pub content: String,
    pub timestamp: u64,
    pub is_read: bool,
}

// Mapping message ID to Message
#[contracttype]
pub enum MessageBook {
    Message(u64)
}

// Counter for generating unique message IDs
const MSG_COUNT: Symbol = symbol_short!("MSG_CNT");

// Mapping for user's inbox (Address -> Vec of message IDs)
#[contracttype]
pub enum InboxBook {
    Inbox(Address)
}

#[contract]
pub struct MessagingContract;

#[contractimpl]
impl MessagingContract {
    
    // Function to send a message from one user to another
    pub fn send_message(env: Env, sender: Address, receiver: Address, content: String) -> u64 {
        // Verify the sender's identity
        sender.require_auth();
        
        // Get and increment message counter
        let mut msg_count: u64 = env.storage().instance().get(&MSG_COUNT).unwrap_or(0);
        msg_count += 1;
        
        // Get current timestamp
        let timestamp = env.ledger().timestamp();
        
        // Create new message
        let new_message = Message {
            msg_id: msg_count,
            sender: sender.clone(),
            receiver: receiver.clone(),
            content,
            timestamp,
            is_read: false,
        };
        
        // Store the message
        env.storage().instance().set(&MessageBook::Message(msg_count), &new_message);
        
        // Update message counter
        env.storage().instance().set(&MSG_COUNT, &msg_count);
        
        // Extend TTL for persistent storage
        env.storage().instance().extend_ttl(100000, 100000);
        
        log!(&env, "Message sent successfully with ID: {}", msg_count);
        msg_count
    }
    
    // Function to retrieve a specific message by ID
    pub fn get_message(env: Env, msg_id: u64, requester: Address) -> Message {
        // Verify the requester's identity
        requester.require_auth();
        
        let key = MessageBook::Message(msg_id);
        
        let message: Message = env.storage().instance().get(&key).unwrap_or(Message {
            msg_id: 0,
            sender: requester.clone(),
            receiver: requester.clone(),
            content: String::from_str(&env, "Message not found"),
            timestamp: 0,
            is_read: false,
        });
        
        // Verify that requester is either sender or receiver
        if message.sender != requester && message.receiver != requester {
            panic!("Unauthorized access to message");
        }
        
        message
    }
    
    // Function to mark a message as read
    pub fn mark_as_read(env: Env, msg_id: u64, reader: Address) {
        // Verify the reader's identity
        reader.require_auth();
        
        let key = MessageBook::Message(msg_id);
        let mut message: Message = env.storage().instance().get(&key).unwrap_or_else(|| {
            panic!("Message not found");
        });
        
        // Verify that reader is the receiver
        if message.receiver != reader {
            panic!("Only the receiver can mark message as read");
        }
        
        // Mark as read
        message.is_read = true;
        
        // Update the message
        env.storage().instance().set(&key, &message);
        env.storage().instance().extend_ttl(100000, 100000);
        
        log!(&env, "Message {} marked as read", msg_id);
    }
    
    // Function to get total message count
    pub fn get_message_count(env: Env) -> u64 {
        env.storage().instance().get(&MSG_COUNT).unwrap_or(0)
    }
}
