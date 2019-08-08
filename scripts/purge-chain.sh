#!/bin/bash
db=$1

if [[ "$OSTYPE" == "linux-gnu" ]]; then
  echo "Clearing local data from home dir: $HOME/.local/share/stafi"
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/.local/share/stafi/chains/staging_testnet/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/.local/share/stafi/chains/dev/
		rm -rf ~/.local/share/stafi/chains/development/
	elif [[ "$db" == "stafi" ]]; then
    	rm -rf ~/.local/share/stafi/chains/stafi/
    	rm -rf ~/.local/share/stafi/chains/stafi_testnet/
	else
		db="all"
	    rm -rf ~/.local/share/stafi/chains/dev/
	    rm -rf ~/.local/share/stafi/chains/development/
	    rm -rf ~/.local/share/stafi/chains/stafi/
	    rm -rf ~/.local/share/stafi/chains/stafi_testnet/
	    rm -rf ~/.local/share/stafi/chains/staging_testnet/
    	rm -rf ~/.local/share/stafi/chains/local_testnet/
	fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
  echo "Clearing local data from home dir: $HOME/Library/Application Support/stafi"
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/Library/Application\ Support/stafi/chains/staging_testnet/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/Library/Application\ Support/stafi/chains/dev/
		rm -rf ~/Library/Application\ Support/stafi/chains/development/
	elif [[ "$db" == "stafi" ]]; then
		rm -rf ~/Library/Application\ Support/stafi/chains/stafi/
		rm -rf ~/Library/Application\ Support/stafi/chains/stafi_testnet/
	else
		db="all"
		rm -rf ~/Library/Application\ Support/stafi/chains/dev/
		rm -rf ~/Library/Application\ Support/stafi/chains/development/
	    rm -rf ~/Library/Application\ Support/stafi/chains/stafi/
	    rm -rf ~/Library/Application\ Support/stafi/chains/stafi_testnet/
	    rm -rf ~/Library/Application\ Support/stafi/chains/staging_testnet/
		rm -rf ~/Library/Application\ Support/stafi/chains/local_testnet/
	fi
else
  echo "Clearing local data from home dir: $HOME/.local/share/stafi"
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/.local/share/stafi/chains/staging_testnet/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/.local/share/stafi/chains/dev/
		rm -rf ~/.local/share/stafi/chains/development/
	elif [[ "$db" == "stafi" ]]; then
    	rm -rf ~/.local/share/stafi/chains/stafi/
    	rm -rf ~/.local/share/stafi/chains/stafi_testnet/
	else
		db="all"
	    rm -rf ~/.local/share/stafi/chains/dev/
	    rm -rf ~/.local/share/stafi/chains/development/
	    rm -rf ~/.local/share/stafi/chains/stafi/
	    rm -rf ~/.local/share/stafi/chains/stafi_testnet/
	    rm -rf ~/.local/share/stafi/chains/staging_testnet/
    	rm -rf ~/.local/share/stafi/chains/local_testnet/
	fi
fi

echo "Deleted $db databases"
