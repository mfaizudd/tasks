export interface Cohort {
    id: string;
    name: string;
    email: string;
    created_at: string;
    updated_at: string;
}

export interface Wrapper<T> {
    data: T;
    error: any;
    message: string;
}
