use crate::*;

pub const NFT_METADATA_SPEC: &str = "nft-1.0.0";

/// Metadata for the contract itself.
#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractData {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub base_uri: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
}

/// Metadata on the individual token level.
#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenData {
    pub copies: Option<u64>,
    pub issued_at: Option<u64>,
    pub expires_at: Option<u64>,
    pub starts_at: Option<u64>,
    pub updated_at: Option<u64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub extra: Option<String>,
    pub media: Option<String>,
    pub media_hash: Option<Base64VecU8>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
}

/// Offers details on the contract-level metadata.
pub trait NftMetadata {
    fn nft_metadata(&self) -> ContractData;
}

impl ContractData {
    pub fn assert_valid(&self) {
        require!(self.spec == NFT_METADATA_SPEC, "Spec is not NFT metadata");
        require!(
            self.reference.is_some() == self.reference_hash.is_some(),
            "Reference and reference hash must be present"
        );
        if let Some(reference_hash) = &self.reference_hash {
            require!(reference_hash.0.len() == 32, "Hash has to be 32 bytes");
        }
    }
}

impl TokenData {
    pub fn assert_valid(&self) {
        require!(self.media.is_some() == self.media_hash.is_some());
        if let Some(media_hash) = &self.media_hash {
            require!(media_hash.0.len() == 32, "Media hash has to be 32 bytes");
        }

        require!(self.reference.is_some() == self.reference_hash.is_some());
        if let Some(reference_hash) = &self.reference_hash {
            require!(
                reference_hash.0.len() == 32,
                "Reference hash has to be 32 bytes"
            );
        }
    }
}
