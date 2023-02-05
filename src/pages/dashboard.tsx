import Layout from "@/components/Layout"
import Link from "next/link"
import { onAuthStateChanged, signOut } from "firebase/auth"
import { auth } from "@/lib/firebase"
import { useRouter } from "next/router"

export default function Dashboard() {
    const router = useRouter()
    onAuthStateChanged(auth, (user) => {
        if (!user) {
            router.push("/")
        }
    })

    return (
        <Layout title="Dashboard">
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
                            <a href="#" onClick={() => signOut(auth)}>Sign out</a>
                        </li>
                    </ul>

                </div>
            </div>
        </Layout>
    )
}
