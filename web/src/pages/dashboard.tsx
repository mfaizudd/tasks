import Layout from "@/components/Layout"
import Link from "next/link"
import { useRouter } from "next/router"
import { useAppContext } from "@/context/state"
import { removeToken } from "@/lib/api"
import { useEffect } from "react"
import { Loading } from "@/components/Loading"

export default function Dashboard() {
    const router = useRouter()
    const state = useAppContext()
    const { setUser } = useAppContext()
    useEffect(() => {
        if (!state.user) {
            router.push("/")
        }
    })
    function signOut() {
        removeToken()
        setUser(null)
        router.push("/")
    }

    return (
        <Layout title="Dashboard">
            {state.user ? (
                <div className="drawer drawer-mobile">
                    <input id="my-drawer-2" type="checkbox" className="drawer-toggle" />
                    <div className="drawer-content flex flex-col p-3">
                        <label htmlFor="my-drawer-2" className="btn drawer-button lg:hidden">Menu</label>
                    </div>
                    <div className="drawer-side">
                        <label htmlFor="my-drawer-2" className="drawer-overlay"></label>
                        <ul className="menu p-4 w-80 bg-base-100 text-base-content">
                            <li>
                                <Link href="/dashboard">Home</Link>
                            </li>
                            <li>
                                <a href="#" onClick={() => signOut()}>Sign out</a>
                            </li>
                        </ul>

                    </div>
                </div>
            ) : (
                <Loading />
            )}
        </Layout>
    )
}
