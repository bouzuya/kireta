import { createContext, useContext } from "react";
import { newStore, type Store } from "@/components/TodayScreen/types/store";

type StoreContextValue = {
  store: Store;
};

const StoreContext = createContext<StoreContextValue>({
  store: newStore(),
});

export function useStore(): StoreContextValue {
  return useContext(StoreContext);
}

type Props = {
  children: React.ReactNode;
};

export function StoreContextProvider({ children }: Props): JSX.Element {
  return (
    <StoreContext.Provider value={{ store: newStore() }}>
      {children}
    </StoreContext.Provider>
  );
}
