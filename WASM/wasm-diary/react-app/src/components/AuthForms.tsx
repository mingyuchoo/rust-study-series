import { useState } from "react";
import type { LoginResult, RegistrationValidation } from "../types/diary";

interface LoginFormProps {
  onLogin: (username: string, password: string) => LoginResult;
  onSwitchToRegister: () => void;
}

export function LoginForm({ onLogin, onSwitchToRegister }: LoginFormProps) {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    const result = onLogin(username, password);
    if (!result.success) {
      setError(result.error ?? "로그인 실패");
    }
  };

  return (
    <div className="auth-form">
      <h2>로그인</h2>
      <form onSubmit={handleSubmit}>
        <div className="form-field">
          <label>사용자 ID</label>
          <input
            type="text"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            placeholder="영문, 숫자, 언더스코어(_)"
            autoComplete="username"
          />
        </div>
        <div className="form-field">
          <label>비밀번호</label>
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            placeholder="비밀번호"
            autoComplete="current-password"
          />
        </div>
        {error && <span className="field-error">{error}</span>}
        <div className="form-actions">
          <button type="submit" className="btn btn-primary">
            로그인
          </button>
          <button
            type="button"
            className="btn btn-secondary"
            onClick={onSwitchToRegister}
          >
            회원가입
          </button>
        </div>
      </form>
      <p className="auth-hint">기본 관리자: admin / admin123</p>
    </div>
  );
}

interface RegisterFormProps {
  onRegister: (
    username: string,
    password: string,
    nickname?: string
  ) => LoginResult;
  onValidate: (
    username: string,
    password: string
  ) => RegistrationValidation;
  onSwitchToLogin: () => void;
}

export function RegisterForm({
  onRegister,
  onValidate,
  onSwitchToLogin,
}: RegisterFormProps) {
  const [username, setUsername] = useState("");
  const [nickname, setNickname] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [userIdError, setUserIdError] = useState("");
  const [passwordError, setPasswordError] = useState("");
  const [generalError, setGeneralError] = useState("");

  const handleUsernameChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const filtered = e.target.value.replace(/[^a-zA-Z0-9_]/g, "");
    setUsername(filtered);
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setUserIdError("");
    setPasswordError("");
    setGeneralError("");

    if (password !== confirmPassword) {
      setPasswordError("비밀번호가 일치하지 않습니다");
      return;
    }

    const validation = onValidate(username, password);
    if (!validation.valid) {
      if (validation.username_error) setUserIdError(validation.username_error);
      if (validation.password_error) setPasswordError(validation.password_error);
      return;
    }

    const trimmedNickname = nickname.trim();
    const result = onRegister(
      username,
      password,
      trimmedNickname || undefined
    );
    if (!result.success) {
      setGeneralError(result.error ?? "회원가입 실패");
    }
  };

  return (
    <div className="auth-form">
      <h2>회원가입</h2>
      <form onSubmit={handleSubmit}>
        <div className="form-field">
          <label>사용자 ID</label>
          <input
            type="text"
            value={username}
            onChange={handleUsernameChange}
            placeholder="영문, 숫자, 언더스코어 (3자 이상)"
            autoComplete="username"
          />
          {userIdError && (
            <span className="field-error">{userIdError}</span>
          )}
        </div>
        <div className="form-field">
          <label>닉네임 (선택)</label>
          <input
            type="text"
            value={nickname}
            onChange={(e) => setNickname(e.target.value)}
            placeholder="표시될 이름 (미입력 시 ID 사용)"
          />
        </div>
        <div className="form-field">
          <label>비밀번호</label>
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            placeholder="6자 이상"
            autoComplete="new-password"
          />
        </div>
        <div className="form-field">
          <label>비밀번호 확인</label>
          <input
            type="password"
            value={confirmPassword}
            onChange={(e) => setConfirmPassword(e.target.value)}
            placeholder="비밀번호 재입력"
            autoComplete="new-password"
          />
          {passwordError && (
            <span className="field-error">{passwordError}</span>
          )}
        </div>
        {generalError && <span className="field-error">{generalError}</span>}
        <div className="form-actions">
          <button type="submit" className="btn btn-primary">
            회원가입
          </button>
          <button
            type="button"
            className="btn btn-secondary"
            onClick={onSwitchToLogin}
          >
            로그인으로 돌아가기
          </button>
        </div>
      </form>
    </div>
  );
}
