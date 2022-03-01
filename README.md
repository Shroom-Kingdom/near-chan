# NEARchan NFT Contract

## Setup

```bash
# setup environment
#export NEAR_ENV=mainnet
export NEAR_ENV=testnet
#export NFT_CONTRACT_ID="near-chan.shrm.near"
export NFT_CONTRACT_ID="near-chan-v7.shrm.testnet"
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
  [media]='https://5xtk5klcotgwrcmkrgqpiy3g5rsji6ov3o7kxrhdjwo2czbhuy.arweave.net/7eau_qWJ0zWiJiomg9GNm7GSUedXbvqvE402doWQnps'
  [reference]='https://ymydrtmvtfodvriikuivcehys26r4ylpebp5ozuie6s3i2yyau.arweave.net/wzA4zZW-ZXDrFCFURURD4lr0eYW8gX9dmiCeltGsYBU'
)
declare -A variant1=(
  [token_id]='smw-big'
  [title]='SMW-BIG'
  [media]='https://sg5qh3h5obec2csk3g6gknzsqzwr5zftmopoaxgi6i5z5z7dhkva.arweave.net/kbsD7P1wSC0KStm8ZTcyhm0e5LNjnuBcyPI7nufjOqo'
  [reference]='https://mric6kqtqbk75rq4uplfccvppojfhymjjj2a3ulpxxnuqfwz64.arweave.net/ZFAvKhOAVf7GHKPWUQqve5JT4YlKdA3Rb73_bSBbZ90'
)
declare -A variant2=(
  [token_id]='smb3-small'
  [title]='SMB3-SMALL'
  [media]='https://g5f3yn32srbdi5ezra64yg65piagmfkhl5rdskfouks4u47yim.arweave.net/N0u8N3qUQjR0mYg9zBvdegBmFUdfYjkor_qKlynP4Q8'
  [reference]='https://lo6b623yhfe2uugklpcjgzueche6esiudbqh5224rzvb6fq44yya.arweave.net/W7wfa3g5SapQylvEk2aEEcniSRQYYH7rXI5qHxYc5jA'
)
declare -A variant3=(
  [token_id]='smb3-big'
  [title]='SMB3-BIG'
  [media]='https://vpjxsqdgaadjwnf2anwhpvnm4ud7a7yfqtk72seuftvvknopippa.arweave.net/q9N5QGYABps0ugNsd9Ws5QfwfwWE1f1IlCzrVTXPQ94'
  [reference]='https://ccapirlanhy3wm65ki54b6vg4uftt5rrt3kdqcuzgpmm5to4.arweave.net/EID0RWBp8bsz3VI7wP_qm5Qs_59jGe1DgKmT-PYzs3c'
)
declare -A variant4=(
  [token_id]='smb1-small'
  [title]='SMB1-SMALL'
  [media]='https://t55yljvd3wtdffoc23hormpoineoa4lspiqpftqnymfxtwjxsf4a.arweave.net/n3uFpqPdpjKVwtbO6LHuQ0jgcXJ6IPLODcMLedk3kXg'
  [reference]='https://p4zeatryvgh5mik7oauwycplube4ghzer4kxh4lg6zh4z2q5fkbq.arweave.net/fzJATjipj9YhX3ApbAnroEnDHySPFXPxZvZPzOodKoM'
)
declare -A variant5=(
  [token_id]='smb1-big'
  [title]='SMB1-BIG'
  [media]='https://dhiq6lwgacfttn4ati6sr7gxhoyhd2fwofodrq37gmrrkfylv3vq.arweave.net/GdEPLsYAizm3gJo9KPzXO7Bx6LZxXDjDfzMjFRcLrus'
  [reference]='https://umwziqqkrzc6p6a3czx4unts4bnjo2iybciwmy33olew3q6p4m.arweave.net/oy2UQgqORef4GxZvyjZy4F_qXaRgIkWZje3LJbcPP44'
)
declare -n variant

for variant in ${!variant@}; do
  near call $NFT_CONTRACT_ID nft_create_series '{
    "token_series_id": "'${variant[token_id]}'",
    "metadata": {
      "title": "Shroom Kingdom NEARchan '${variant[title]}'",
      "description": "https://near-chan.github.io/",
      "media": "'${variant[media]}'",
      "copies": '$copies',
      "issued_at": '$timestamp',
      "reference": "'${variant[reference]}'"
    }
  }' --accountId $NFT_CONTRACT_ID --amount 0.1

  for ((i=1; i<=$copies; i++)); do
    near call $NFT_CONTRACT_ID nft_mint '{
      "edition_id": "'$i'",
      "token_series_id": "'${variant[token_id]}'",
      "receiver_id": "'$NFT_CONTRACT_ID'",
      "perpetual_royalties": {
        "'$DAO_ID'": 1000
      }
    }' --accountId $NFT_CONTRACT_ID --amount 0.1
  done
done
```
