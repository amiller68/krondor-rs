use ethers::{
    prelude::*,
    providers::{Http, Provider},
    types::{Address, TransactionRequest},
    contract::Contract, abi::Abi, signers::LocalWallet
};
use anyhow::Result;
use std::sync::Arc;

const ABI_STRING: &str = r#"
  [
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
  "#;

#[derive(Clone, Debug)]
pub struct RootCid {
  pub(crate) contract: Contract<ethers::providers::Provider<Http>>,
  signer: Option<SignerMiddleware<Provider<Http>, LocalWallet>>,
  chain_id: u64,
}


impl RootCid {
    pub fn new(
        url: String,
        address: String,
        chain_id: u64,
        key: Option<String>,
    ) -> Result<Self> {
        let client = Provider::<Http>::try_from(url)?;
        let address: Address = address.parse()?;
        let abi: Abi = serde_json::from_str(ABI_STRING)?;
        let contract = Contract::new(address, abi, Arc::new(client.clone()));
        let signer = match key {
            Some(key) => {
                let wallet = key.parse::<LocalWallet>()?.with_chain_id(chain_id);
                let signer = SignerMiddleware::new(client, wallet);
                Some(signer)
            }
            None => None,
        };
        Ok(Self {
            contract,
            signer,
            chain_id,
        })
    }

    pub async fn update(&self, cid: String) -> Result<Option<TransactionReceipt>> {
      let data = self.contract.encode("updateCID", (cid,))?;
      let tx = TransactionRequest::new()
        .to(self.contract.address())
        .data(data)
        .chain_id(self.chain_id);
      match self.signer {
        Some(ref signer) => {
          let signed_tx = signer.send_transaction(tx, None).await?;
          let reciept = signed_tx.await?;
          Ok(reciept)
        }
        None => {
          panic!("No signer found");
        }
      }
    }

    pub async fn get(&self) -> Result<String> {
      let cid: String= self.contract
        .method::<_, String>("getCID", ())?
        .call()
        .await?;
      Ok(cid)
    }
}