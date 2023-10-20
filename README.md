# Krondor-Rs
The stupidly simple blog example written in Rust and version controlled by whatever git provider you have.
Can be deployed to IPFS as a static site.

# Requirements
- Rust
- Cargo
- Trunk
- Yarn

# Build
```bash
# CLI App
cargo build
# Web App
yarn prepare && yarn build
```

# Run
```bash
# CLI App
# Initialize a new blog space at ./github/public
cargo run -- init
# Add a new post to the blog space from ./github/public/posts/
# This can be a markdown file
cargo run -- add --name <name> --title <title> --description <description>
# Web App
yarn start
```