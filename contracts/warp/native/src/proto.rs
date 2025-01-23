use cosmwasm_std::{Binary, CosmosMsg};
use prost::Message;

#[derive(serde::Serialize, serde::Deserialize, ::prost::Message)]
pub struct MsgCreateDenom {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// subdenom can be up to 44 "alphanumeric" characters long.
    #[prost(string, tag = "2")]
    pub subdenom: ::prost::alloc::string::String,
}

impl From<MsgCreateDenom> for CosmosMsg {
    fn from(v: MsgCreateDenom) -> Self {
        CosmosMsg::Stargate {
            type_url: MsgCreateDenom::type_url(),
            value: Binary(v.encode_to_vec()),
        }
    }
}

impl MsgCreateDenom {
    fn type_url() -> String {
        #[cfg(feature = "osmosis")]
        {
            "/osmosis.tokenfactory.v1beta1.MsgCreateDenom".to_string()
        }
        #[cfg(feature = "injective")]
        {
            "/injective.tokenfactory.v1beta1.MsgCreateDenom".to_string()
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, ::prost::Message)]
pub struct MsgCreateDenomResponse {
    #[prost(string, tag = "1")]
    pub new_token_denom: ::prost::alloc::string::String,
}

impl TryFrom<cosmwasm_std::Binary> for MsgCreateDenomResponse {
    type Error = cosmwasm_std::StdError;

    fn try_from(binary: cosmwasm_std::Binary) -> Result<Self, Self::Error> {
        Self::decode(&binary[..]).map_err(|e| cosmwasm_std::StdError::ParseErr {
            target_type: stringify!(#ident).to_string(),
            msg: format!(
                "Unable to decode binary: \n  - base64: {}\n  - bytes array: {:?}\n\n{:?}",
                binary,
                binary.to_vec(),
                e
            ),
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, ::prost::Message)]
pub struct Coin {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}

#[derive(serde::Serialize, serde::Deserialize, ::prost::Message)]
pub struct MsgMint {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<Coin>,
}

impl From<MsgMint> for CosmosMsg {
    fn from(v: MsgMint) -> Self {
        CosmosMsg::Stargate {
            type_url: MsgMint::type_url(),
            value: Binary(v.encode_to_vec()),
        }
    }
}

impl MsgMint {
    fn type_url() -> String {
        #[cfg(feature = "osmosis")]
        {
            "/osmosis.tokenfactory.v1beta1.MsgMint".to_string()
        }
        #[cfg(feature = "injective")]
        {
            "/injective.tokenfactory.v1beta1.MsgMint".to_string()
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, ::prost::Message)]
pub struct MsgBurn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<Coin>,
}

impl From<MsgBurn> for CosmosMsg {
    fn from(v: MsgBurn) -> Self {
        CosmosMsg::Stargate {
            type_url: MsgBurn::type_url(),
            value: Binary(v.encode_to_vec()),
        }
    }
}

impl MsgBurn {
    fn type_url() -> String {
        #[cfg(feature = "osmosis")]
        {
            "/osmosis.tokenfactory.v1beta1.MsgBurn".to_string()
        }
        #[cfg(feature = "injective")]
        {
            "/injective.tokenfactory.v1beta1.MsgBurn".to_string()
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, ::prost::Message)]
pub struct DenomUnit {
    /// denom represents the string name of the given denom unit (e.g uatom).
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// exponent represents power of 10 exponent that one must
    /// raise the base_denom to in order to equal the given DenomUnit's denom
    /// 1 denom = 1^exponent base_denom
    /// (e.g. with a base_denom of uatom, one can create a DenomUnit of 'atom' with
    /// exponent = 6, thus: 1 atom = 10^6 uatom).
    #[prost(uint32, tag = "2")]
    #[serde(
        serialize_with = "as_str::serialize",
        deserialize_with = "as_str::deserialize"
    )]
    pub exponent: u32,
    /// aliases is a list of string aliases for the given denom
    #[prost(string, repeated, tag = "3")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, ::prost::Message)]
pub struct Metadata {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// denom_units represents the list of DenomUnit's for a given coin
    #[prost(message, repeated, tag = "2")]
    pub denom_units: ::prost::alloc::vec::Vec<DenomUnit>,
    /// base represents the base denom (should be the DenomUnit with exponent = 0).
    #[prost(string, tag = "3")]
    pub base: ::prost::alloc::string::String,
    /// display indicates the suggested denom that should be
    /// displayed in clients.
    #[prost(string, tag = "4")]
    pub display: ::prost::alloc::string::String,
    /// name defines the name of the token (eg: Cosmos Atom)
    ///
    /// Since: cosmos-sdk 0.43
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// symbol is the token symbol usually shown on exchanges (eg: ATOM). This can
    /// be the same as the display.
    ///
    /// Since: cosmos-sdk 0.43
    #[prost(string, tag = "6")]
    pub symbol: ::prost::alloc::string::String,
}

#[derive(serde::Serialize, serde::Deserialize, ::prost::Message)]
pub struct MsgSetDenomMetadata {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<Metadata>,
}

impl From<MsgSetDenomMetadata> for CosmosMsg {
    fn from(v: MsgSetDenomMetadata) -> Self {
        CosmosMsg::Stargate {
            type_url: MsgSetDenomMetadata::type_url(),
            value: Binary(v.encode_to_vec()),
        }
    }
}

impl MsgSetDenomMetadata {
    fn type_url() -> String {
        #[cfg(feature = "osmosis")]
        {
            "/osmosis.tokenfactory.v1beta1.MsgSetDenomMetadata".to_string()
        }
        #[cfg(feature = "injective")]
        {
            "/injective.tokenfactory.v1beta1.MsgSetDenomMetadata".to_string()
        }
    }
}

mod as_str {
    use serde::{de, Deserialize, Deserializer, Serializer};
    use std::{fmt::Display, str::FromStr};

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        T::from_str(&s).map_err(de::Error::custom)
    }

    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Display,
    {
        serializer.serialize_str(&value.to_string())
    }
}
