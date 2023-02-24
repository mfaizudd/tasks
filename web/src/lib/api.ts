import axios from "axios";
import jwt_decode from "jwt-decode";
import { Claims, refreshToken } from "./oauth";

export function getApi() {
    return axios.create({
        baseURL: process.env.NEXT_PUBLIC_API_URL,
        headers: {
            "Content-Type": "application/json",
        }
    });
}

export async function getAuthorizedApi() {
    let token = getToken();
    try {
        const claims = jwt_decode<Claims>(token ?? "");
        if (claims.exp * 1000 < Date.now()) {
            const refresh_token = getRefreshToken();
            const response = await refreshToken(refresh_token);
            console.log(response.data);
            token = response.data.access_token;
            setToken(token);
            setRefreshToken(response.data.refresh_token);
        }
    } catch (error) {
        console.log(error);
    }
    return axios.create({
        baseURL: process.env.NEXT_PUBLIC_API_URL,
        headers: {
            "Content-Type": "application/json",
            "Authorization": `Bearer ${token}`
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
