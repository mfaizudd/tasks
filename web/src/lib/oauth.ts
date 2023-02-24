import sha256 from "crypto-js/sha256";
import Base64 from "crypto-js/enc-base64";
import axios from "axios";

export interface Claims {
    iss: string,
    sub: string
    aud: string,
    exp: number,
    iat: number,
    acr: string,
}

export interface TokenResponse {
    access_token: string;
    expires_in: number;
    id_token?: string;
    refresh_token: string;
}

function randomstring(length: number) {
    let result = '';
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    const charactersLength = characters.length;
    let counter = 0;
    while (counter < length) {
      result += characters.charAt(Math.floor(Math.random() * charactersLength));
      counter += 1;
    }
    return result;
}

export function oauthSignIn() {
    const authUrl = process.env.NEXT_PUBLIC_AUTHORIZE_URL;
    const clientId = process.env.NEXT_PUBLIC_CLIENT_ID;
    const redirectUri = process.env.NEXT_PUBLIC_REDIRECT_URI;
    const scope = "openid+profile+email";
    const code_verifier = randomstring(128);
    setCodeVerifier(code_verifier);
    const code_challenge = Base64.stringify(sha256(code_verifier)).replace(/=/g, "").replace(/\+/g, "-").replace(/\//g, "_");
    const state = randomstring(32);
    const url = `${authUrl}?client_id=${clientId}&redirect_uri=${redirectUri}&scope=${scope}&response_type=code&code_challenge=${code_challenge}&code_challenge_method=S256&state=${state}`;
    window.open(url, "_self");
}

export function setCodeVerifier(code_verifier: string) {
    localStorage.setItem("code_verifier", code_verifier);
}

export function getCodeVerifier() {
    return localStorage.getItem("code_verifier");
}

export function removeCodeVerifier() {
    localStorage.removeItem("code_verifier");
}

export async function exchangeCodeForToken(code: string, code_verifier: string) {
    const tokenUrl = process.env.NEXT_PUBLIC_TOKEN_URL ?? "";
    const clientId = process.env.NEXT_PUBLIC_CLIENT_ID;
    const redirectUri = process.env.NEXT_PUBLIC_REDIRECT_URI;
    const body = `grant_type=authorization_code&code=${code}&code_verifier=${code_verifier}&redirect_uri=${redirectUri}&client_id=${clientId}`;
    const response = await axios.post<TokenResponse>(tokenUrl, body, {
        headers: {
            "Content-Type": "application/x-www-form-urlencoded",
        },
    });
    console.log(response.data);
    removeCodeVerifier();
    return response.data;
}

export function refreshToken(refresh_token: string|null) {
    const tokenUrl = process.env.NEXT_PUBLIC_TOKEN_URL ?? "";
    const clientId = process.env.NEXT_PUBLIC_CLIENT_ID;
    const body = {
        grant_type: "refresh_token",
        refresh_token: refresh_token,
        client_id: clientId,
    }
    return axios.post<TokenResponse>(tokenUrl, body, {
        headers: {
            "Content-Type": "application/x-www-form-urlencoded",
        },
    });
}
