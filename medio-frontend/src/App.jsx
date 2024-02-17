import React from 'react';
import { BrowserRouter as Router } from 'react-router-dom';
import AuthProvider from './provider/AuthProvider';
import AppRoutes from './routes';

const App = () => {
  return (
    <AuthProvider>
      <Router>
        <AppRoutes />
      </Router>
    </AuthProvider>
  );
};

export default App;