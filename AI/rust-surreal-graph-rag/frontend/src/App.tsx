import React from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import { ThemeProvider } from '@fluentui/react';
import Home from './pages/Home';
import Login from './pages/Login';
import Chat from './pages/Chat';
import VectorSearch from './pages/VectorSearch';
import Health from './pages/Health';
import NavBar from './components/NavBar';
import AdminReindex from './pages/AdminReindex';
import { AuthProvider, useAuth } from './store/auth';

// 인증이 필요한 라우트 보호 컴포넌트
const PrivateRoute: React.FC<{ children: React.ReactElement }> = ({ children }) => {
  const { isAuthenticated } = useAuth();
  return isAuthenticated ? children : <Navigate to="/login" replace />;
};

const App: React.FC = () => {
  return (
    // 구 Fluent UI와의 호환을 위해 ThemeProvider 유지(필수는 아님)
    <ThemeProvider>
      <AuthProvider>
        <NavBar />
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/login" element={<Login />} />
          <Route
            path="/chat"
            element={
              <PrivateRoute>
                <Chat />
              </PrivateRoute>
            }
          />
          <Route
            path="/search"
            element={
              <PrivateRoute>
                <VectorSearch />
              </PrivateRoute>
            }
          />
          <Route path="/health" element={<Health />} />
          <Route
            path="/reindex"
            element={
              <PrivateRoute>
                <AdminReindex />
              </PrivateRoute>
            }
          />
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </AuthProvider>
    </ThemeProvider>
  );
};

export default App;
