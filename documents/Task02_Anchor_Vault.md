# Anchor 


## Concepts 

- PDA
  - Program Derived Address, is derived from a program ID + seeds
  - why use it? 
    - To store program state (like a "database record")
    - To allow your program to act as a signer for that account during CPI. 
- CPI 
  - Cross-Program Invocation, lets your program call instructions in other programs (e.g., Token Program, System Program).
  - Your program prepares accounts + data, then invokes another program.
  - You can pass PDAs as signers in CPI, even though they have no private key!
  - How signing works in CPI
    - If you pass a PDA as a signer, you must provide the seeds used to derive it.
    - The runtime re-derives the PDA and confirms it matches → this acts as the "signature".

- System Program
  - The System Program (Pubkey::new_from_array([0u8; 32])) is a built-in Solana program that handles:
    - Only a signer can call `SystemProgram::transfer` to send SOL from their account.

- Signer 
  - A signer is an account that cryptographically authorizes a transaction or instruction.
  - In a transaction, signers are accounts whose corresponding private keys signed the transaction.


## Imagine building a token vault

1. User sends a transaction signed by their wallet (a signer).
2. Your program derives a PDA (e.g., vault_user123) to store vault state.
3. Your program performs a CPI to the Token Program to
   - Transfer tokens from user (user is signer)
   - To the vault’s associated token account (controlled by PDA)
4. To authorize the transfer out of the vault later, your program uses the PDA as a signer in another CPI—by passing the derivation seeds.

## Program Accounts 



## References 

- [Task02: Anchor Vault](https://learn.blueshift.gg/en/challenges/anchor-vault)
- [Anchor 101](https://learn.blueshift.gg/en/courses/anchor-for-dummies/anchor-101)
- [Anchor Accounts](https://learn.blueshift.gg/en/courses/anchor-for-dummies/anchor-accounts)
- [Solana/更多开发者工具/Anchor 环境搭建](https://accu.cc/content/solana/tool_anchor_install/)