import React from "react";
import { useAuth } from "../context/AuthContext";

export const Home: React.FC = () => {
  const { user, logout } = useAuth();

  return (
    <div className="home-container" style={{ maxWidth: "500px", margin: "auto", padding: "20px" }}>
      <h2>Welcome, {user?.email || "User"}!</h2>
      <p>You have successfully logged in.</p>
      <button
        onClick={logout}
        style={{ padding: "10px", backgroundColor: "#dc3545", color: "white", border: "none", borderRadius: "4px", cursor: "pointer" }}
      >
        Logout
      </button>
    </div>
  );
};
