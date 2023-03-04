import Dashboard from "@/components/Dashboard"
import { Loading } from "@/components/Loading";
import { getAuthorizedApi } from "@/lib/api";
import { Assignment, Wrapper } from "@/lib/entities";
import Link from "next/link";
import { useEffect, useState } from "react";

const Assignments = () => {
    const actions = [
        {
            label: "Create assignment",
            href: "/assignments/create",
        }
    ]
    const [assignments, setAssignments] = useState<Assignment[]>([]);
    const [loading, setLoading] = useState(false);

    const fetchAssignments = async () => {
        setLoading(true);
        try {
            const api = await getAuthorizedApi();
            const response = await api.get<Wrapper<Assignment[]>>("/assignments");
            const data = response.data.data;
            setAssignments(data);
        } catch (err) {
            console.log(err);
        } finally {
            setLoading(false);
        }
    }

    const deleteCohort = async (id: string) => {
        if (confirm("Are you sure you wan to delete this assignment?")) {
            try {
                const api = await getAuthorizedApi();
                await api.delete(`/assignments/${id}`)
            } catch (err) {
                console.log(err)
            }
        }
    }

    useEffect(() => {
        fetchAssignments();
    }, [])

    return (
        <Dashboard title="Assignments" actions={actions}>
            {loading ? <Loading /> : (
                <div className="overflow-x-auto">
                    <table className="table w-full">
                        <thead>
                            <tr>
                                <th></th>
                                <th>Name</th>
                                <th>Cohort</th>
                                <th></th>
                            </tr>
                        </thead>
                        <tbody>
                            {assignments.map((assignment, i) => (
                                <tr key={assignment.id} className="hover">
                                    <th>{i + 1}</th>
                                    <td>{assignment.name}</td>
                                    <td>{assignment.cohort_name}</td>
                                    <td className="flex gap-x-2">
                                        <Link className="btn" href={`/assignments/${assignment.id}`}>Edit</Link>
                                        <button className="btn btn-accent" onClick={() => deleteCohort(assignment.id)}>Delete</button>
                                    </td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
            )}
        </Dashboard>
    )
}

export default Assignments;
