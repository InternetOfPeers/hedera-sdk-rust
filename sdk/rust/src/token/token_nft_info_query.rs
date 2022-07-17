use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    Query,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    NftId,
    ToProtobuf,
    TokenNftInfo,
};

/// Gets info on an NFT for a given TokenID and serial number.
pub type TokenNftInfoQuery = Query<TokenNftInfoQueryData>;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenNftInfoQueryData {
    /// The ID of the NFT
    nft_id: Option<NftId>,
}

impl From<TokenNftInfoQueryData> for AnyQueryData {
    #[inline]
    fn from(data: TokenNftInfoQueryData) -> Self {
        Self::TokenNftInfo(data)
    }
}

impl TokenNftInfoQuery {
    /// Sets the file ID for which contents are requested.
    pub fn nft_id(&mut self, nft_id: impl Into<NftId>) -> &mut Self {
        self.data.nft_id = Some(nft_id.into());
        self
    }
}

impl ToQueryProtobuf for TokenNftInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let nft_id = self.nft_id.as_ref().map(NftId::to_protobuf);

        services::Query {
            query: Some(services::query::Query::TokenGetNftInfo(services::TokenGetNftInfoQuery {
                header: Some(header),
                nft_id,
            })),
        }
    }
}

#[async_trait]
impl QueryExecute for TokenNftInfoQueryData {
    type Response = TokenNftInfo;

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        TokenServiceClient::new(channel).get_token_nft_info(request).await
    }
}