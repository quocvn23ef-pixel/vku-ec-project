#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    ContractAlreadyExists = 1,
    ContractNotFound = 2,
    InvalidDepositAmount = 3,
    InvalidDeductionAmount = 4,
    InvalidStatus = 5,
    UnauthorizedCaller = 6,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DepositStatus {
    Created = 0,
    Deposited = 1,
    DeductionRecorded = 2,
    RefundConfirmed = 3,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositAgreement {
    pub id: u64,
    pub landlord: Address,
    pub student: Address,
    pub deposit_amount: i128,
    pub deduction_amount: i128,
    pub refund_amount: i128,
    pub deduction_reason: String,
    pub status: DepositStatus,
    pub updated_at: u64,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Agreement(u64),
}

#[contract]
pub struct RentDepositContract;

#[contractimpl]
impl RentDepositContract {
    // Chủ trọ tạo hợp đồng đặt cọc mới với sinh viên và số tiền cọc ban đầu.
    pub fn create_deposit(
        env: Env,
        id: u64,
        landlord: Address,
        student: Address,
        deposit_amount: i128,
    ) -> Result<(), ContractError> {
        landlord.require_auth();

        if deposit_amount <= 0 {
            return Err(ContractError::InvalidDepositAmount);
        }

        let key = DataKey::Agreement(id);
        if env.storage().persistent().has(&key) {
            return Err(ContractError::ContractAlreadyExists);
        }

        let agreement = DepositAgreement {
            id,
            landlord,
            student,
            deposit_amount,
            deduction_amount: 0,
            refund_amount: deposit_amount,
            deduction_reason: String::from_str(&env, ""),
            status: DepositStatus::Created,
            updated_at: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&key, &agreement);
        Ok(())
    }

    // Sinh viên xác nhận đã chuyển tiền đặt cọc cho chủ trọ.
    pub fn confirm_deposit(env: Env, id: u64, student: Address) -> Result<(), ContractError> {
        student.require_auth();

        let key = DataKey::Agreement(id);
        let mut agreement: DepositAgreement = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(ContractError::ContractNotFound)?;

        if agreement.student != student {
            return Err(ContractError::UnauthorizedCaller);
        }

        if agreement.status != DepositStatus::Created {
            return Err(ContractError::InvalidStatus);
        }

        agreement.status = DepositStatus::Deposited;
        agreement.updated_at = env.ledger().timestamp();

        env.storage().persistent().set(&key, &agreement);
        Ok(())
    }

    // Chủ trọ ghi nhận khoản tiền khấu trừ và lý do khi sinh viên trả phòng.
    pub fn record_deduction(
        env: Env,
        id: u64,
        landlord: Address,
        deduction_amount: i128,
        deduction_reason: String,
    ) -> Result<(), ContractError> {
        landlord.require_auth();

        let key = DataKey::Agreement(id);
        let mut agreement: DepositAgreement = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(ContractError::ContractNotFound)?;

        if agreement.landlord != landlord {
            return Err(ContractError::UnauthorizedCaller);
        }

        if agreement.status != DepositStatus::Deposited {
            return Err(ContractError::InvalidStatus);
        }

        if deduction_amount < 0 || deduction_amount > agreement.deposit_amount {
            return Err(ContractError::InvalidDeductionAmount);
        }

        agreement.deduction_amount = deduction_amount;
        agreement.refund_amount = agreement.deposit_amount - deduction_amount;
        agreement.deduction_reason = deduction_reason;
        agreement.status = DepositStatus::DeductionRecorded;
        agreement.updated_at = env.ledger().timestamp();

        env.storage().persistent().set(&key, &agreement);
        Ok(())
    }

    // Sinh viên xác nhận đồng ý với khoản khấu trừ và số tiền được hoàn lại.
    pub fn confirm_refund(env: Env, id: u64, student: Address) -> Result<(), ContractError> {
        student.require_auth();

        let key = DataKey::Agreement(id);
        let mut agreement: DepositAgreement = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(ContractError::ContractNotFound)?;

        if agreement.student != student {
            return Err(ContractError::UnauthorizedCaller);
        }

        if agreement.status != DepositStatus::DeductionRecorded {
            return Err(ContractError::InvalidStatus);
        }

        agreement.status = DepositStatus::RefundConfirmed;
        agreement.updated_at = env.ledger().timestamp();

        env.storage().persistent().set(&key, &agreement);
        Ok(())
    }

    // Xem chi tiết thông tin và trạng thái hiện tại của một hợp đồng cọc.
    pub fn get_deposit(env: Env, id: u64) -> Result<DepositAgreement, ContractError> {
        let key = DataKey::Agreement(id);
        env.storage()
            .persistent()
            .get(&key)
            .ok_or(ContractError::ContractNotFound)
    }
}