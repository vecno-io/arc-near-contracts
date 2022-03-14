use crate::*;

pub const NFT_METADATA_SPEC: &str = "nft-1.0.0";
pub const ARC_METADATA_SPEC: &str = "arc-1.0.0";

/// Metadata for the contract itself.
#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub base_uri: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
}

#[derive(Clone, Default, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub base_uri: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
}

pub trait ArcMetadata {
    fn arc_metadata(&self) -> JsonMetadata;
}

pub trait NftMetadata {
    fn nft_metadata(&self) -> JsonMetadata;
}

impl Metadata {
    pub fn assert_valid(&self) {
        require!(
            self.reference.is_some() == self.reference_hash.is_some(),
            "Reference and reference hash must be present"
        );
        if let Some(reference_hash) = &self.reference_hash {
            require!(reference_hash.0.len() == 32, "Hash has to be 32 bytes");
        }
    }
}
impl JsonMetadata {
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

#[macro_export]
macro_rules! impl_meta {
    //where $data is LazyOption<Metadata>
    ($contract: ident, $data: ident) => {
        use $crate::*;

        impl ArcMetadata for $contract {
            fn arc_metadata(&self) -> JsonMetadata {
                if let Some(data) = self.$data.get() {
                    return JsonMetadata {
                        spec: ARC_METADATA_SPEC.to_string(),
                        name: data.name,
                        symbol: data.symbol,
                        icon: data.icon,
                        base_uri: data.base_uri,
                        reference: data.reference,
                        reference_hash: data.reference_hash,
                    };
                }
                Default::default()
            }
        }

        impl NftMetadata for $contract {
            fn nft_metadata(&self) -> JsonMetadata {
                if let Some(data) = self.$data.get() {
                    return JsonMetadata {
                        spec: NFT_METADATA_SPEC.to_string(),
                        name: data.name,
                        symbol: data.symbol,
                        icon: data.icon,
                        base_uri: data.base_uri,
                        reference: data.reference,
                        reference_hash: data.reference_hash,
                    };
                }
                Default::default()
            }
        }
    };
}
