import { ContractDeployment, NetworkId } from 'typink';
import minidaoMetadata from './artifacts/challenge_5_contract/challenge_5_contract.json';
import superdaoMetadata from './artifacts/superdao/superdao.json';

export enum ContractId {
  MINIDAO = 'challenge5',
  SUPERDAO = 'superdao'
}

export const minidaoDeployments: ContractDeployment[] = [
  {
    id: ContractId.MINIDAO,
    metadata: minidaoMetadata as any,
    network: NetworkId.POP_TESTNET,
    address: '12eFHM8Q5uSkokCjT6hMN6DTDnUDqWxKoV6D5tAS4CqUppbh',
  },
];

export const superdaoDeployments: ContractDeployment[] = [
  {
    id: ContractId.SUPERDAO,
    metadata: superdaoMetadata as any,
    network: NetworkId.POP_TESTNET,
    address: '12oasvoSoRKxQ7ABrjgCftm78g4wvfyLQ33prWK69htwNsc7',
  },
];

export const deployments = [...minidaoDeployments, ...superdaoDeployments];
