use crate::*;

pub type TokenSeriesId = String;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenSeries {
    pub metadata: TokenMetadata,
    creator_id: AccountId,
    tokens: UnorderedSet<TokenId>,
    pub royalty: HashMap<AccountId, u32>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenSeriesJson {
    token_series_id: TokenSeriesId,
    metadata: TokenMetadata,
    creator_id: AccountId,
    royalty: HashMap<AccountId, u32>,
}

/// between token_series_id and edition number e.g. 42:2 where 42 is series and 2 is edition
pub const TOKEN_DELIMITER: char = ':';
/// TokenMetadata.title returned for individual token e.g. "Title — 2/10" where 10 is max copies
pub const TITLE_DELIMITER: &str = " #";
/// e.g. "Title — 2/10" where 10 is max copies
pub const EDITION_DELIMITER: &str = "/";

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_create_series(
        &mut self,
        token_series_id: TokenSeriesId,
        metadata: TokenMetadata,
        royalty: HashMap<AccountId, u32>,
    ) {
        assert!(
            !self.series_by_id.contains_key(&token_series_id),
            "Series already exists"
        );
        self.token_metadata_by_series_id
            .insert(&token_series_id, &metadata);
        let token_series = TokenSeries {
            metadata,
            creator_id: env::signer_account_id(),
            tokens: UnorderedSet::new(
                StorageKey::TokensBySeriesInner {
                    token_series: token_series_id.clone(),
                }
                .try_to_vec()
                .unwrap(),
            ),
            royalty,
        };
        self.series_by_id.insert(&token_series_id, &token_series);
    }

    pub fn nft_get_series_single(&self, token_series_id: TokenSeriesId) -> TokenSeriesJson {
        let token_series = self
            .series_by_id
            .get(&token_series_id)
            .expect("Series does not exist");
        let metadata = self
            .token_metadata_by_series_id
            .get(&token_series_id)
            .unwrap();
        TokenSeriesJson {
            token_series_id,
            metadata,
            creator_id: token_series.creator_id,
            royalty: token_series.royalty,
        }
    }

    pub fn nft_get_series_format(self) -> (char, &'static str, &'static str) {
        (TOKEN_DELIMITER, TITLE_DELIMITER, EDITION_DELIMITER)
    }
}
