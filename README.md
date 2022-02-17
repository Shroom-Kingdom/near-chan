# NEARchan NFT Contract

## Setup

```bash
# setup environment
#export NEAR_ENV=mainnet
export NEAR_ENV=testnet
#export NFT_CONTRACT_ID="near-chan.shrm.near"
export NFT_CONTRACT_ID="near-chan-v5.shrm.testnet"
#export DAO_ID="shrm.sputnik-dao.near"
export DAO_ID="shrm.sputnik-dao.testnet"

# create account
near create-account $NFT_CONTRACT_ID --masterAccount shrm.testnet --initialBalance 10

# build and deploy
./build.sh
near deploy --wasmFile out/main.wasm --accountId $NFT_CONTRACT_ID

# initialize contract
near call $NFT_CONTRACT_ID new_default_meta '{"owner_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID
```

## Mint

```bash
# only works in bash
copies=2
timestamp=$(date +%s%N)
declare -A variant0=(
  [token_id]='smw-small'
  [title]='SMW-SMALL'
  [media]='QmYbUAX7DnLKSZwQz6Bpmwt1MozbcJTyVrHd38LedFn9r9'
  [reference]='QmbxWjVF2J2sMJsPt3WGWxBCc1SDQUX1Ujo1eqpjghgN8J'
)
declare -A variant1=(
  [token_id]='smw-big'
  [title]='SMW-BIG'
  [media]='QmaE1fYhyGEL9QzVZUX1A3nYdG12pfEiCkyNuFaEQZ93NJ'
  [reference]='QmRcjJksGw5ucrDEj5MTPqYnMd3CR35v5owdZpvvrKduvq'
)
declare -A variant2=(
  [token_id]='smb3-small'
  [title]='SMB3-SMALL'
  [media]='QmVvJeiUhRQgeS1RpCPRLAEBca2QnHckbEr3MdT79BvZKx'
  [reference]='QmU9ndy5JaefP4AaJwE7v1E6tRk6dkJhnNM5h1m6suhzyD'
)
declare -A variant3=(
  [token_id]='smb3-big'
  [title]='SMB3-BIG'
  [media]='QmVn2qad7CPGVejLDHnXZtLKs53HJRCTBzEzQ7Qj78ihbL'
  [reference]='QmRH9nGnAzLWJCCEPUmdJJJZAASMwxZLhSeyvMoHUHkcyY'
)
declare -A variant4=(
  [token_id]='smb1-small'
  [title]='SMB1-SMALL'
  [media]='QmUHgybFgr6LC5b1RHbahqKZaZLxk9U5P55WHbXyaDHD53'
  [reference]='QmZw7Hdz2gvvg9VNG3yX7Xvt9tJzv7iCUmt7ohAJLo1wK9'
)
declare -A variant5=(
  [token_id]='smb1-big'
  [title]='SMB1-BIG'
  [media]='QmU4YEhw8zz7tTofeWj3RDnojz3SriqYCpYV5cqCPaZ42t'
  [reference]='QmeDdDBGPRzCLLavKJm19neeqkKWcftMMn4cQ1WhJprn38'
)
declare -n variant

for variant in ${!variant@}; do
  for ((i=1; i<=$copies; i++)); do
    near call $NFT_CONTRACT_ID nft_mint '{
      "token_id": "'${variant[token_id]}':'$i'",
      "metadata": {
        "title": "Shroom Kingdom NEARchan #'${variant[title]}' #'$i'",
        "description": "https://near-chan.github.io/",
        "media": "'${variant[media]}'",
        "copies": '$copies',
        "issued_at": '$timestamp',
        "reference": "'${variant[reference]}'"
      },
      "receiver_id": "'$NFT_CONTRACT_ID'",
      "perpetual_royalties": {
        "'$DAO_ID'": 1000
      }
    }' --accountId $NFT_CONTRACT_ID --amount 0.1
  done
done
```
