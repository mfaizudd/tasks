import { User } from "@/context/state";
import axios from "axios";
import jwt_decode from "jwt-decode";
import getConfig from "next/config";
import { Claims, refreshToken } from "./oauth";

export function getApi() {
    const { publicRuntimeConfig } = getConfig();
    return axios.create({
        baseURL: publicRuntimeConfig.api_url,
        headers: {
            "Content-Type": "application/json",
        }
    });
}

export async function getAuthorizedApi() {
    const { publicRuntimeConfig } = getConfig();
    let token = getToken();
    try {
        const claims = jwt_decode<Claims>(token ?? "");
        if (claims.exp * 1000 < Date.now()) {
            const refresh_token = getRefreshToken();
            const response = await refreshToken(refresh_token);
            token = response.data.access_token;
            setToken(token);
            setRefreshToken(response.data.refresh_token);
        }
    } catch (error) {
        console.log(error);
    }
    return axios.create({
        baseURL: publicRuntimeConfig.api_url,
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

export function setIdToken(token: string) {
    return localStorage.setItem("id_token", token);
}

export function getUserInfo() {
    const token = localStorage.getItem("id_token");
    if (token) {
        return jwt_decode<User>(token);
    }
    return null;
}

export function getRefreshToken() {
    return localStorage.getItem("refresh_token");
}

export function removeToken() {
    localStorage.removeItem("access_token");
    localStorage.removeItem("id_token")
}

export function removeRefreshToken() {
    return localStorage.removeItem("refresh_token");
}
