require('@nomiclabs/hardhat-ethers');
require('@openzeppelin/hardhat-upgrades');
require('@nomiclabs/hardhat-etherscan');
require('@nomicfoundation/hardhat-chai-matchers');
require('hardhat-gas-reporter');

// Deployment secret management
require('dotenv').config({ path: './../.env' });

// Dev and prod config
let config;
if (process.env.NODE_ENV === 'development') {
  config = require('./../krondor.dev.json');
} else {
  config = require('./../krondor.json');
}

module.exports = {
  solidity: '0.8.19',
  networks: {
    default: {
      url: `${config.eth.rpc_url}/${process.env.RPC_API_KEY}`,
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
    },
  },
  etherscan: {
    apiKey: process.env.ETHERSCAN_API_KEY ?? '',
  },
};
