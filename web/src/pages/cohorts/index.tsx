import Dashboard from "@/components/Dashboard";
import { getAuthorizedApi } from "@/lib/api";
import { Cohort, Wrapper } from "@/lib/entities";
import { NextPage } from "next";
import Link from "next/link";
import React, { useEffect, useState } from "react";

const CohortIndex: NextPage = () => {
    const actions = [
        {
            label: "Create Cohort",
            href: "/cohorts/create",
        }
    ]
    const [cohorts, setCohorts] = useState<any[]>(() => []);

    const fetchCohorts = async () => {
        try {
            const api = await getAuthorizedApi();
            const response = await api.get<Wrapper<Cohort[]>>("/cohorts");
            const data = response.data.data;
            setCohorts(data)
        } catch (err) {
            console.log(err)
        }
    }

    const deleteCohort = async (id: number) => {
        if (confirm("Are you sure you want to delete this cohort?")) {
            try {
                const api = await getAuthorizedApi();
                await api.delete(`/cohorts/${id}`);
                fetchCohorts();
            } catch (err) {
                console.log(err)
            }
        }
    }

    useEffect(() => {
        fetchCohorts();
    }, [])
    return (
        <Dashboard title="Cohorts" actions={actions}>
            <div className="overflow-x-auto">
                <table className="table w-full">
                    <thead>
                        <tr>
                            <th></th>
                            <th>Name</th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                        {cohorts.map((cohort, i) => (
                            <tr key={cohort.id} className="hover">
                                <th>{i + 1}</th>
                                <td>{cohort.name}</td>
                                <td className="flex gap-x-2">
                                    <Link className="btn" href={`/cohorts/${cohort.id}`}>Edit</Link>
                                    <button className="btn btn-accent" onClick={() => deleteCohort(cohort.id)}>Delete</button>
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </Dashboard>
    )
}

export default CohortIndex;
