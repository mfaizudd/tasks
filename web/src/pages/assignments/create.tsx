import Dashboard from "@/components/Dashboard";
import { Loading } from "@/components/Loading";
import { getAuthorizedApi } from "@/lib/api";
import { Cohort } from "@/lib/entities";
import { NextPage } from "next";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";

export const Create: NextPage = () => {
    const router = useRouter();
    const [name, setName] = useState<string>("");
    const [description, setDescription] = useState<string>("");
    const [cohort_id, setCohortId] = useState<string>("");
    const [cohorts, setCohorts] = useState<Cohort[]>([]);
    const [loading, setLoading] = useState<boolean>(false);

    const fetchData = async () => {
        setLoading(true);
        try {
            const api = await getAuthorizedApi();
            const res = await api.get("/cohorts");
            if (res.status === 200) {
                setCohorts(res.data.data);
            }
        } catch (error) {
            console.log(error);
        } finally {
            setLoading(false);
        }
    }

    const submit = async () => {
        try {
            const api = await getAuthorizedApi();
            const res = await api.post("/assignments", { name, description, cohort_id });
            if (res.status === 201) {
                router.push("/assignments");
            }
        } catch (error) {
            console.log(error);
        }
    }

    useEffect(() => {
        fetchData();
    }, [])

    if (loading) {
        return <Loading />
    }

    return (
        <Dashboard>
            <div className="p-5 w-full flex justify-center">
                <form onSubmit={e => { e.preventDefault(); submit() }}>
                    <div className="flex flex-col gap-3">
                        <input type="text" value={name} onChange={(e) => setName(e.target.value)} placeholder="Assignment name" className="input input-bordered w-full max-w-xs" />
                        <input type="text" value={description} onChange={(e) => setDescription(e.target.value)} placeholder="Assignment description" className="input input-bordered w-full max-w-xs" />
                        <select value={cohort_id} onChange={(e) => setCohortId(e.target.value)} className="select w-full max-w-xs select-bordered">
                            {cohorts.map(cohort => (
                                <option key={cohort.id} value={cohort.id}>{cohort.name}</option>
                            ))}
                        </select>
                        <button className="btn btn-primary w-full max-w-xs" type="submit">Submit</button>
                    </div>
                </form>
            </div>
        </Dashboard>
    )
}

export default Create;
