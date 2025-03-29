# Turbin3 Solana Prerequisite - Part 1 (TypeScript)

This repository contains the TypeScript code developed for Part 1 of the Turbin3 Solana prerequisite assessment. It demonstrates fundamental interactions with the Solana blockchain using `@solana/web3.js` and `@coral-xyz/anchor`.

The core functionalities implemented include:
*   Generating Solana keypairs (`keygen.ts`).
*   Requesting SOL airdrops on Devnet (`airdrop.ts`).
*   Calculating fees and transferring the entire remaining balance of an account (`transfer.ts`).
*   Interacting with the Turbin3 prerequisite program on Devnet (`Trb3aEx85DW1cEEvoqEaBkMn1tfmNEEEPaKzLSu4YAv`) via its IDL using Anchor to submit enrollment data (`enroll.ts`).

*(Note: This repository also contains a small Rust utility in `rust_key_converter/` for converting key formats, which was related to an optional section of the original tutorial materials.)*

## Prerequisites

*   [Node.js](https://nodejs.org/) (LTS version recommended)
*   [Yarn](https://yarnpkg.com/) (Classic v1.x used here)

## Setup

1.  **Clone the repository:**
    ```bash
    git clone <your-repo-url> airdrop
    cd airdrop
    ```

2.  **Install dependencies:**
    ```bash
    yarn install
    ```

## Configuration: Wallet Files (IMPORTANT)

This project requires **two** wallet files containing secret keys in the Solana byte array format (e.g., `[12, 34, ..., 56]`).

**These files are NOT included in the repository and should NEVER be committed to Git.** The included `.gitignore` file explicitly ignores `*wallet.json` and OS files like `.DS_Store` to help prevent accidental commits.

You need to create these files manually in the root directory (`airdrop/`):

1.  **`dev-wallet.json`:** This wallet will be used for receiving airdrops and performing initial transfers.
    *   Run the keygen script: `yarn keygen`
    *   Copy the **byte array output** (the line starting with `[` and ending with `]`).
    *   Create a file named `dev-wallet.json` in the project root.
    *   Paste the copied byte array into this file. Make sure it's the only content.

2.  **`Turbin3-wallet.json`:** This wallet **must** be the one you originally registered with Turbin3 for the course application (`EY5GD8F4eFqzGKXB8mtAm1WS99SDNKnVd4qiwj9wjZR7` according to your form data). It will be used for the final enrollment submission (`enroll.ts`).
    *   Retrieve the secret key byte array for this specific wallet. (You might have saved this previously or need to export/convert it securely).
    *   Create a file named `Turbin3-wallet.json` in the project root.
    *   Paste the correct byte array into this file.

## Usage

All functionalities are executed using `yarn` scripts defined in `package.json`.

*   **Generate a new Keypair:**
    ```bash
    yarn keygen
    ```
    *(Use the output byte array for `dev-wallet.json` as described above)*

*   **Airdrop 2 SOL to `dev-wallet.json`:**
    ```bash
    yarn airdrop
    ```
    *(Note: Devnet airdrops can sometimes be slow or fail. Check the balance on the explorer or retry if the transaction link doesn't work immediately.)*

*   **Transfer ALL remaining SOL (minus fees) from `dev-wallet.json`:** (Requires `dev-wallet.json` to be funded)
    ```bash
    yarn transfer
    ```
    *(Note: This uses a hardcoded recipient address from the tutorial (`EY5G...`).)*

*   **Submit Enrollment to Turbin3 Program:** (Requires `Turbin3-wallet.json` to be funded and correct)
    ```bash
    yarn enroll
    ```
    *(**IMPORTANT:** You **must** edit the `github` variable inside `enroll.ts` to use your actual GitHub username you applied with (`NotRithik`) before running this!)*

## Key Concepts Covered

*   Solana Keypair generation (`@solana/web3.js`).
*   Connecting to Solana RPC nodes.
*   Requesting SOL airdrops.
*   Constructing, signing, and sending Transactions.
*   Using the System Program for SOL transfers.
*   Calculating transaction fees (`getFeeForMessage`).
*   Using Anchor (`@coral-xyz/anchor`) as a client framework.
*   Defining and using program IDLs in TypeScript.
*   Deriving Program Derived Addresses (PDAs) client-side.
*   Calling instructions on an on-chain program using Anchor methods.
*   Running TypeScript files directly using `ts-node`.

## Disclaimer

Do not bother looking for my actual wallets' private keys here, you won't find them :)
