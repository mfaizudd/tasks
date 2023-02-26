import Dashboard from "@/components/Dashboard";
import { getAuthorizedApi } from "@/lib/api";
import { NextPage } from "next";
import { useRouter } from "next/router";
import { useState } from "react";

export const Create: NextPage = () => {
    const router = useRouter();
    const [name, setName] = useState("");
    const submit = async () => {
        try {
            const api = await getAuthorizedApi();
            const res = await api.post("/cohorts", { name: name });
            if (res.status === 201) {
                router.push("/cohorts")
            }
        } catch (err) {
            console.log(err);
        }
    }
    return (
        <Dashboard>
            <div className="p-5">
                <form onSubmit={e => { e.preventDefault(); submit() }}>
                    <div className="flex gap-3">
                        <input type="text" value={name} onChange={(e) => setName(e.target.value)} placeholder="Cohort name" className="input input-bordered w-full max-w-xs" />
                        <button className="btn btn-primary" type="submit">Submit</button>
                    </div>
                </form>
            </div>
        </Dashboard>
    )
}

export default Create;
