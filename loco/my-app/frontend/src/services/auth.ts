import axios from "axios";

// Assuming your API is running on localhost:5150, adjust if different
const API_URL = "http://localhost:5150/api/auth";

interface LoginResponse {
  user: {
    email: string;
    pid: string;
  };
  token: string;
}

export const login = async (email: string, password: string): Promise<LoginResponse> => {
  const response = await axios.post(`${API_URL}/login`, { email, password });
  if (response.data) {
    return response.data;
  }
  throw new Error("Login failed");
};
