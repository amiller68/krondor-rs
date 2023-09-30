import { time, loadFixture } from "@nomicfoundation/hardhat-network-helpers";
const { anyValue } = require("@nomicfoundation/hardhat-chai-matchers/withArgs");
import { expect } from "chai";
// @ts-ignore
import { ethers } from "hardhat";

/* RootCid Contract Test Suite */

describe("RootCid", function () {
  async function fixture() {
    // Get the contract factory and signer
    const RootCid = await ethers.getContractFactory("RootCid");
    const [owner, otherAccount] = await ethers.getSigners();

    // Deploy the contract
    const rootCid = await RootCid.deploy();
    await rootCid.deployed();
    return { rootCid, owner, otherAccount };
  }

  it("should deploy the contract", async function () {
    const { rootCid } = await loadFixture(fixture);
    expect(rootCid.address).to.not.equal(anyValue());
  });

  it("should allow the owner to update the CID", async function () {
    const { rootCid, owner } = await loadFixture(fixture);
    const cid = "QmYXlkdu0md0d2";
    // @ts-ignore - chai-matchers doesn't get picked up, but this should work
    await expect(rootCid.updateCID(cid))
      .to.emit(rootCid, "updatedCID")
      .withArgs(cid);
    expect(await rootCid.getCID()).to.equal(cid);
  });

  it("should not allow a non-owner to update the CID", async function () {
    const { rootCid, otherAccount } = await loadFixture(fixture);
    const cid = "QmYXlkdu0md0d2";
    // @ts-ignore - chai-matchers doesn't get picked up, but this should work
    await expect(
      rootCid.connect(otherAccount).updateCID(cid)
    ).to.be.revertedWith("Ownable: caller is not the owner");
  });
});
