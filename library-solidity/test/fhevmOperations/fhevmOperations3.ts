import { expect } from 'chai';
import { ethers } from 'hardhat';

import type {
  FHEVMTestSuite1,
  FHEVMTestSuite2,
  FHEVMTestSuite3,
  FHEVMTestSuite4,
  FHEVMTestSuite5,
  FHEVMTestSuite6,
  FHEVMTestSuite7
} from '../../types/contracts/tests';
import {
  createInstances,
  decrypt8, decrypt16, decrypt32, decrypt64, decrypt128, decrypt256, decryptBool
} from '../instance';
import { getSigners, initSigners } from '../signers';

async function deployFHEVMTestFixture<T>(name: string): Promise<T> {
  const { alice: admin } = await getSigners();
  const contractFactory = await ethers.getContractFactory(name);
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();
  return contract as T;
}

describe('FHEVM operations 3', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    this.contract1 = await deployFHEVMTestFixture<FHEVMTestSuite1>('FHEVMTestSuite1');
    this.contract1Address = await this.contract1.getAddress();

    this.contract2 = await deployFHEVMTestFixture<FHEVMTestSuite2>('FHEVMTestSuite2');
    this.contract2Address = await this.contract2.getAddress();

    this.contract3 = await deployFHEMV...
