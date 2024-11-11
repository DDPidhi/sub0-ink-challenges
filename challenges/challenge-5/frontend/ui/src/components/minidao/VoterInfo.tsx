import {Box, Button, Spinner, Text, Input, FormControl, FormLabel, Switch} from '@chakra-ui/react';
import { useApp } from '@/providers/AppProvider.tsx';
import { txToaster } from '@/utils/txToaster.tsx';
import {useContractQuery, useContractTx, useWatchContractQuery} from 'typink';
import {useState} from "react";

interface VoterInfoProps {
    address: string;
}

export function VoterInfo({ address }: VoterInfoProps) {
    const { minidaoContract: contract } = useApp();

    const { data: isVoter, isLoading, refresh } = useContractQuery({ contract, fn: 'hasVoter', args: [address] });
    const registerVoterTx = useContractTx(contract, 'registerVoter');
    const deregisterVoterTx = useContractTx(contract, 'deregisterVoter');
    const voteProposalTx = useContractTx(contract, 'voteProposal');

    const registerVoter = async () => {
        const toaster = txToaster('Signing transaction...');
        try {
            await registerVoterTx.signAndSend({
                args: [address],
                callback: ({ status }) => {
                    if (status.type === 'BestChainBlockIncluded') {
                        refresh();
                    }
                    toaster.updateTxStatus(status);
                }
            });
        } catch (e: any) {
            toaster.onError(e);
        }
    };

    const deregisterVoter = async () => {
        const toaster = txToaster('Signing transaction...');
        try {
            await deregisterVoterTx.signAndSend({
                callback: ({status}) => {
                    if (status.type === 'BestChainBlockIncluded') {
                        refresh();
                    }

                    toaster.updateTxStatus(status);
                },
                args: [address]
            });
        } catch (e: any) {
            toaster.onError(e);
        }
    };

    if (isLoading) {
        return (
            <Box mt={4}>
                <Spinner />
            </Box>
        )
    }

    return (
        <>
            {isVoter ? (
                <Box mt={2}>
                    <Text>You're a voter</Text>
                    <Button mt={2} size='sm' onClick={deregisterVoter} isLoading={deregisterVoterTx.inBestBlockProgress}>
                        Deregister
                    </Button>
                    <br/>
                </Box>
            ) : (
                <Box mt={2}>
                    <Text>You're not a voter, register now!</Text>
                    <Button mt={2} size='sm' onClick={registerVoter} isLoading={registerVoterTx.inBestBlockProgress}>
                        Register as voter
                    </Button>
                    <br/>
                </Box>
            )}
        </>
    );
}