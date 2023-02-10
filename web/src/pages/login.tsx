import Layout from "@/components/Layout";
import { useAppContext } from "@/context/state";
import { setToken } from "@/lib/api";
import { useRouter } from "next/router";
import jwt_decode from "jwt-decode";

interface Claims {
    exp: number,
    iat: number,
    sub: string
}

export default function Login() {
    const { user, setUser } = useAppContext();
    const router = useRouter();
    if (router.isReady) {
        const token = router.query.token as string;
        setToken(token);
        const decoded = jwt_decode<Claims>(token);
        setUser({ email: decoded.sub })
    }
    return (
        <Layout title="Login">
            {user ? (<p>Logged in as {user.email}</p>) : (<p>Logging in...</p>)}
        </Layout>
    )
}
