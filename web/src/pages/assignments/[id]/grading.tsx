import Dashboard from "@/components/Dashboard"
import { Loading } from "@/components/Loading";
import { getAuthorizedApi } from "@/lib/api";
import { AssignmentScore, Wrapper } from "@/lib/entities";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";

const Grading = () => {
    const router = useRouter();
    const [students, setStudents] = useState<AssignmentScore[]>([]);
    const [loading, setLoading] = useState<boolean>(false);

    const fetchData = async () => {
        setLoading(true);
        try {
            const assignmentId = router.query.id;
            const api = await getAuthorizedApi();
            const res = await api.get<Wrapper<AssignmentScore[]>>(`/assignments/${assignmentId}/scores`);
            setStudents(res.data.data);
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
                                <th>Assignment</th>
                                <th>Student Name</th>
                                <th>Score</th>
                            </tr>
                        </thead>
                        <tbody>
                            {students.map((student, i) => (
                                <tr key={`${student.assignment_id}${student.student_id}`} className="hover">
                                    <th>{i + 1}</th>
                                    <td>{student.assignment_name}</td>
                                    <td>{student.student_name}</td>
                                    <td>
                                        <input type="number"
                                            min="0"
                                            max="100"
                                            className="input input-bordered w-20"
                                            value={student.score}
                                            onChange={(e) => {
                                                if (Number(e.target.value) <= 100 && Number(e.target.value) >= 0) {
                                                    const newStudents = [...students];
                                                    newStudents[i].score = Number(e.target.value);
                                                    setStudents(newStudents);
                                                }
                                            }}
                                        />
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

export default Grading;
