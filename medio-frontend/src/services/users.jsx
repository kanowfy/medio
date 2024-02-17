import axios from "axios";

const APP_URL = import.meta.env.VITE_APP_BASE_URL || "http://localhost:8080/api/";

const url = APP_URL;

const login = async (username, password) => {
    const response = await axios.post(`${url}login`, { username, password });
    return response.data;
};

const register = async (data) => {
    const response = await axios.post(`${url}register`, data);
    return response.data;
}

const getAllForUser = async (token, authorId) => {
    const response = await axios.get(`${url}author/${authorId}`, {
        headers: { Authorization: `Bearer ${token}`}
    });
    return response.data
}

export default { login, register, getAllForUser };