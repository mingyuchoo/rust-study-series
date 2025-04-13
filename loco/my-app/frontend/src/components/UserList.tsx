import React, { useEffect, useState } from "react";
import { getUsers, deleteUser, User } from "../services/users";
import { useNavigate } from "react-router-dom";
import "./UserStyles.css";

export const UserList: React.FC = () => {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();

  const fetchUsers = async () => {
    try {
      setLoading(true);
      const data = await getUsers();
      setUsers(data);
      setError(null);
    } catch (err) {
      setError("Failed to fetch users. Please try again.");
      console.error("Error fetching users:", err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchUsers();
  }, []);

  const handleDelete = async (id: string) => {
    if (window.confirm("Are you sure you want to delete this user?")) {
      try {
        await deleteUser(id);
        // Refresh the user list
        fetchUsers();
      } catch (err) {
        setError("Failed to delete user. Please try again.");
        console.error("Error deleting user:", err);
      }
    }
  };

  const handleEdit = (id: string) => {
    navigate(`/users/edit/${id}`);
  };

  const handleView = (id: string) => {
    navigate(`/users/${id}`);
  };

  if (loading) return <div>Loading users...</div>;

  return (
    <div>
      <h2>사용자 목록</h2>
      {error && <div className="error-message">{error}</div>}
      
      <div className="actions">
        <button 
          onClick={() => navigate("/users/new")} 
          className="btn btn-primary"
        >
          새 사용자 추가
        </button>
      </div>

      {users.length === 0 ? (
        <p>No users found.</p>
      ) : (
        <table className="user-table">
          <thead>
            <tr>
              <th>ID</th>
              <th>이름</th>
              <th>이메일</th>
              <th>생성일</th>
              <th>액션</th>
            </tr>
          </thead>
          <tbody>
            {users.map((user) => (
              <tr key={user.pid}>
                <td>{user.pid}</td>
                <td>{user.name}</td>
                <td>{user.email}</td>
                <td>{new Date(user.created_at).toLocaleDateString()}</td>
                <td className="actions">
                  <button 
                    onClick={() => handleView(user.pid)} 
                    className="btn btn-secondary"
                  >
                    조회
                  </button>
                  <button 
                    onClick={() => handleEdit(user.pid)} 
                    className="btn btn-primary"
                  >
                    수정
                  </button>
                  <button 
                    onClick={() => handleDelete(user.pid)} 
                    className="btn btn-danger"
                  >
                    삭제
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
};
