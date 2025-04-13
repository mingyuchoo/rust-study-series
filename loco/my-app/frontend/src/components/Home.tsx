import React from "react";
import { useAuth } from "../context/AuthContext";
import { UserList } from "./UserList";
import "./UserStyles.css";

export const Home: React.FC = () => {
  const { user, logout } = useAuth();

  return (
    <div className="user-container">
      <div className="header">
        <h2>환영합니다, {user?.email || "사용자"}님!</h2>
        <button
          onClick={logout}
          className="btn btn-danger"
        >
          로그아웃
        </button>
      </div>
      
      <UserList />
    </div>
  );
};
