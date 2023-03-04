import { useAppContext } from "@/context/state"
import { removeRefreshToken, removeToken } from "@/lib/api"
import Link from "next/link"
import { useRouter } from "next/router"
import React from "react"
import Auth from "./Auth"
import Layout from "./Layout"

export interface Action {
    label: string
    href: string
}

interface Props {
    title?: string
    children?: React.ReactNode
    actions?: Action[]
}

const Dashboard: React.FC<Props> = ({ children, title, actions }) => {
    const router = useRouter()
    const { setUser } = useAppContext()
    function signOut() {
        removeToken()
        removeRefreshToken()
        setUser(null)
        router.push("/")
    }

    return <Layout title={title ?? "Dashboard"}>
        <Auth>
            <div className="drawer drawer-mobile">
                <input id="my-drawer-2" type="checkbox" className="drawer-toggle" />
                <div className="drawer-content flex flex-col">
                    <label htmlFor="my-drawer-2" className="btn drawer-button lg:hidden">Menu</label>
                    <div className="w-full navbar bg-base-300">
                        <div className="flex-none lg:hidden">
                            <label htmlFor="my-drawer-3" className="btn btn-square btn-ghost">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" className="inline-block w-6 h-6 stroke-current"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                            </label>
                        </div>
                        <div className="flex-1 px-2 mx-2">{title ?? "Dashboard"}</div>
                        <div className="flex-none hidden lg:block">
                            <ul className="menu menu-horizontal">
                                {actions?.map((action, index) => (
                                    <li key={index}>
                                        <Link href={action.href}>
                                            {action.label}
                                        </Link>
                                    </li>
                                ))}
                            </ul>
                        </div>
                    </div>
                    {children}
                </div>
                <div className="drawer-side">
                    <label htmlFor="my-drawer-2" className="drawer-overlay"></label>
                    <ul className="menu p-4 w-80 bg-base-100 text-base-content">
                        <li>
                            <Link href="/dashboard">Home</Link>
                        </li>
                        <li>
                            <Link href="/cohorts">Cohorts</Link>
                        </li>
                        <li>
                            <Link href="/assignments">Assignments</Link>
                        </li>
                        <li>
                            <a href="#" onClick={() => signOut()}>Sign out</a>
                        </li>
                    </ul>

                </div>
            </div>
        </Auth>
    </Layout>
}

export default Dashboard
