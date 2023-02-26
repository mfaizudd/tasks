import Dashboard from "@/components/Dashboard";
import { Loading } from "@/components/Loading";
import { getAuthorizedApi } from "@/lib/api";
import { GetServerSideProps, NextPage } from "next";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";

interface Props {
    id?: string;
    student_id?: string;
}

export const getServerSideProps: GetServerSideProps<Props> = async ({ params }) => {
    const id = params?.id;
    const student_id = params?.student_id;
    if (!id && !student_id) {
        return {
            props: {}
        }
    }

    return {
        props: {
            id: id as string,
            student_id: student_id as string
        }
    }
}

const Edit: NextPage<Props> = ({ id, student_id }) => {
    const router = useRouter();
    const [loading, setLoading] = useState<boolean>(true);
    const [name, setName] = useState<string>("");
    const [number, setNumber] = useState<string>("");
    const [cohort_id, setCohortId] = useState<string>("");
    const fetchData = async () => {
        try {
            const api = await getAuthorizedApi();
            const res = await api.get(`/students/${student_id}`);
            if (res.status === 200) {
                setName(res.data.data.name);
                setNumber(res.data.data.number);
                setCohortId(res.data.data.cohort_id);
                setLoading(false);
            }
        } catch (err) {
            console.log(err)
        }
    }
    const submit = async () => {
        try {
            const api = await getAuthorizedApi();
            const res = await api.put(`/students/${student_id}`, {
                name,
                number,
                cohort_id
            });
            if (res.status === 200) {
                router.push(`/cohorts/${id}`);
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
            <div className="p-5 flex flex-col gap-3">
                {loading ? <Loading /> : (
                    <>
                        <form onSubmit={e => { e.preventDefault(); submit() }}>
                            <div className="flex gap-3">
                                <input type="text" value={name} onChange={(e) => setName(e.target.value)} placeholder="Student name" className="input input-bordered w-full max-w-xs" />
                                <input type="text" value={number} onChange={(e) => setNumber(e.target.value)} placeholder="Number" className="input input-bordered w-full max-w-xs" />
                                <button className="btn btn-primary" type="submit">Submit</button>
                            </div>
                        </form>
                    </>
                )}
            </div>
        </Dashboard>
    )
}

export default Edit;
