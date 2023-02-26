import Dashboard from "@/components/Dashboard";
import { Loading } from "@/components/Loading";
import { getAuthorizedApi } from "@/lib/api";
import { Student } from "@/lib/entities";
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
    const [loading, setLoading] = useState<boolean>(true);
    const [students, setStudents] = useState<Student[]>([]);
    const [name, setName] = useState<string>("");
    const fetchData = async () => {
        try {
            const api = await getAuthorizedApi();
            const res = await api.get(`/cohorts/${id}`);
            if (res.status === 200) {
                setName(res.data.data.name);
                setLoading(false);
            }
            const studentsRes = await api.get(`/cohorts/${id}/students`);
            if (studentsRes.status === 200) {
                setStudents(studentsRes.data.data);
                setLoading(false);
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
            <div className="p-5 flex flex-col gap-3">
                {loading ? <Loading /> : (
                    <>
                        <form onSubmit={e => { e.preventDefault(); submit() }}>
                            <div className="flex gap-3">
                                <input type="text" value={name} onChange={(e) => setName(e.target.value)} placeholder="Cohort name" className="input input-bordered w-full max-w-xs" />
                                <button className="btn btn-primary" type="submit">Submit</button>
                            </div>
                        </form>
                        <div className="overflow-x-auto">
                            <table className="table w-full">
                                <thead>
                                    <tr>
                                        <th></th>
                                        <th>Number</th>
                                        <th>Name</th>
                                        <th></th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {students.map((student, i) => (
                                        <tr key={student.id} className="hover">
                                            <th>{i + 1}</th>
                                            <td>{student.number}</td>
                                            <td>{student.name}</td>
                                        </tr>
                                    ))}
                                </tbody>
                            </table>
                        </div>
                    </>
                )}
            </div>
        </Dashboard>
    )
}

export default Edit;
