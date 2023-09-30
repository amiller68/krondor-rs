import Web3 from 'web3';

const ABI =   [
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "address",
          "name": "previousOwner",
          "type": "address"
        },
        {
          "indexed": true,
          "internalType": "address",
          "name": "newOwner",
          "type": "address"
        }
      ],
      "name": "OwnershipTransferred",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "string",
          "name": "cid",
          "type": "string"
        }
      ],
      "name": "updatedCID",
      "type": "event"
    },
    {
      "inputs": [],
      "name": "cid",
      "outputs": [
        {
          "internalType": "string",
          "name": "",
          "type": "string"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "getCID",
      "outputs": [
        {
          "internalType": "string",
          "name": "",
          "type": "string"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "owner",
      "outputs": [
        {
          "internalType": "address",
          "name": "",
          "type": "address"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "renounceOwnership",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "newOwner",
          "type": "address"
        }
      ],
      "name": "transferOwnership",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "string",
          "name": "_cid",
          "type": "string"
        }
      ],
      "name": "updateCID",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    }
  ]

/*
 * A class for interacting with the Ethereum blockchain.
 */
//@ ts-ignore
export default class RootCid {
  contract;
  web3;
  chain_id;
  signer;
  /*
   * @param contract_address - The address of the contract.
   * @param rpc_url - The URL of the RPC server.
   * @param chain_id - The chain ID of the network.
   */
  constructor(rpc_url, contract_address, chain_id) {
    this.web3 = new Web3(new Web3.providers.HttpProvider(rpc_url));
    this.contract = new this.web3.eth.Contract(ABI, contract_address);
    this.chain_id = chain_id;
  }
  
  async get() {
    return await this.contract.methods.getCID().call();
  }
}
