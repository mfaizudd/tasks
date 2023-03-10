import Dashboard from "@/components/Dashboard"
import { Loading } from "@/components/Loading";
import { getAuthorizedApi } from "@/lib/api";
import { Assignment, AssignmentScore, Wrapper } from "@/lib/entities";
import { useRouter } from "next/router";
import { useCallback, useEffect, useState } from "react";

const Grading = () => {
    const router = useRouter();
    const [students, setStudents] = useState<AssignmentScore[]>([]);
    const [assignment, setAssignment] = useState<Assignment | null>(null);
    const [loading, setLoading] = useState<boolean>(false);
    const [saving, setSaving] = useState<boolean>(false);

    const fetchData = useCallback(async () => {
        setLoading(true);
        try {
            const assignmentId = router.query.id;
            const api = await getAuthorizedApi();
            const assignmentRes = await api.get<Wrapper<Assignment>>(`/assignments/${assignmentId}`);
            setAssignment(assignmentRes.data.data);
            const res = await api.get<Wrapper<AssignmentScore[]>>(`/assignments/${assignmentId}/scores`);
            setStudents(res.data.data);
        } catch (err) {
            console.log(err);
        } finally {
            setLoading(false);
        }
    }, [router.query.id]);

    const saveScore = async (score: AssignmentScore) => {
        setSaving(true);
        try {
            const api = await getAuthorizedApi();
            await api.put(`/scores`, {
                assignment_id: score.assignment_id,
                student_id: score.student_id,
                score: score.score
            })
        } catch (err) {
            console.log(err);
        } finally {
            setSaving(false);
        }
    }

    useEffect(() => {
        fetchData();
    }, [fetchData])

    return (
        <Dashboard title={`Scores for assignment: ${assignment?.name}`}>
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
                                            className={`btn btn-sm btn-primary ml-2 ${saving ? 'loading' : ''}`}
                                            onClick={() => saveScore(student)} disabled={saving}>
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
