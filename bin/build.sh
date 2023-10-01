# echo "Building cli..."
# cd ../
# cargo build --release
echo "Building web..."
trunk build --release

echo "Fixing HTML..."
sed -i -e "s/\/krondor/.\/krondor/g" dist/index.html
sed -i -e "s/href=\".\/krondor/href=\"krondor/g" dist/index.html
rm dist/index.html-e

# move the contents of dist to .github/content
# remove all files in .github/content
rm public/*
mv dist/* public
