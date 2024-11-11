import {Box, Button, Flex, Heading, Input, Spinner, Text} from '@chakra-ui/react';
import React, {useState} from 'react';
import { Proposal } from '@/components/minidao/Proposal.tsx';
import { useApp } from '@/providers/AppProvider.tsx';
import { txToaster } from '@/utils/txToaster.tsx';
import { useContractTx } from 'typink';
import { useWatchContractQuery } from 'typink/hooks/useContractQuery.ts';

interface ProposalsPanelProps {
    address: string;
}

export function ProposalsPanel({ address }: ProposalsPanelProps) {
    const { minidaoContract: contract } = useApp();
    const { superdaoContract: superContract } = useApp();

    const [encodedExtrinsic, setEncodedExtrinsic] = useState<string>('');
    const createCrossChainProposalTx = useContractTx(contract, 'createSuperdaoCrossChainProposal');
    const createContractCallProposalTx = useContractTx(contract, 'createContractCallProposal');
    const { data: proposals, isLoading } = useWatchContractQuery({
        contract: superContract,
        fn: 'superDaoQueryGetProposals',
    });

    const feeMax: bigint = BigInt(1);
    const refTime: bigint = BigInt(100000);
    const proofSize: bigint = BigInt(0);

    const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        setEncodedExtrinsic(event.target.value);
    };

    const doCreateProposal = async () => {
        const toaster = txToaster('Signing transaction...');
        try {

            await createCrossChainProposalTx.signAndSend({
                args: [address, encodedExtrinsic, feeMax, refTime, proofSize],
                callback: ({ status }) => {
                    toaster.updateTxStatus(status);
                },
            });
        } catch (e: any) {
            console.error(e);
            toaster.onError(e);
        }
    };

    const doCreateContractCallProposal = async () => {
        const toaster = txToaster('Signing transaction...');
        try {
            await createContractCallProposalTx.signAndSend({
                args: [address],
                callback: ({ status }) => {
                    toaster.updateTxStatus(status);
                },
            });
        } catch (e: any) {
            console.error(e);
            toaster.onError(e);
        }
    };

    return (
        <Box mt={4}>
            <Heading size='sm'>Proposals</Heading>
            <Flex direction='column' gap={4}>

                <Input
                    type='number'
                    value={encodedExtrinsic}
                    onChange={handleInputChange}
                    placeholder='Enter encoded extrinsic'
                    mt={2}
                />

                <Button mt={4} size='sm' onClick={doCreateProposal}
                        isLoading={createCrossChainProposalTx.inBestBlockProgress}>
                    Create Cross Chain Proposal
                </Button>

                <Button
                    mt={4}
                    size='sm'
                    onClick={doCreateContractCallProposal}
                    isLoading={createContractCallProposalTx.inBestBlockProgress}>
                    Create Contract Call Proposal
                </Button>
            </Flex>

            <Box mt={4}>
                {isLoading && <Spinner />}
                {proposals && proposals.length === 0 && <Text>No proposals</Text>}
                {proposals && (
                    <Flex direction='column' gap={2}>
                        {proposals.map(([index, p], idx) => (
                            <Proposal proposal={p} index={index} key={idx}  address={address}/>
                        ))}
                    </Flex>
                )}
            </Box>
        </Box>
    );
}