#!/bin/bash

cache_dir=.cache
contract_address_file=$cache_dir/contract_address

# Read contract address from cache
contract_address=$(cat $contract_address_file)

cargo contract call --contract $contract_address --suri //Alice --message set_manual_kill --args true --execute --skip-confirm