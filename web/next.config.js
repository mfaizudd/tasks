/** @type {import('next').NextConfig} */
const nextConfig = {
    reactStrictMode: true,
    output: "standalone",
    publicRuntimeConfig: {
        client_id: process.env.TASKS_WEB_CLIENT_ID,
        authorize_url: process.env.TASKS_WEB_AUTHORIZE_URL,
        token_url: process.env.TASKS_WEB_TOKEN_URL,
        redirect_uri: process.env.TASKS_WEB_REDIRECT_URI,
        api_url: process.env.TASKS_WEB_API_URL,
    }
}

module.exports = nextConfig
