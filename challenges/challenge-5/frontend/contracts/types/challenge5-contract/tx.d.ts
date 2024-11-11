// Generated by dedot cli

import type { GenericSubstrateApi } from 'dedot/types';
import type { AccountId32Like, BytesLike } from 'dedot/codecs';
import type {
  GenericContractTx,
  GenericContractTxCall,
  ContractTxOptions,
  ContractSubmittableExtrinsic,
} from 'dedot/contracts';

export interface ContractTx<ChainApi extends GenericSubstrateApi> extends GenericContractTx<ChainApi> {
  /**
   *
   * @param {AccountId32Like} voter
   * @param {ContractTxOptions} options
   *
   * @selector 0x81b2cf8b
   **/
  registerVoter: GenericContractTxCall<
    ChainApi,
    (voter: AccountId32Like, options: ContractTxOptions) => ContractSubmittableExtrinsic<ChainApi>
  >;

  /**
   *
   * @param {AccountId32Like} voter
   * @param {ContractTxOptions} options
   *
   * @selector 0xa4279724
   **/
  deregisterVoter: GenericContractTxCall<
    ChainApi,
    (voter: AccountId32Like, options: ContractTxOptions) => ContractSubmittableExtrinsic<ChainApi>
  >;

  /**
   *
   * @param {AccountId32Like} voter
   * @param {BytesLike} encodedExtrinsic
   * @param {bigint} feeMax
   * @param {bigint} refTime
   * @param {bigint} proofSize
   * @param {ContractTxOptions} options
   *
   * @selector 0x547cc8db
   **/
  createSuperdaoCrossChainProposal: GenericContractTxCall<
    ChainApi,
    (
      voter: AccountId32Like,
      encodedExtrinsic: BytesLike,
      feeMax: bigint,
      refTime: bigint,
      proofSize: bigint,
      options: ContractTxOptions,
    ) => ContractSubmittableExtrinsic<ChainApi>
  >;

  /**
   *
   * @param {AccountId32Like} voter
   * @param {ContractTxOptions} options
   *
   * @selector 0x0071768f
   **/
  createContractCallProposal: GenericContractTxCall<
    ChainApi,
    (voter: AccountId32Like, options: ContractTxOptions) => ContractSubmittableExtrinsic<ChainApi>
  >;

  /**
   *
   * @param {ContractTxOptions} options
   *
   * @selector 0x9bcaeb73
   **/
  updateValue: GenericContractTxCall<ChainApi, (options: ContractTxOptions) => ContractSubmittableExtrinsic<ChainApi>>;

  /**
   *
   * @param {ContractTxOptions} options
   *
   * @selector 0xca6f2170
   **/
  getValue: GenericContractTxCall<ChainApi, (options: ContractTxOptions) => ContractSubmittableExtrinsic<ChainApi>>;

  /**
   *
   * @param {number} proposalId
   * @param {boolean} vote
   * @param {AccountId32Like} voter
   * @param {ContractTxOptions} options
   *
   * @selector 0x946595e4
   **/
  voteProposal: GenericContractTxCall<
    ChainApi,
    (
      proposalId: number,
      vote: boolean,
      voter: AccountId32Like,
      options: ContractTxOptions,
    ) => ContractSubmittableExtrinsic<ChainApi>
  >;
}
