import { createContext, useContext, useState } from 'react';

interface Props {
    children?: React.ReactNode;
}

export interface User {
    email: string;
}

interface State {
    user: User | null;
    setUser: (user: User | null) => void;
}

const AppContext = createContext<State>({ user: null, setUser: () => { } });

export function AppWrapper({ children }: Props) {
    const [user, setUser] = useState<User | null>(null);
    const state: State = { user, setUser };

    return (
        <AppContext.Provider value={state}>
            {children}
        </AppContext.Provider>
    )
}

export function useAppContext() {
    return useContext(AppContext);
}
