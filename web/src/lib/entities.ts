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

export interface Assignment {
    id: string;
    name: string;
    description: string;
    cohort_email: string;
    cohort_name: string;
    cohort_id: string;
    created_at: string;
    updated_at: string;
}

export interface AssignmentScore {
    assignment_id: string;
    assignment_name: string;
    cohort_id: string;
    cohort_name: string;
    student_id: string;
    student_number: string;
    student_name: string;
    score?: number;
    created_at?: string;
    updated_at?: string;
}

export interface Wrapper<T> {
    data: T;
    error: any;
    message: string;
}
