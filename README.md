# solana-6551

## Abstract

Solana-6551 utilizes Solana's unique account model, permitting the creation of a unique Program Derived Account (PDA) for each NFT. allows NFTs to own assets and interact with applications, without requiring changes to existing smart contracts or infrastructure.

## Specification

### Overview

The system outlined in this proposal has two main components:

- Program to create PDA (Program-Derived Addresses) accounts for NFTs
- A common interface for NFT_PDA implementations

The following diagram illustrates the relationship between NFTs, NFT holders, token bound pdas, implement, and the Program:
![Solana-6551](./images/Solana-6551.png)