import {Box, Heading, Text} from "@chakra-ui/react";
import { useApp } from "@/providers/AppProvider.tsx";
import {useContractQuery, useTypink} from "typink";
import React from "react";
import {VoterInfo} from "@/components/minidao/VoterInfo.tsx";
import {ProposalsPanel} from "@/components/minidao/ProposalsPanel.tsx";

export function MiniDaoBoard() {

    const { selectedAccount } = useTypink()
    const { minidaoContract: contract} = useApp();
    const { data: name, isLoading } = useContractQuery({
        contract,
        fn: 'getName'
    })


    return (
        <Box>
            <Heading size='md'> { name } </Heading>
            {selectedAccount
                ? (
                    <Box>
                        <VoterInfo address={selectedAccount.address} />
                        <ProposalsPanel  address={selectedAccount.address}/>
                    </Box>
                )
                : (<Text>Connect to your wallet to show voter info!</Text>)}

        </Box>
    )
}