import React, { createContext, useContext, useReducer } from 'react';
import axios from 'axios';

const AuthContext = createContext();

const ACTIONS = {
    LOGIN: 'login',
    LOGOUT: 'logout',
};

const authReducer = (state, action) => {
    switch (action.type) {
        case ACTIONS.LOGIN:
            // Set the authentication token in axios headers and local storage
            axios.defaults.headers.common['Authorization'] = `Bearer ${action.payload.token}`;
            localStorage.setItem('token', action.payload.token);
            localStorage.setItem('userid', action.payload.user.id);
            return { ...state, token: action.payload.token, userid: action.payload.user.id };
        case ACTIONS.LOGOUT:
            // Clear the authentication token from axios headers and local storage
            delete axios.defaults.headers.common['Authorization'];
            localStorage.removeItem('token');
            localStorage.removeItem('userid');
            return { ...state, token: null, userid: null };
        default:
            throw new Error(`Unhandled action type: ${action.type}`);
    }
};

const AuthProvider = ({ children }) => {
    const [state, dispatch] = useReducer(authReducer, {
        token: localStorage.getItem('token'),
        userid: localStorage.getItem('userid')
    });

    const login = (response) => dispatch({
        type: ACTIONS.LOGIN,
        payload: response
    });
    const logout = () => dispatch({ type: ACTIONS.LOGOUT });

    const contextValue = { ...state, login, logout };

    return (
        <AuthContext.Provider value={contextValue}>
            {children}
        </AuthContext.Provider>
    );
};

export const useAuth = () => useContext(AuthContext);
export default AuthProvider;