import React from "react";
import { BrowserRouter as Router, Route, Routes, Navigate } from "react-router-dom";
import { AuthProvider, useAuth } from "./context/AuthContext";
import { Login } from "./components/Login";
import { Home } from "./components/Home";
import { UserDetail } from "./components/UserDetail";
import { UserForm } from "./components/UserForm";

const ProtectedRoute: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const { isAuthenticated } = useAuth();
  return isAuthenticated ? <>{children}</> : <Navigate to="/login" replace />;
};

export const App: React.FC = () => {
  return (
    <Router>
      <AuthProvider>
        <Routes>
          <Route path="/login" element={<Login />} />
          <Route
            path="/"
            element={
              <ProtectedRoute>
                <Home />
              </ProtectedRoute>
            }
          />
          <Route
            path="/users/:id"
            element={
              <ProtectedRoute>
                <UserDetail />
              </ProtectedRoute>
            }
          />
          <Route
            path="/users/new"
            element={
              <ProtectedRoute>
                <UserForm />
              </ProtectedRoute>
            }
          />
          <Route
            path="/users/edit/:id"
            element={
              <ProtectedRoute>
                <UserForm />
              </ProtectedRoute>
            }
          />
          <Route path="*" element={<Navigate to="/login" replace />} />
        </Routes>
      </AuthProvider>
    </Router>
  );
};
