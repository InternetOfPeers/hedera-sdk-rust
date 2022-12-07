/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::file_service_client::FileServiceClient;
use hedera_proto::services::smart_contract_service_client::SmartContractServiceClient;
use tonic::transport::Channel;

use crate::entity_id::AutoValidateChecksum;
use crate::protobuf::ToProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    ContractId,
    Error,
    FileId,
    LedgerId,
    Transaction,
};

/// Undelete a file or smart contract that was deleted by a [`SystemDeleteTransaction`](crate::SystemDeleteTransaction).
pub type SystemUndeleteTransaction = Transaction<SystemUndeleteTransactionData>;

/// Undelete a file or smart contract that was deleted by  [`SystemDeleteTransaction`](crate::SystemDeleteTransaction).
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(default, rename_all = "camelCase"))]
pub struct SystemUndeleteTransactionData {
    pub file_id: Option<FileId>,
    pub contract_id: Option<ContractId>,
}

impl SystemUndeleteTransaction {
    /// Sets the contract ID to undelete.
    pub fn contract_id(&mut self, id: impl Into<ContractId>) -> &mut Self {
        self.body.data.file_id = None;
        self.body.data.contract_id = Some(id.into());
        self
    }

    /// Sets the file ID to undelete.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        self.body.data.contract_id = None;
        self.body.data.file_id = Some(id.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for SystemUndeleteTransactionData {
    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.contract_id.validate_checksum_for_ledger_id(ledger_id)?;
        self.file_id.validate_checksum_for_ledger_id(ledger_id)
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        if self.file_id.is_some() {
            FileServiceClient::new(channel).system_undelete(request).await
        } else {
            SmartContractServiceClient::new(channel).system_undelete(request).await
        }
    }
}

impl ToTransactionDataProtobuf for SystemUndeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let contract_id = self.contract_id.to_protobuf();
        let file_id = self.file_id.to_protobuf();

        let id = match (contract_id, file_id) {
            (Some(contract_id), _) => {
                Some(services::system_undelete_transaction_body::Id::ContractId(contract_id))
            }

            (_, Some(file_id)) => {
                Some(services::system_undelete_transaction_body::Id::FileId(file_id))
            }

            _ => None,
        };

        services::transaction_body::Data::SystemUndelete(services::SystemUndeleteTransactionBody {
            id,
        })
    }
}

impl From<SystemUndeleteTransactionData> for AnyTransactionData {
    fn from(transaction: SystemUndeleteTransactionData) -> Self {
        Self::SystemUndelete(transaction)
    }
}
