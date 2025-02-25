import { ethers, Overrides } from "ethers";
import {
  createWrappedOnAlgorand,
  createWrappedOnSolana,
  createWrappedOnTerra,
  createWrappedOnNear,
  submitVAAOnInjective,
  createWrappedOnXpla,
  createWrappedOnAptos,
} from ".";
import { Bridge__factory } from "../ethers-contracts";

export async function updateWrappedOnEth(
  tokenBridgeAddress: string,
  signer: ethers.Signer,
  signedVAA: Uint8Array,
  overrides: Overrides & { from?: string | Promise<string> } = {}
) {
  const bridge = Bridge__factory.connect(tokenBridgeAddress, signer);
  const v = await bridge.updateWrapped(signedVAA, overrides);
  const receipt = await v.wait();
  return receipt;
}

export const updateWrappedOnTerra = createWrappedOnTerra;

export const updateWrappedOnInjective = submitVAAOnInjective;

export const updateWrappedOnXpla = createWrappedOnXpla;

export const updateWrappedOnSolana = createWrappedOnSolana;

export const updateWrappedOnAlgorand = createWrappedOnAlgorand;

export const updateWrappedOnNear = createWrappedOnNear;

export const updateWrappedOnAptos = createWrappedOnAptos;
