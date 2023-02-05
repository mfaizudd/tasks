import { auth } from '@/lib/firebase';
import { onAuthStateChanged, User } from 'firebase/auth';
import { createContext, useContext, useState } from 'react';

interface Props {
    children?: React.ReactNode;
}

interface State {
    user: User | null;
}

const AppContext = createContext<State>({ user: null });

export function AppWrapper({ children }: Props) {
    const [state, setState] = useState<State>({ user: null });
    onAuthStateChanged(auth, (user) => {
        setState({ user });
    });

    return (
        <AppContext.Provider value={state}>
            {children}
        </AppContext.Provider>
    )
}

export function useAppContext() {
    return useContext(AppContext);
}
