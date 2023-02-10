import Layout from '@/components/Layout'
import { useAppContext } from '@/context/state'
import { removeToken } from '@/lib/api'
import { useRouter } from 'next/router'

export default function Home() {
    const router = useRouter()
    const { user, setUser } = useAppContext()
    let button = <button className="btn btn-primary" onClick={() => signIn()}>Sign In</button>
    if (user !== null) {
        button = <button className="btn btn-primary" onClick={() => signOut()}>Sign Out</button>
    }
    function signOut() {
        removeToken()
        setUser(null)
    }
    function signIn() {
        router.push('http://localhost:8000/api/auth/google?redirect_url=http://localhost:3000/login')
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
