# Start `ipfs daemon` in a new terminal window
echo "Starting ipfs daemon..."
tmux new-session -d -s ipfs 'ipfs daemon'
# Start `npx hard`
echo "Starting hardhat node..."
tmux new-session -d -s hardhat 'cd contracts && npx hardhat node'

# Function to be executed when SIGINT (Ctrl+C) is caught
on_exit() {
    echo "Closing tmux session..."
    tmux kill-session -t ipfs
    tmux kill-session -t hardhat
    exit
}

# Trap the SIGINT signal to execute the on_exit function when Ctrl+C is pressed
trap on_exit SIGINT

# Deploy contracts
echo "Deploying contract..."
cd contracts
yarn deploy:local

echo "Running!"

while true; do
    sleep 1
done