use std::collections::{HashMap, HashSet};

use candid::{CandidType, Int, Nat, Principal};
use ic_cdk::{api, query, update};
use serde::{de::value::Error, Deserialize, Serialize};

#[derive(CandidType, Deserialize, Default)]
pub struct CanisterState {
    pub metadata: CanisterMetaData,
    pub certificates: HashMap<CertificateId, Certificate>,
    pub owners: HashMap<Principal, HashSet<CertificateId>>,
    pub owner_names: HashMap<String, Principal>,
    pub stats: CanisterStats,
}

/// Metadata for ICP NFT standard.
#[derive(CandidType, Deserialize, Clone, Default)]
pub struct CanisterMetaData {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub logo: Option<String>,
    pub created_at: u64,
    pub upgraded_at: u64,
    /// Admin accounts associated with the canister
    pub custodians: HashSet<Principal>,
}

pub type CertificateId = u64;

#[derive(CandidType, Deserialize)]
pub struct Certificate {
    pub metadata: CertificateMetaData,
    pub data: CertificateData,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct CertificateMetaData {
    pub owner: Principal,
    pub is_burned: bool,
    pub properties: Vec<(String, GenericValue)>,
    pub burned_at: Option<u64>,
    pub burned_by: Option<Principal>,
    pub minted_at: u64,
    pub minted_by: Principal,
}

impl CertificateMetaData {
    pub fn new(owner: Principal, properties: Option<Vec<(String, GenericValue)>>) -> Self {
        Self {
            owner,
            is_burned: false,
            properties: properties.unwrap_or_default(),
            burned_at: None,
            burned_by: None,
            minted_at: api::time(),
            minted_by: owner,
        }
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CertificateData {
    pub name: String,
    pub data: Data,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct HealthRecords {
    pub name: String,
    pub date_of_birth: String,
    pub allergies: Vec<String>,
    pub medications: Vec<String>,
    pub conditions: Vec<String>,
    pub vitals: Vec<VitalSigns>, // Stores vital sign measurements
    pub immunizations: Vec<Immunization>, // Tracks past immunizations
    pub surgeries: Vec<Surgery>, // Records past surgeries
    pub family_history: Vec<String>, // Lists family health conditions
    pub lifestyle_factors: LifestyleFactors, // Stores details on habits
}

// Sub-structs for specific data types
#[derive(CandidType, Deserialize, Clone)]
pub struct VitalSigns {
    pub date: String,
    pub blood_pressure: String, // Can be improved with specific values (systolic/diastolic)
    pub heart_rate: u32,
    pub temperature: f32,
    pub respiration_rate: u32,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Immunization {
    pub name: String,
    pub date: String,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Surgery {
    pub name: String,
    pub date: String,
    pub surgeon: String,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct LifestyleFactors {
    pub smoking: bool,
    pub alcohol_consumption: String, // Can be improved for specific details
    pub exercise_frequency: String,  // Can be improved for duration/intensity
}
#[derive(CandidType, Deserialize, Clone)]
pub enum Data {
    Link(String),
    Raw(Vec<u8>),
}

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct CanisterStats {
    pub total_supply: u64,
    pub total_transactions: u64,
    pub total_unique_holders: u64,
    pub cycles: u64,
}

// =================== Errors ============================

pub type CanisterResult<T> = Result<T, CanisterError>;

#[derive(CandidType, Serialize)]
pub enum CanisterError {
    AttemptedSelfTransfer,
    TokenNotFound,
    TransactionNotFound,
    NotAuthorizedAsCustodian,
    NotAuthorizedAsOwner,
    NotAuthorizedAsOperator,
    CertificateAlreadyExists,
    OwnerNotFound,
    Other(String),
}

// =================== Utilities =========================

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub enum GenericValue {
    BoolContent(bool),
    TextContent(String),
    BlobContent(Vec<u8>),
    Principal(Principal),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
    NatContent(Nat),
    Int8Content(i8),
    Int16Content(i16),
    Int32Content(i32),
    Int64Content(i64),
    IntContent(Int),
    FloatContent(f64),
    NestedContent(Vec<(String, GenericValue)>),
}
