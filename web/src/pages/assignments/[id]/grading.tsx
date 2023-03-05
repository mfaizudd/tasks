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

    const saveScore = async (score: AssignmentScore) => {
        try {
            const api = await getAuthorizedApi();
            await api.put(`/scores`, {
                assignment_id: score.assignment_id,
                student_id: score.student_id,
                score: score.score
            })
        } catch (err) {
            console.log(err);
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
                                <th>Cohort</th>
                                <th>Number</th>
                                <th>Student Name</th>
                                <th>Score</th>
                            </tr>
                        </thead>
                        <tbody>
                            {students.map((student, i) => (
                                <tr key={`${student.assignment_id}${student.student_id}`} className="hover">
                                    <th>{i + 1}</th>
                                    <td>{student.cohort_name}</td>
                                    <td>{student.student_number}</td>
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
                                        <button 
                                            className="btn btn-sm btn-primary ml-2"
                                            onClick={() => saveScore(student)}>
                                            Save
                                        </button>
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
