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
        const api = await getAuthorizedApi();
        const response = await api.get<Wrapper<Cohort[]>>("/cohorts");
        const data = response.data.data;
        setCohorts(data)
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
                                <td>
                                    <Link className="btn btn-primary" href={`/cohorts/${cohort.id}`}>Edit</Link>
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
