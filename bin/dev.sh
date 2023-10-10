echo "Starting ipfs daemon..."
tmux new-session -d -s ipfs 'ipfs daemon'

echo "Building static site..."
export APP_DEV=true && yarn build

source .env.config
IPFS_GATEWAY=$DEV_APP_IPFS_GATEWAY

echo "Pushing to IPFS..."
ipfs add public/* -r -w -q | tail -n 1 >.ipfs_hash
IPFS_HASH=$(cat .ipfs_hash)
echo "Inspect at $IPFS_GATEWAY/$IPFS_HASH"

# Function to be executed when SIGINT (Ctrl+C) is caught
on_exit() {
	echo "Closing tmux session..."
	tmux kill-session -t ipfs
	exit
}

while true; do
	sleep 1
done
