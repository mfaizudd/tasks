import Dashboard from "@/components/Dashboard";
import { getAuthorizedApi } from "@/lib/api";
import { GetServerSideProps, NextPage } from "next";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";

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
            id: id as string
        }
    }
}

const Edit: NextPage<Props> = ({ id }) => {
    const router = useRouter();
    const [name, setName] = useState<string>("");
    const fetchData = async () => {
        try {
            const api = await getAuthorizedApi();
            const res = await api.get(`/cohorts/${id}`);
            if (res.status === 200) {
                setName(res.data.data.name);
            }
        } catch (err) {
            console.log(err)
        }
    }
    const submit = async () => {
        try {
            const api = await getAuthorizedApi();
            const res = await api.put(`/cohorts/${id}`, { name });
            if (res.status === 200) {
                router.push("/cohorts");
            }
        } catch (err) {
            console.log(err);
        }
    }
    useEffect(() => {
        fetchData()
    }, [])
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

export default Edit;
