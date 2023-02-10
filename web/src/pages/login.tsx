import Layout from "@/components/Layout";
import { useAppContext, User } from "@/context/state";
import { setToken } from "@/lib/api";
import { useRouter } from "next/router";
import jwt_decode from "jwt-decode";
import { useEffect } from "react";
import Link from "next/link";

interface Claims {
    exp: number,
    iat: number,
    sub: string
}

export default function Login() {
    const { user, setUser } = useAppContext();
    const router = useRouter();
    useEffect(() => {
        if (router.isReady) {
            const token = router.query.token;
            if (!token) {
                return;
            }
            const tokenString = token as string;
            setToken(tokenString);
            const decoded = jwt_decode<Claims>(tokenString);
            setUser({
                email: decoded.sub,
            })
        }
    }, [router.isReady])
    return (
        <Layout title="Login">
            <div className="hero min-h-screen bg-base-200">
                <div className="hero-content text-center">
                    <div className="max-w-md">
                        <p className="py-6">
                            {user ? (`Logged in as ${user.email}`) : (`Logging in...`)}
                        </p>
                        <Link href="/">
                            <button className="btn btn-primary">Starts doing tasks</button>
                        </Link>
                    </div>
                </div>
            </div>
        </Layout>
    )
}
