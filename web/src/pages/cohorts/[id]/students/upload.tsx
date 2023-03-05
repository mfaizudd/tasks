import Dashboard from "@/components/Dashboard";
import { Loading } from "@/components/Loading";
import { getAuthorizedApi } from "@/lib/api";
import { Cohort, Wrapper } from "@/lib/entities";
import { GetServerSideProps, NextPage } from "next";
import { useRouter } from "next/router";
import { ChangeEvent, useEffect, useState } from "react";

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
            id: id as string,
        }
    }
}

export const Add: NextPage<Props> = ({ id }) => {
    const router = useRouter();
    const [cohort, setCohort] = useState<Cohort | null>(null);
    const [file, setFile] = useState<File>();
    const [loading, setLoading] = useState(false);
    const [saving, setSaving] = useState(false);

    const fetchData = async () => {
        setLoading(true);
        try {
            const api = await getAuthorizedApi();
            const res = await api.get<Wrapper<Cohort>>(`/cohorts/${id}`);
            if (res.status === 200) {
                setCohort(res.data.data);
            }
        } catch (err) {
            console.log(err);
        } finally {
            setLoading(false);
        }
    }

    const submit = async () => {
        setSaving(true);
        try {
            const api = await getAuthorizedApi();
            const formData = new FormData();
            formData.append('students', file!);
            const res = await api.post(`/cohorts/${id}/students/upload`, formData, {
                headers: {
                    'Content-Type': 'multipart/form-data',
                }
            });
            if (res.status === 201) {
                router.push(`/cohorts/${id}`)
            }
        } catch (err) {
            console.log(err);
        } finally {
            setSaving(false);
        }
    }

    const onFileChange = async (e: ChangeEvent<HTMLInputElement>) => {
        const file = e.target.files?.[0];
        if (!file) {
            return;
        }
        setFile(file);
    }

    useEffect(() => {
        fetchData();
    }, []);

    if (loading) {
        return <Loading />;
    }

    return (
        <Dashboard title={`Uploading students to ${cohort?.name}`}>
            <div className="p-5">
                <form onSubmit={e => { e.preventDefault(); !saving && submit() }}>
                    <div className="flex gap-3">
                        <input type="file" onChange={onFileChange} placeholder="Student name" className="file-input file-input-bordered w-full max-w-xs" />
                        <button className="btn btn-primary" type="submit" disabled={saving}>Submit</button>
                    </div>
                </form>
            </div>
        </Dashboard>
    )
}

export default Add;
