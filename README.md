# Solana Fellowship Assignment – Rust HTTP Server

## What This Repo Does

This repository implements a **Rust-based HTTP server** for Solana-related operations, designed for the Solana Fellowship assignment. The server exposes a set of RESTful endpoints for:

- Generating Solana keypairs
- Creating and minting SPL tokens (constructing valid on-chain instructions)
- Signing and verifying messages using Ed25519
- Constructing SOL and SPL token transfer instructions

The server does **not** submit transactions to the blockchain, but instead returns the necessary instructions and data for clients to use in their own Solana workflows.

---

## Folder Structure

```
rust_deploy/
├── src/
│   ├── main.rs                # Application entrypoint, HTTP server setup
│   │   ├── keypair.rs         # /keypair endpoint
│   │   ├── token_create.rs    # /token/create endpoint
│   │   ├── token_mint.rs      # /token/mint endpoint
│   │   ├── send_sol.rs        # /send/sol endpoint
│   │   ├── send_token.rs      # /send/token endpoint
│   │   ├── message_sign.rs    # /message/sign endpoint
│   │   ├── message_verify.rs  # /message/verify endpoint
│   │   └── mod.rs             # Endpoint module re-exports
│   └── helpers/               # Shared types and utilities
│       ├── global_structs.rs  # Common response/account structs
│       └── mod.rs             # Helper module re-exports
├── tests/
│   ├── test-file.test.js      # Main test suite for all endpoints
│   ├── package.json           # Test dependencies (Jest, Solana web3, SPL Token, etc.)
│   └── jest.config.js         # Jest configuration
├── Cargo.toml                 # Rust dependencies and project metadata
└── README.md                  # Project documentation (this file)
```

---

## Solution Overview

### Server & Routing

- Uses [Actix Web](https://actix.rs/) for the HTTP server and routing.
- All endpoints are registered in `main.rs` using `.service(...)`.
- Each endpoint is implemented in its own file under `src/endpoints/`, following RESTful conventions.

### Endpoints

- **Keypair Generation:** Returns a new Solana keypair (base58-encoded).
- **Token Creation & Minting:** Constructs SPL token instructions (not sent to chain).
- **Message Signing/Verification:** Uses Ed25519 for cryptographic operations, with base58/base64 encoding as appropriate.
- **SOL & Token Transfers:** Constructs valid Solana and SPL token transfer instructions, including proper associated token account derivation.

### Shared Types

- Common response and account metadata structures are defined in `src/helpers/global_structs.rs` and re-exported for use in endpoints.
- All responses follow a consistent format with `success`, `data`, or `error` fields.

### Error Handling

- All endpoints validate input fields and return a 400 error with a descriptive message if validation fails.
- No private keys are stored on the server; all cryptographic operations are stateless and ephemeral.

### Security & Best Practices

- Uses only standard, audited cryptographic libraries (e.g., `ed25519-dalek`, `solana-sdk`).
- No sensitive data is persisted.
- All input is validated and all errors are handled gracefully to avoid information leakage.

---

## Overview

Build a **Rust-based HTTP server** exposing **Solana-related endpoints**. The server provides functionality to generate keypairs, handle SPL tokens, sign/verify messages, and construct valid on-chain instructions.

## HTTP-URL

```
https://your-url
```

## Response Format

All endpoints return JSON responses in the following format:

### Success Response (Status 200)

```json
{
  "success": true,
  "data": {
    /* endpoint-specific result */
  }
}
```

### Error Response (Status: 400)

```json
{
  "success": false,
  "error": "Description of error"
}
```

## Endpoints Specification

### 1. Generate Keypair

Generate a new Solana keypair.

**Endpoint:** `POST /keypair`

**Response:**

```json
{
  "success": true,
  "data": {
    "pubkey": "base58-encoded-public-key",
    "secret": "base58-encoded-secret-key"
  }
}
```

### 2. Create Token

Create a new SPL token initialise mint instruction.

**Endpoint:** `POST /token/create`

**Request:**

```json
{
  "mintAuthority": "base58-encoded-public-key",
  "mint": "base58-encoded-public-key",
  "decimals": 6
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "program_id": "string",
    "accounts": [{
      "pubkey": "pubkey",
      "is_signer": boolean,
      "is_writable": boolean
    }...],
    "instruction_data": "base64-encoded-data"
  }
}
```

### 3. Mint Token

Create a mint-to instruction for SPL tokens.

**Endpoint:** `POST /token/mint`

**Request:**

```json
{
  "mint": "mint-address",
  "destination": "destination-user-address",
  "authority": "authority-address",
  "amount": 1000000
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "program_id": "string",
    "accounts": [
      {
        "pubkey": "pubkey",
        "is_signer": false,
        "is_writable": true
      }...
    ],
    "instruction_data": "base64-encoded-data"
  }
}
```

### 4. Sign Message

Sign a message using a private key.

**Endpoint:** `POST /message/sign`

**Request:**

```json
{
  "message": "Hello, Solana!",
  "secret": "base58-encoded-secret-key"
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "signature": "base64-encoded-signature",
    "public_key": "base58-encoded-public-key",
    "message": "Hello, Solana!"
  }
}
```

**Error Response (Missing Fields):**

```json
{
  "success": false,
  "error": "Missing required fields"
}
```

### 5. Verify Message

Verify a signed message.

**Endpoint:** `POST /message/verify`

**Request:**

```json
{
  "message": "Hello, Solana!",
  "signature": "base64-encoded-signature",
  "pubkey": "base58-encoded-public-key"
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "valid": true,
    "message": "Hello, Solana!",
    "pubkey": "base58-encoded-public-key"
  }
}
```

### 6. Send SOL

Create a SOL transfer instruction. Should only process valid inputs.

**Endpoint:** `POST /send/sol`

**Request:**

```json
{
  "from": "sender-address",
  "to": "recipient-address",
  "lamports": 100000
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "program_id": "respective program id",
    "accounts": ["address of first account", "address of second account"],
    "instruction_data": "instruction_data"
  }
}
```

### 7. Send Token

Create an SPL token transfer instruction.

**Endpoint:** `POST /send/token`

**Request:**

```json
{
  "destination": "destination-user-address",
  "mint": "mint-address",
  "owner": "owner-address",
  "amount": 100000
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "program_id": "respective program id",
    "accounts": [
      {
        "pubkey": "pubkey",
        "is_signer": boolean
      }
    ],
    "instruction_data": "instruction_data"
  }
}
```

## Technical Details

### Signature Implementation

- Uses Ed25519 for signing/verification
- Base58 encoding for public/private keys
- Base64 encoding for signatures

### Error Handling

- Detailed error messages in response
- Proper validation of all input fields
- Consistent error message format

### Security Considerations

- No private keys stored on server
- All cryptographic operations use standard libraries
- Input validation for all endpoints
- Proper error handling to avoid information leakage

## Testing

- The `tests/` folder contains a comprehensive test suite (`test-file.test.js`) that covers all endpoints, validates both success and error cases, and checks the structure and content of returned instructions.
- Tests are run with the below command from from the `tests/` directory.

```shell
HTTP_URL=http://0.0.0.0:8080 jest test-file.test.js
```
