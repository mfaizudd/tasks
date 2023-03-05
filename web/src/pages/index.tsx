import Layout from '@/components/Layout'
import { useAppContext } from '@/context/state'
import { getUserInfo, removeRefreshToken, removeToken } from '@/lib/api'
import { oauthSignIn } from '@/lib/oauth'
import Link from 'next/link'
import { useEffect } from 'react'

export default function Home() {
    const { user, setUser } = useAppContext()
    let button = <button className="btn btn-primary" onClick={() => signIn()}>Sign In</button>
    if (user !== null) {
        button = <button className="btn btn-primary" onClick={() => signOut()}>Sign Out</button>
    }
    useEffect(() => {
        const user = getUserInfo()
        if (user !== null) {
            setUser(user)
        }
    }, [])
    function signOut() {
        removeToken()
        removeRefreshToken()
        setUser(null)
    }
    function signIn() {
        oauthSignIn()
    }
    return (
        <Layout>
            <div className="hero min-h-screen bg-base-200">
                <div className="hero-content text-center">
                    <div className="max-w-md">
                        <h1 className="text-5xl font-bold">Tasks</h1>
                        <p className="py-6">Do your homework man! (or not, I'm not your mom)</p>
                        {user !== null && <p className="p-2">Welcome {user.name}</p>}
                        <div className="flex gap-2 justify-center">
                            {user !== null && <Link href="/dashboard" className="btn btn-primary">Start doing some tasks</Link>}
                            {button}
                        </div>
                    </div>
                </div>
            </div>
        </Layout>
    )
}
