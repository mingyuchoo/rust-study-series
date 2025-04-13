import axios from "axios";
import { useAuth } from "../context/AuthContext";

const API_URL = "http://localhost:5150/api/users";

export interface User {
  pid: string;
  email: string;
  name: string;
  created_at: string;
  updated_at: string;
}

export interface UserCreate {
  email: string;
  name: string;
  password: string;
}

export interface UserUpdate {
  email?: string;
  name?: string;
  password?: string;
}

// Helper function to get auth token
const getAuthHeader = () => {
  const token = localStorage.getItem("token");
  return {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  };
};

// Get all users
export const getUsers = async (): Promise<User[]> => {
  const response = await axios.get(API_URL, getAuthHeader());
  return response.data;
};

// Get user by ID
export const getUserById = async (id: string): Promise<User> => {
  const response = await axios.get(`${API_URL}/${id}`, getAuthHeader());
  return response.data;
};

// Create new user
export const createUser = async (user: UserCreate): Promise<User> => {
  const response = await axios.post(API_URL, user, getAuthHeader());
  return response.data;
};

// Update user
export const updateUser = async (id: string, user: UserUpdate): Promise<User> => {
  const response = await axios.put(`${API_URL}/${id}`, user, getAuthHeader());
  return response.data;
};

// Delete user
export const deleteUser = async (id: string): Promise<void> => {
  await axios.delete(`${API_URL}/${id}`, getAuthHeader());
};
