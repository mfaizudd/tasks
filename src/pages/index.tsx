import Layout from '@/components/Layout'
import { useAppContext } from '@/context/state'
import { auth } from '@/lib/firebase'
import { signInWithPopup, GoogleAuthProvider, signOut } from "firebase/auth"
import { useRouter } from 'next/router'
import { useEffect } from 'react'

export default function Home() {
    const router = useRouter()
    const state = useAppContext()
    useEffect(() => {
        if (state.user) {
            router.push("/dashboard")
        }
    }, [state.user])
    let button = <button className="btn btn-primary" onClick={() => signIn()}>Sign In</button>
    function signIn() {
        const provider = new GoogleAuthProvider();
        signInWithPopup(auth, provider).then(() => {
            router.push('/dashboard')
        }).catch((error) => {
            console.log(error)
        });
    }
    if (state.user) {
        button = <button className="btn btn-primary" onClick={() => signOut(auth)}>Sign Out</button>
    }
    return (
        <Layout>
            <div className="hero min-h-screen bg-base-200">
                <div className="hero-content text-center">
                    <div className="max-w-md">
                        <h1 className="text-5xl font-bold">Tasks</h1>
                        <p className="py-6">Do your homework man! (or not, I'm not your mom)</p>
                        {button}
                    </div>
                </div>
            </div>
        </Layout>
    )
}
