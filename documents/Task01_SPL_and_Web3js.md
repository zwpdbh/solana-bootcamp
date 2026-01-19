# SPL and Web3.js 

## Notes 

- PDA, ATA, Token, Mint account, SPL, Token2022
- What is the differences between mint account, token account and associated token account?
  - **Mint** = what the token is  
  - **Token Account** = where tokens are held  
  - **Associated Token Account (ATA)** = the *standard, predictable way* to get a token account for a user + mint pair

- **All data on the Solana network is stored in accounts.**
  - Program account 
  - Data account 
    - An associated token account (ATA) is a token account that follows a specific derivation path, making it easily discoverable.
  
- My confusing parts:
  - "Token" can refer to any fungible asset on Solana, but commonly refers to SPL tokens.
  - What is the relationshipt between accounts and token (SPL token) , so account store information, and token is one of them?
    - Accounts store data and SOL balance.
    - SPL tokens are represented by token accounts, which are a specific type of account that stores token balances.
    - Token accounts are specialized accounts that track token balances.
    - Tokens themselves are the digital assets represented by data in token accounts.
  - **Tokens don't exist independently - they're just data in token accounts.**
    - SPL tokens are data stored in token accounts, which are specialized data accounts.
    - "SPL tokens are represented by token accounts" means that the token's existence and balance are embodied in these accounts. 
    - So tokens are both stored in and represented by token accounts. 
    - The data in the account includes the token balance, owner, and other metadata.

## References 
- [铸造一个 SPL 代币](https://learn.blueshift.gg/zh-CN/challenges/typescript-mint-an-spl-token)