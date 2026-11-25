# Project Title
**RentDeposit** – Transparent Student Rental Deposit Management Smart Contract on Stellar

---

## Project Description
**RentDeposit** is a Soroban smart contract built on the Stellar blockchain to record and verify rental deposit agreements between landlords and students. In student housing, disputes frequently arise at check-out regarding deposit deductions and final refund amounts. This smart contract eliminates ambiguity by storing the initial deposit amount, itemized deduction amounts, deduction reasons, and timestamps on-chain, requiring explicit cryptographic authorization (`require_auth`) from both the landlord and the student at each stage of the rental lifecycle.

---

## Project Vision
To establish a transparent, tamper-proof, and trust-minimized standard for student rental agreements where both landlords and tenants have clear, verifiable proof of deposit terms, check-out deductions, and final refund settlements without relying on easily lost paper receipts or informal text messages.

---

## Key Features
* **On-Chain Deposit Agreement Creation:** Landlords can initialize a unique deposit record (`create_deposit`) specifying the agreement ID, landlord address, student address, and positive deposit amount (`i128`).
* **Two-Party Cryptographic Confirmation:** Enforces strict role-based authorization—only the designated student can confirm the deposit payment (`confirm_deposit`) and final refund (`confirm_refund`), and only the designated landlord can record check-out deductions (`record_deduction`).
* **Strict Lifecycle State Machine:** Enforces a sequential workflow (`Created` $\rightarrow$ `Deposited` $\rightarrow$ `DeductionRecorded` $\rightarrow$ `RefundConfirmed`), rejecting any out-of-order actions with custom contract errors (`ContractError`).
* **Automated Refund Calculation & Validation:** Automatically calculates `refund_amount` (`deposit_amount - deduction_amount`) when a landlord logs a deduction and prevents deductions from exceeding the original deposit amount.
* **Immutable Audit Trail & Querying:** Records the ledger timestamp (`updated_at`) on every state transition and allows anyone to inspect the full agreement details via `get_deposit`.

---

## Usage Instructions

### 1. Build the Contract
Compile the Rust code into a WebAssembly (`.wasm`) binary using the Stellar CLI:
```bash
stellar contract build