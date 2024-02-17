import axios from "axios";

const APP_URL = import.meta.env.VITE_APP_BASE_URL || "http://localhost:8080/api/";

const url = APP_URL + "subscribe";

const subscribe = async (email) => {
    const response = await axios.post(url, { email });
    return response.data;
}

export default { subscribe }