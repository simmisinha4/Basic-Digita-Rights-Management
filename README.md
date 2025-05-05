# Basic Digital Rights Management

## Project Description

This project implements a simple digital rights management system using Soroban smart contracts. It allows creators to register digital content and transfer ownership rights securely on-chain.

## Project Vision

To provide a decentralized, transparent, and tamper-proof way of managing digital content rights—empowering creators and reducing infringement risks in a trustless environment.

## Key Features

- **Content Registration**: Creators can register their work with title and URI metadata.
- **Ownership Tracking**: The contract stores and verifies ownership of each piece of content.
- **Ownership Transfer**: Owners can transfer their content rights to another address.
- **Content Query**: Anyone can fetch content metadata and verify ownership.
- **Total Registry Count**: Retrieve the number of registered digital works on-chain.

## Contract Details

- Contract Address: CAINGJTLHT3HGPR76CMDI54OT4C5ME6IH72YSIXNCIM2U6OVKKME7K2H

- ![Screenshot (84)](https://github.com/user-attachments/assets/ab4cdd5d-f09d-4be3-ae06-de435556561a)


- `register_content(owner: Address, title: String, uri: String) -> u64`  
  Registers new content and assigns a unique content ID.

- `transfer_ownership(content_id: u64, new_owner: Address, sender: Address)`  
  Transfers content ownership from the current owner to a new address.

- `get_content(content_id: u64) -> Content`  
  Returns metadata and owner address of the specified content.

- `total_registered() -> u64`  
  Returns the total number of content items registered in the system.
