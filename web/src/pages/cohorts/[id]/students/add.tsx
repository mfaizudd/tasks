import Dashboard from "@/components/Dashboard";
import { getAuthorizedApi } from "@/lib/api";
import { GetServerSideProps, NextPage } from "next";
import { useRouter } from "next/router";
import { useState } from "react";

interface Props {
    id?: string;
}

export const getServerSideProps: GetServerSideProps<Props> = async ({ params }) => {
    const id = params?.id;
    if (!id) {
        return {
            props: {}
        }
    }

    return {
        props: {
            id: id as string,
        }
    }
}

export const Add: NextPage<Props> = ({ id }) => {
    const router = useRouter();
    const [name, setName] = useState("");
    const [number, setNumber] = useState("");
    const submit = async () => {
        try {
            const api = await getAuthorizedApi();
            const res = await api.post(`/students`, {
                name: name,
                number: number,
                cohort_id: id,
            });
            if (res.status === 201) {
                router.push(`/cohorts/${id}`)
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
                        <input type="text" value={name} onChange={(e) => setName(e.target.value)} placeholder="Student name" className="input input-bordered w-full max-w-xs" />
                        <input type="text" value={number} onChange={(e) => setNumber(e.target.value)} placeholder="Number" className="input input-bordered w-full max-w-xs" />
                        <button className="btn btn-primary" type="submit">Submit</button>
                    </div>
                </form>
            </div>
        </Dashboard>
    )
}

export default Add;
