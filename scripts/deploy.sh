!/bin/bash

cache_dir=.cache
mkdir -p $cache_dir

grid_file=$1
echo "Building contract..."
cargo contract build

echo "Uploading contract..."
cargo contract upload --suri //Alice --execute

echo "Instantiating contract..."
grid=$(cat $grid_file)
contract_address=$(cargo contract instantiate --suri //Alice --args "$grid" --execute --output-json --skip-confirm | jq .contract)
echo "Contract address: $contract_address"
# Write contract address to cache
echo $contract_address > $cache_dir/contract_address