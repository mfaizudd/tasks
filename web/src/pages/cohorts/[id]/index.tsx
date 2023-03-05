import Dashboard from "@/components/Dashboard";
import { Loading } from "@/components/Loading";
import { getAuthorizedApi } from "@/lib/api";
import { Student } from "@/lib/entities";
import { GetServerSideProps, NextPage } from "next";
import Link from "next/link";
import { useRouter } from "next/router";
import { useCallback, useEffect, useState } from "react";

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
    const [page, setPage] = useState<number>(1);
    const fetchData = useCallback(async () => {
        setLoading(true);
        try {
            const api = await getAuthorizedApi();
            const res = await api.get(`/cohorts/${id}`);
            if (res.status === 200) {
                setName(res.data.data.name);
            }
            const studentsRes = await api.get(`/cohorts/${id}/students?page=${page}`);
            if (studentsRes.data.data.length === 0 && page > 1) {
                setPage(page - 1);
                return;
            }
            if (studentsRes.status === 200) {
                setStudents([...studentsRes.data.data]);
                setLoading(false);
            }
        } catch (err) {
            console.log(err)
        }
    }, [id, page]);
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

    const deleteStudent = async (id: string) => {
        if (confirm("Are you sure you want to delete this cohort?")) {
            try {
                const api = await getAuthorizedApi();
                await api.delete(`/students/${id}`);
                await fetchData();
            } catch (err) {
                console.log(err)
            }
        }
    }

    const previous = async () => {
        if (page > 1) {
            setPage(page - 1);
            await fetchData();
        }
    }

    const next = async () => {
        setPage(page + 1);
    }

    useEffect(() => {
        fetchData()
    }, [fetchData])
    const actions = [
        {
            label: "Upload students",
            href: `/cohorts/${id}/students/upload`,
        },
        {
            label: "Add student",
            href: `/cohorts/${id}/students/add`,
        }
    ]
    return (
        <Dashboard actions={actions}>
            {loading ? <Loading /> : (
                <div className="p-3 flex flex-col gap-3">
                    <div className="flex gap-3 justify-between">
                        <form onSubmit={e => { e.preventDefault(); submit() }}>
                            <div className="flex gap-3">
                                <input type="text" value={name} onChange={(e) => setName(e.target.value)} placeholder="Cohort name" className="input input-bordered w-full max-w-xs" />
                                <button className="btn btn-primary" type="submit">Submit</button>
                            </div>
                        </form>
                        <div className="flex gap-3">
                            <button className="btn" onClick={previous}>Previous</button>
                            <button className="btn" onClick={next}>Next</button>
                        </div>
                    </div>
                    <div className="overflow-x-auto">
                        <table className="table w-full">
                            <thead>
                                <tr>
                                    <th>Number</th>
                                    <th>Name</th>
                                    <th></th>
                                </tr>
                            </thead>
                            <tbody>
                                {students.map((student) => (
                                    <tr key={student.id} className="hover">
                                        <td>{student.number}</td>
                                        <td>{student.name}</td>
                                        <td className="flex gap-3">
                                            <Link className="btn" href={`/cohorts/${id}/students/${student.id}`}>Edit</Link>
                                            <button className="btn btn-accent" onClick={() => deleteStudent(student.id)}>Delete</button>
                                        </td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                </div>
            )}
        </Dashboard>
    )
}

export default Edit;
