use crate::utils::{addr, u256};
use num_bigint::BigUint;
use primitive_types::U256;
use pyo3::prelude::*;
use revm::TransactTo;

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct Env(revm::Env);

#[pymethods]
impl Env {
    #[new]
    fn new(cfg: Option<CfgEnv>, block: Option<BlockEnv>, tx: Option<TxEnv>) -> Self {
        Env(revm::Env {
            cfg: cfg.unwrap_or_default().into(),
            block: block.unwrap_or_default().into(),
            tx: tx.unwrap_or_default().into(),
        })
    }
}

impl From<revm::Env> for Env {
    fn from(env: revm::Env) -> Self {
        Env(env)
    }
}

impl From<Env> for revm::Env {
    fn from(env: Env) -> Self {
        env.0
    }
}

#[pyclass]
#[derive(Debug, Default, Clone)]
pub struct TxEnv(pub revm::TxEnv);

#[pymethods]
impl TxEnv {
    #[new]
    pub fn new(
        caller: Option<&str>,
        gas_limit: Option<u64>,
        gas_price: Option<BigUint>,
        gas_priority_fee: Option<BigUint>,
        to: Option<&str>,
        value: Option<BigUint>,
        data: Option<Vec<u8>>,
        chain_id: Option<u64>,
        nonce: Option<u64>,
    ) -> PyResult<Self> {
        Ok(TxEnv(revm::TxEnv {
            caller: addr(caller.unwrap_or_default())?,
            gas_limit: gas_limit.unwrap_or(u64::MAX),
            gas_price: u256(gas_price.unwrap_or_default()),
            gas_priority_fee: gas_priority_fee.map(u256),
            transact_to: match to {
                Some(inner) => TransactTo::Call(addr(inner)?),
                // TODO: Figure out how to integrate CREATE2 here
                None => TransactTo::Create(revm::CreateScheme::Create),
            },
            value: u256(value.unwrap_or_default()),
            data: data.unwrap_or_default().into(),
            chain_id,
            nonce,
            // TODO: Add access list.
            ..Default::default()
        }))
    }
}

impl From<TxEnv> for revm::TxEnv {
    fn from(env: TxEnv) -> Self {
        env.0
    }
}

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct BlockEnv(revm::BlockEnv);

#[pymethods]
impl BlockEnv {
    #[new]
    fn new(
        number: Option<BigUint>,
        coinbase: Option<&str>,
        timestamp: Option<BigUint>,
        difficulty: Option<BigUint>,
        basefee: Option<BigUint>,
        gas_limit: Option<BigUint>,
    ) -> PyResult<Self> {
        Ok(BlockEnv(revm::BlockEnv {
            number: u256(number.unwrap_or_default()),
            coinbase: addr(coinbase.unwrap_or("0x0000000000000000000000000000000000000000"))?,
            timestamp: if let Some(timestamp) = timestamp {
                u256(timestamp)
            } else {
                U256::from(1)
            },
            difficulty: u256(difficulty.unwrap_or_default()),
            basefee: u256(basefee.unwrap_or_default()),
            gas_limit: u256(gas_limit.unwrap_or_else(|| u64::MAX.into())),
        }))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

impl From<BlockEnv> for revm::BlockEnv {
    fn from(env: BlockEnv) -> Self {
        env.0
    }
}

#[pyclass]
#[derive(Default, Clone, Debug)]
pub struct CfgEnv(revm::CfgEnv);

#[pymethods]
impl CfgEnv {
    #[new]
    fn new() -> Self {
        CfgEnv(revm::CfgEnv::default())
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

impl From<CfgEnv> for revm::CfgEnv {
    fn from(env: CfgEnv) -> Self {
        env.0
    }
}
