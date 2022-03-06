use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        edition_id: TokenId,
        token_series_id: TokenSeriesId,
        receiver_id: AccountId,
    ) {
        let initial_storage_usage = env::storage_usage();

        let token_series = self.series_by_id.get(&token_series_id).unwrap();

        let royalty = token_series.royalty.clone();

        //specify the token struct that contains the owner ID
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            creator_id: env::signer_account_id(),
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty,
        };
        let mut token_id = token_series_id;
        token_id.push(TOKEN_DELIMITER);
        token_id.push_str(&edition_id);

        assert!(
            !self.tokens_by_id.contains_key(&token_id),
            "Token already exists"
        );
        self.tokens_by_id.insert(&token_id, &token);

        let mut metadata = token_series.metadata;
        metadata.title = metadata.title.map(|mut title| {
            title.push_str(TITLE_DELIMITER);
            title.push_str(&edition_id);
            if let Some(copies) = metadata.copies {
                title.push_str(EDITION_DELIMITER);
                title.push_str(&copies.to_string());
            }
            title
        });

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &metadata);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }
}
