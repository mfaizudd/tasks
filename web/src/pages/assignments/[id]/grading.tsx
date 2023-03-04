import Dashboard from "@/components/Dashboard"
import { Loading } from "@/components/Loading";
import { getAuthorizedApi } from "@/lib/api";
import { Assignment, Student, Wrapper } from "@/lib/entities";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";

const Grading = () => {
    const router = useRouter();
    const [students, setStudents] = useState<Student[]>([]);
    const [loading, setLoading] = useState<boolean>(false);

    const fetchData = async () => {
        setLoading(true);
        try {
            const assignmentId = router.query.id;
            const api = await getAuthorizedApi();
            const assignmentRes = await api.get<Wrapper<Assignment>>(`/assignments/${assignmentId}`);
            const assignment = assignmentRes.data.data;
            const studentsRes = await api.get<Wrapper<Student[]>>(`/cohorts/${assignment.cohort_id}/students`);
            const students = studentsRes.data.data;
            setStudents(students);
        } catch (err) {
            console.log(err);
        } finally {
            setLoading(false);
        }
    }

    useEffect(() => {
        fetchData();
    }, [])

    return (
        <Dashboard>
            {loading ? <Loading /> : (
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
            )}
        </Dashboard>
    )
}

export default Grading;
