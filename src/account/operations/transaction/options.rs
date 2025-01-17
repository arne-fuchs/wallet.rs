// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::block::{output::OutputId, payload::tagged_data::TaggedDataPayload};
use serde::{Deserialize, Serialize};

use crate::account::types::address::AccountAddress;

/// Options for transactions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionOptions {
    #[serde(rename = "remainderValueStrategy", default)]
    pub remainder_value_strategy: RemainderValueStrategy,
    #[serde(rename = "taggedDataPayload", default)]
    pub tagged_data_payload: Option<TaggedDataPayload>,
    // If custom inputs are provided only they are used. If also other additional inputs should be used,
    // `mandatory_inputs` should be used instead.
    #[serde(rename = "customInputs", default)]
    pub custom_inputs: Option<Vec<OutputId>>,
    #[serde(rename = "mandatoryInputs", default)]
    pub mandatory_inputs: Option<Vec<OutputId>>,
    #[serde(rename = "allowBurning", default)]
    pub allow_burning: bool,
    pub note: Option<String>,
}

#[allow(clippy::enum_variant_names)]
/// The strategy to use for the remainder value management when sending funds.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "strategy", content = "value")]
pub enum RemainderValueStrategy {
    /// Keep the remainder value on the source address.
    ReuseAddress,
    /// Move the remainder value to a change address.
    ChangeAddress,
    /// Move the remainder value to any specified address.
    CustomAddress(AccountAddress),
}

impl Default for RemainderValueStrategy {
    fn default() -> Self {
        Self::ReuseAddress
    }
}
