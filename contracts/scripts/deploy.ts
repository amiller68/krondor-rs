// @ts-ignore
import { ethers } from "hardhat";
import fs from "fs";

/* RootCid Deployment script */
async function main() {
  const gas = await ethers.provider.getGasPrice();
  const RootCid = await ethers.getContractFactory("RootCid");
  const rootCidData = RootCid.getDeployTransaction().data;
  const estimateGas = await ethers.provider.estimateGas({ data: rootCidData });

  console.log(
    "Deploying rootCid contract to network:",
    ethers.provider.network.name
  );

  console.log("Gas estimate: ", estimateGas.toString());
  console.log("Gas price: ", gas.toString());

  console.log("Would you like to deploy? (y/n)");
  const stdin = process.openStdin();
  stdin.addListener("data", async (d) => {
    const input = d.toString().trim();
    if (input !== "y") {
      console.log("Exiting...");
      process.exit(0);
    }
    const rootCid = await RootCid.deploy({
      gasLimit: estimateGas,
      gasPrice: gas,
    });
    await rootCid.deployed();
    console.log("RootCid deployed to:", rootCid.address);
    console.log(
      "Gas used:",
      (await rootCid.deployTransaction.wait()).gasUsed.toString()
    );
    console.log("Saving contract address to file...");
    let config_path = "./../../krondor.json";
    let save_path = "./../krondor.json";
    let config = require(config_path);
    config.eth.contract_address = rootCid.address;
    fs.writeFileSync(save_path, JSON.stringify(config, null, 2));
    console.log("Done!");
    process.exit(0);
  });
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
