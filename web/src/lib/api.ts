import axios from "axios";

export function getApi() {
    return axios.create({
        baseURL: "https://api.example.com",
        headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${getToken()}`,
        }
    });
}

export function getToken() {
    return localStorage.getItem("token");
}

export function setToken(token: string) {
    return localStorage.setItem("token", token);
}

export function setRefreshToken(token: string) {
    return localStorage.setItem("refreshToken", token);
}

export function getRefreshToken() {
    return localStorage.getItem("refreshToken");
}

export function removeToken() {
    return localStorage.removeItem("token");
}

export function removeRefreshToken() {
    return localStorage.removeItem("refreshToken");
}
