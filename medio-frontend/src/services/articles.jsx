import axios from "axios";

const APP_URL = import.meta.env.VITE_APP_BASE_URL || "http://localhost:8080/api/";

const url = APP_URL + "articles";

const getAll = async () => {
    const response = await axios.get(url);
    return response.data;
}

const getOne = async (id) => {
    const response = await axios.get(`${url}/${id}`);
    return response.data;
}

const create = async (token, article) => {
    const response = await axios.post(url, article, {
        headers: { Authorization: `Bearer ${token}`},
    });

    return response.data;
}

const update = async (token, article) => {
    const response = await axios.patch(url, article, {
        headers: { Authorization: `Bearer ${token}`}
    });

    return response.data;
}

const remove = async (token, article) => {
    const response = await axios.delete(url, article, {
        headers: { Authorization: `Bearer ${token}`}
    });

    return response.data;
}

export default { getAll, getOne, create, update, remove };