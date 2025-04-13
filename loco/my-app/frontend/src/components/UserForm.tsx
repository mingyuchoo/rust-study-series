import React, { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { getUserById, createUser, updateUser, User, UserCreate, UserUpdate } from "../services/users";
import "./UserStyles.css";

interface UserFormData {
  name: string;
  email: string;
  password?: string;
}

export const UserForm: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const isEditMode = !!id;
  const navigate = useNavigate();
  
  const [formData, setFormData] = useState<UserFormData>({
    name: "",
    email: "",
    password: "",
  });
  const [loading, setLoading] = useState(isEditMode);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchUser = async () => {
      if (!isEditMode) return;
      
      try {
        setLoading(true);
        const user = await getUserById(id);
        setFormData({
          name: user.name,
          email: user.email,
          // Password is not included when fetching a user
          password: "",
        });
        setError(null);
      } catch (err) {
        setError("Failed to fetch user data. Please try again.");
        console.error("Error fetching user:", err);
      } finally {
        setLoading(false);
      }
    };

    fetchUser();
  }, [id, isEditMode]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setSubmitting(true);
    setError(null);

    try {
      if (isEditMode) {
        // For edit mode, only send password if it's provided
        const updateData: UserUpdate = {
          name: formData.name,
          email: formData.email,
        };
        
        if (formData.password && formData.password.trim() !== "") {
          updateData.password = formData.password;
        }
        
        await updateUser(id, updateData);
      } else {
        // For create mode, password is required
        if (!formData.password) {
          setError("Password is required");
          setSubmitting(false);
          return;
        }
        
        const newUser: UserCreate = {
          name: formData.name,
          email: formData.email,
          password: formData.password,
        };
        await createUser(newUser);
      }
      
      // Navigate back to the user list on success
      navigate("/");
    } catch (err: any) {
      setError(err.response?.data?.message || "An error occurred. Please try again.");
      console.error("Error submitting form:", err);
    } finally {
      setSubmitting(false);
    }
  };

  if (loading) return <div>Loading...</div>;

  return (
    <div className="user-container">
      <h2>{isEditMode ? "사용자 수정" : "새 사용자 추가"}</h2>
      
      {error && <div className="error-message">{error}</div>}
      
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="name">이름:</label>
          <input
            type="text"
            id="name"
            name="name"
            value={formData.name}
            onChange={handleChange}
            required
          />
        </div>
        
        <div className="form-group">
          <label htmlFor="email">이메일:</label>
          <input
            type="email"
            id="email"
            name="email"
            value={formData.email}
            onChange={handleChange}
            required
          />
        </div>
        
        <div className="form-group">
          <label htmlFor="password">
            {isEditMode ? "비밀번호 (변경하려면 입력)" : "비밀번호:"}
          </label>
          <input
            type="password"
            id="password"
            name="password"
            value={formData.password}
            onChange={handleChange}
            required={!isEditMode}
          />
        </div>
        
        <div className="form-actions">
          <button 
            type="submit" 
            disabled={submitting}
            className="btn btn-success"
          >
            {submitting ? "저장 중..." : "저장"}
          </button>
          <button 
            type="button" 
            onClick={() => navigate("/")}
            className="btn btn-secondary"
          >
            취소
          </button>
        </div>
      </form>
    </div>
  );
};
