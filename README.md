# Decentralized Messaging

## Project Description

The Decentralized Messaging smart contract is a blockchain-based communication system built on the Stellar/Soroban platform. This contract enables users to send, receive, and manage messages in a completely decentralized manner without relying on centralized servers or intermediaries. Each message is stored immutably on the blockchain with cryptographic authentication, ensuring privacy, security, and censorship resistance.

The contract provides core messaging functionalities including sending messages between addresses, retrieving messages with proper authorization, marking messages as read, and tracking message counts. All interactions are authenticated using Stellar addresses, ensuring that only authorized parties can access their messages.

## Project Vision

Our vision is to revolutionize digital communication by creating a truly decentralized, censorship-resistant messaging platform that puts users in complete control of their data. In an era where centralized messaging platforms can be compromised, censored, or shut down, blockchain-based messaging offers an alternative that is:

- **Immutable**: Messages are permanently stored on the blockchain
- **Censorship-resistant**: No central authority can block or delete messages
- **Privacy-focused**: Only sender and receiver can access message content
- **Trustless**: No need to trust third-party service providers
- **Transparent**: All message transactions are verifiable on-chain

We envision this contract as the foundation for a new generation of communication applications where users truly own their conversations and data.

## Key Features

### 1. Secure Message Sending
Send messages between any two Stellar addresses with automatic authentication using require_auth() to ensure sender verification. Each message receives a unique ID for easy tracking and retrieval, with timestamps recorded automatically for message chronology.

### 2. Privacy-Controlled Message Retrieval
Only the sender or receiver can access a specific message. Authorization checks prevent unauthorized access. Messages are retrieved using unique message IDs with built-in error handling for non-existent messages.

### 3. Read Status Management
Receivers can mark messages as read, and only the intended receiver can update read status. This helps users track which messages they've viewed and prevents unauthorized status modifications.

### 4. Message Count Tracking
A global counter tracks total messages sent on the platform, useful for analytics and platform statistics, providing transparency about platform usage.

### 5. Persistent Storage
Extended TTL (Time To Live) ensures long-term message availability. Messages remain accessible even after extended periods through a reliable storage mechanism using Soroban's instance storage.

### 6. Event Logging
Important actions are logged for transparency, helping with debugging and auditing while providing clear feedback on contract operations.

## Future Scope

### Short-term Enhancements
- **Group Messaging**: Implement functionality for multiple recipients and group conversations
- **Message Encryption**: Add end-to-end encryption for message content
- **Message Deletion**: Allow users to delete their sent messages (soft delete)
- **Inbox Management**: Create functions to list all messages for a specific user
- **Message Filtering**: Add ability to filter messages by sender, date, or read status

### Medium-term Features
- **Attachments Support**: Enable sending files and media alongside text messages
- **Message Reactions**: Allow users to react to messages with emojis
- **Message Threading**: Implement conversation threads and reply functionality
- **User Profiles**: Add profile management for users including display names and avatars
- **Message Search**: Implement search functionality across user's messages
- **Notifications**: Create a notification system for new messages

### Long-term Vision
- **Cross-chain Messaging**: Enable messaging across different blockchain networks
- **AI Integration**: Implement smart message categorization and spam filtering
- **Decentralized Identity**: Integration with decentralized identity systems (DID)
- **Token-gated Channels**: Create premium messaging channels accessible via token ownership
- **DAO Governance**: Implement community governance for protocol upgrades
- **Mobile SDK**: Develop SDKs for easy integration into mobile applications
- **Message Marketplace**: Allow users to monetize their expertise through paid messaging
- **Multi-signature Messages**: Require multiple approvals for sensitive communications

### Technical Improvements
- Optimize gas costs for message operations
- Implement batch message sending
- Add pagination for message retrieval
- Create indexing solutions for faster message queries
- Implement message compression for storage efficiency
- Add rate limiting to prevent spam

---

## Getting Started

### Prerequisites
- Rust and Cargo installed
- Soroban CLI tools
- Stellar account with testnet funds

### Installation
```bash
# Clone the repository
git clone <repository-url>

# Build the contract
soroban contract build

# Deploy to testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/messaging_contract.wasm \
  --source <your-secret-key> \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

### Usage Example
```bash
# Send a message
soroban contract invoke \
  --id <contract-id> \
  --source-account <sender-account> \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- \
  send_message \
  --sender <sender-address> \
  --receiver <receiver-address> \
  --content "Hello, decentralized world!"
```

## Contributing

We welcome contributions from the community! Please read our contributing guidelines and submit pull requests for any enhancements.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contact

For questions, suggestions, or collaboration opportunities, please open an issue on GitHub or reach out to the development team.

---

## Contract Details
CAHJBN5XMBYO67ICPCUGYO5QEZCZBXVF3XVBHA4I3QXUAOK5ONTN355V
![alt text](<Screenshot 2025-10-29 at 3.19.29 PM.png>)



