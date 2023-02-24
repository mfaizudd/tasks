import Layout from "@/components/Layout";
import { useAppContext } from "@/context/state";
import { setRefreshToken, setToken } from "@/lib/api";
import { useRouter } from "next/router";
import jwt_decode from "jwt-decode";
import { useEffect } from "react";
import Link from "next/link";
import { GetServerSideProps } from "next";
import { exchangeCodeForToken, getCodeVerifier } from "@/lib/oauth";

interface Claims {
    exp: number,
    iat: number,
    sub: string
}

interface Props {
    auth_code?: string
}

export const getServerSideProps: GetServerSideProps<Props> = async ({ query }) => {
    const auth_code = query.code;
    if (!auth_code) {
        return {
            props: {}
        }
    }

    return {
        props: {
            auth_code: auth_code as string
        }
    }
}

export default function Login({ auth_code }: Props) {
    // const { user, setUser } = useAppContext();
    const router = useRouter();
    useEffect(() => {
        if (auth_code) {
            const code_verifier = getCodeVerifier();
            (async () => {
                const token_response = await exchangeCodeForToken(auth_code, code_verifier ?? "");
                if (token_response) {
                    setToken(token_response.access_token);
                    setRefreshToken(token_response.refresh_token);
                    router.push("/dashboard");
                }
            })();
        }
    }, [])
    return (
        <Layout title="Login">
            <div className="hero min-h-screen bg-base-200">
                <div className="hero-content text-center">
                    <div className="max-w-md">
                        <p className="py-6">
                            {/*user ? (`Logged in as ${user.email}`) : (`Logging in...`)*/}
                        </p>
                        <Link href="/dashboard">
                            <button className="btn btn-primary">Starts doing tasks</button>
                        </Link>
                    </div>
                </div>
            </div>
        </Layout>
    )
}
