if [[ -z $(subkey) ]]; then
	cargo install --force --git https://github.com/paritytech/substrate subkey
fi

new_mnemonic=$(subkey generate | grep -o '`.*`' | tr -d '`')
mnemonic=${1:-$new_mnemonic}

main_seed=$(subkey inspect "${mnemonic}" | grep -o ': .*' | sed '1!d' | tr -d ': ')
main_pubkey=$(subkey inspect "${mnemonic}" | grep -o ': .*' | sed '2!d' | tr -d ': ')
main_address=$(subkey inspect "${mnemonic}" | grep -o ': .*' | sed '3!d' | tr -d ': ')

stash_pubkey=$(subkey inspect "${mnemonic}"//stash | grep -o ': .*' | sed '1!d' | tr -d ': ')
stash_address=$(subkey inspect "${mnemonic}"//stash | grep -o ': .*' | sed '2!d' | tr -d ': ')

controller_pubkey=$(subkey inspect "${mnemonic}"//controller | grep -o ': .*' | sed '1!d' | tr -d ': ')
controller_address=$(subkey inspect "${mnemonic}"//controller | grep -o ': .*' | sed '2!d' | tr -d ': ')

grandpa_pubkey=$(subkey -e inspect "${mnemonic}"//grandpa | grep -o ': .*' | sed '1!d' | tr -d ': ')
grandpa_address=$(subkey -e inspect "${mnemonic}"//grandpa | grep -o ': .*' | sed '2!d' | tr -d ': ')

babe_pubkey=$(subkey -e inspect "${mnemonic}"//babe | grep -o ': .*' | sed '1!d' | tr -d ': ')
babe_address=$(subkey -e inspect "${mnemonic}"//babe | grep -o ': .*' | sed '2!d' | tr -d ': ')

imonline_pubkey=$(subkey -e inspect "${mnemonic}"//imonline | grep -o ': .*' | sed '1!d' | tr -d ': ')
imonline_address=$(subkey -e inspect "${mnemonic}"//imonline | grep -o ': .*' | sed '2!d' | tr -d ': ')

echo ""
echo "*********** SAVE THIS MNEMONIC FOR FUTURE USE OR RISK LOSING ACCESS TO ANY FUNDS ***********"
echo ""
echo "Mnemonic: ${mnemonic}"
echo "Seed: ${main_seed}"
echo "Pubkey: ${main_pubkey}"
echo "Address: ${main_address}"
echo ""
echo "********************************************************************************************"
echo ""
echo "*********** SR25519 STASH ACCOUNT FOR STORING FUNDS TO DELEGATE TO VALIDATORS OR GENERAL USE ***********"
echo ""
echo "Stash pubkey: ${stash_pubkey}"
echo "Stash address: ${stash_address}"
echo ""
echo "*********** SR25519 CONTROLLER ACCOUNT FOR CONTROLLING A VALIDATOR NODE OR GENERAL USE ***********"
echo ""
echo "Controller pubkey: ${controller_pubkey}"
echo "Controller address: ${controller_address}"
echo ""
echo "*********** ED25519 AUTHORITY ACCOUNT FOR CONTROLLING A GRANDPA NODE OR GENERAL USE ***********"
echo ""
echo "GRANDPA pubkey: ${grandpa_pubkey}"
echo "GRANDPA address: ${grandpa_address}"
echo ""
echo "*********** ED25519 AUTHORITY ACCOUNT FOR CONTROLLING A BABE NODE OR GENERAL USE ***********"
echo ""
echo "Babe pubkey: ${babe_pubkey}"
echo "Babe address: ${babe_address}"
echo ""
echo "*********** ED25519 AUTHORITY ACCOUNT FOR CONTROLLING AN IMONLINE NODE OR GENERAL USE ***********"
echo ""
echo "Imonline pubkey: ${imonline_pubkey}"
echo "Imonline address: ${imonline_address}"
echo ""
echo ""

echo " For development purposes, disregard "
echo " // "${stash_address}""
echo " hex![\"${stash_pubkey}\"].unchecked_into(), "
echo " // "${controller_address}""
echo " hex![\"${controller_pubkey}\"].unchecked_into(), "
echo " // "${grandpa_address}""
echo " hex![\"${grandpa_pubkey}\"].unchecked_into(), "
echo " // "${babe_address}""
echo " hex![\"${babe_pubkey}\"].unchecked_into(), "
echo " // "${imonline_address}""
echo " hex![\"${imonline_pubkey}\"].unchecked_into(), "
