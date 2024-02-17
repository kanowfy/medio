import React from 'react';
import { Routes, Route } from 'react-router-dom';
import Home from './pages/Home';
import Login from './pages/Login';
import NotFound from './pages/NotFound';
import Register from './pages/Register';
import Article from './pages/Article';
import Logout from './pages/Logout';
import Profile from './pages/Profile';
import Publish from './components/Publish';


const AppRoutes = () => {
    return (
        <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/login" element={<Login />} />
            <Route path="/logout" element={<Logout />} />
            <Route path="/register" element={<Register />} />
            <Route path="/profile" element={<Profile />} />
            <Route path="/profile/publish" element={<Publish />} />
            <Route path="/article/:articleId" element={<Article />} />
            <Route path="*" element={<NotFound />} />
        </Routes>
    );
};

export default AppRoutes;