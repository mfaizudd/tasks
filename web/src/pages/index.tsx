import Layout from '@/components/Layout'
import { useAppContext } from '@/context/state'
import { auth } from '@/lib/firebase'
import { onAuthStateChanged, signInWithPopup, GoogleAuthProvider, signOut, User } from "firebase/auth"
import { useRouter } from 'next/router'
import { useEffect, useState } from 'react'

export default function Home() {
    const router = useRouter()
    const [provider] = useState(new GoogleAuthProvider());
    let button = <button className="btn btn-primary" onClick={() => signIn()}>Sign In</button>
    const [user, setUser] = useState<User | null>(null)
    onAuthStateChanged(auth, (user) => {
        setUser(user)
    })
    if (user !== null) {
        button = <button className="btn btn-primary" onClick={() => signOut(auth)}>Sign Out</button>
    }
    const signIn = async () => {
        signInWithPopup(auth, provider)
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
