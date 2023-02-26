import { useAppContext } from "@/context/state";
import { getUserInfo } from "@/lib/api";
import { useRouter } from "next/router";
import React, { useEffect } from "react";
import { Loading } from "./Loading";

interface Props {
    children?: React.ReactNode;
}

const Auth: React.FC<Props> = ({ children }) => {
    const router = useRouter();
    const { user, setUser } = useAppContext();

    useEffect(() => {
        const userInfo = getUserInfo();
        if (userInfo) {
            setUser(userInfo);
        } else {
            router.push("/")
        }
    }, [])
    let content = <Loading />;
    if (user) {
        content = <>{children}</>;
    }
    return content;
}

export default Auth;
