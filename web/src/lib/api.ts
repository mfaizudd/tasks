import axios from "axios";

export function getApi() {
    return axios.create({
        baseURL: process.env.NEXT_PUBLIC_API_URL,
        headers: {
            "Content-Type": "application/json",
        }
    });
}

export function getAuthorizedApi() {
    return axios.create({
        baseURL: process.env.NEXT_PUBLIC_API_URL,
        headers: {
            "Content-Type": "application/json",
            "Authorization": `Bearer ${getToken()}`
        }
    });
}

export function getToken() {
    return localStorage.getItem("access_token");
}

export function setToken(token: string) {
    return localStorage.setItem("access_token", token);
}

export function setRefreshToken(token: string) {
    return localStorage.setItem("refresh_token", token);
}

export function getRefreshToken() {
    return localStorage.getItem("refresh_token");
}

export function removeToken() {
    return localStorage.removeItem("access_token");
}

export function removeRefreshToken() {
    return localStorage.removeItem("refresh_token");
}
