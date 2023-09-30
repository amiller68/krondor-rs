echo "Building contracts..."
cd contracts
yarn install
yarn build

echo "Building cli..."
cd ../
cargo build --release
echo "Building web..."
trunk build --release

echo "Fixing HTML..."
sed -i -e "s/\/krondor/.\/krondor/g" dist/index.html
sed -i -e "s/href=\".\/krondor/href=\"krondor/g" dist/index.html
