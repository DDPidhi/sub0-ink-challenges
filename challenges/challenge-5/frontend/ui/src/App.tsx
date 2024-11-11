import { Box, Flex, Tab, TabList, TabPanel, TabPanels, Tabs } from '@chakra-ui/react';
import { useState } from 'react';
import { useSearchParam } from 'react-use';
import {MiniDaoBoard} from '@/components/MiniDaoBoard.tsx';
import BalanceInsufficientAlert from '@/components/shared/BalanceInsufficientAlert.tsx';
import MainFooter from '@/components/shared/MainFooter';
import MainHeader from '@/components/shared/MainHeader';
import { BlockInfo } from '@/components/shared/BlockInfo.tsx';

function App() {
  const tab = useSearchParam('tab');
  const tabIndex = tab ? parseInt(tab) : 0;
  const [index, setIndex] = useState(tabIndex);

  const handleTabsChange = (index: number) => {
    setIndex(index);
    history.pushState({}, '', location.pathname + `?tab=${index}`);
  };

  return (
    <Flex direction='column' minHeight='100vh'>
      <MainHeader />
      <Box maxWidth='container.md' mx='auto' my={4} px={4} flex={1} w='full'>
        <BalanceInsufficientAlert />
        <BlockInfo />
        <Tabs mt={4} index={index} onChange={handleTabsChange}>
          <TabList>
            <Tab>Minidao Contract</Tab>
          </TabList>

          <TabPanels>
            <TabPanel>
              <MiniDaoBoard/>
            </TabPanel>
          </TabPanels>
        </Tabs>
      </Box>
      <MainFooter />
    </Flex>
  );
}

export default App;
