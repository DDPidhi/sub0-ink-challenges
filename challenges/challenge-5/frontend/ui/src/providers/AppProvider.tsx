import { createContext, useContext } from 'react';
import { Props } from '@/types.ts';
import { Contract } from 'dedot/contracts';
import { Challenge5ContractContractApi } from 'contracts/types/challenge5-contract';
import {SuperdaoContractApi} from "contracts/types/superdao";
import { useContract } from 'typink';
import { ContractId } from 'contracts/deployments.ts';


interface AppContextProps {
  minidaoContract?: Contract<Challenge5ContractContractApi>
  superdaoContract?: Contract<SuperdaoContractApi>
}

const AppContext = createContext<AppContextProps>(null as any);

export const useApp = () => {
  return useContext(AppContext);
};

export function AppProvider({ children }: Props) {
  const { contract: minidaoContract } = useContract<Challenge5ContractContractApi>(ContractId.MINIDAO);
  const { contract: superdaoContract } = useContract<SuperdaoContractApi>(ContractId.SUPERDAO);

  return (
    <AppContext.Provider value={{ minidaoContract, superdaoContract }}>
      {children}
    </AppContext.Provider>
  );
}
