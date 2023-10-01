echo "Starting ipfs daemon..."
tmux new-session -d -s ipfs 'ipfs daemon'

echo "Building static site..."
yarn build

echo "Pushing to IPFS..."
ipfs add public/* -r -w -q | tail -n 1 > .ipfs_hash
IPFS_HASH=$(cat .ipfs_hash)
echo "Inspect at http://localhost:8080/ipfs/$IPFS_HASH"

# Function to be executed when SIGINT (Ctrl+C) is caught
on_exit() {
    echo "Closing tmux session..."
    tmux kill-session -t ipfs
    exit
}

while true; do
    sleep 1
done