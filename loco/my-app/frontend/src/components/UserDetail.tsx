import React, { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { getUserById, User } from "../services/users";
import "./UserStyles.css";

export const UserDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();

  useEffect(() => {
    const fetchUser = async () => {
      if (!id) return;
      
      try {
        setLoading(true);
        const data = await getUserById(id);
        setUser(data);
        setError(null);
      } catch (err) {
        setError("Failed to fetch user details. Please try again.");
        console.error("Error fetching user:", err);
      } finally {
        setLoading(false);
      }
    };

    fetchUser();
  }, [id]);

  if (loading) return <div>Loading user details...</div>;
  if (error) return <div className="error-message">{error}</div>;
  if (!user) return <div>User not found</div>;

  return (
    <div className="user-container">
      <h2>사용자 상세정보</h2>
      
      <div className="user-info">
        <p><strong>ID:</strong> {user.pid}</p>
        <p><strong>이름:</strong> {user.name}</p>
        <p><strong>이메일:</strong> {user.email}</p>
        <p><strong>생성일:</strong> {new Date(user.created_at).toLocaleString()}</p>
        <p><strong>업데이트일:</strong> {new Date(user.updated_at).toLocaleString()}</p>
      </div>
      
      <div className="form-actions">
        <button 
          onClick={() => navigate(`/users/edit/${user.pid}`)} 
          className="btn btn-primary"
        >
          수정
        </button>
        <button 
          onClick={() => navigate("/")} 
          className="btn btn-secondary"
        >
          목록으로 돌아가기
        </button>
      </div>
    </div>
  );
};
