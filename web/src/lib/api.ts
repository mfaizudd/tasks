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

export function removeToken() {
    return localStorage.removeItem("token");
}
