export interface Cohort {
    id: string;
    name: string;
    email: string;
    created_at: string;
    updated_at: string;
}

export interface Student {
    id: string;
    number: string;
    name: string;
    cohort_id: string;
    created_at: string;
    updated_at: string;
}

export interface Wrapper<T> {
    data: T;
    error: any;
    message: string;
}