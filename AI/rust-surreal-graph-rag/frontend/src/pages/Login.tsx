import React, { useState } from 'react';
import { Stack, TextField, PrimaryButton } from '@fluentui/react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '@/store/auth';

const Login: React.FC = () => {
  // 로그인 폼 상태
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const navigate = useNavigate();
  const { login } = useAuth();

  const onSubmit = async () => {
    setLoading(true);
    setError(null);
    try {
      await login({ email, password });
      navigate('/chat');
    } catch (e: any) {
      setError(e?.message ?? '로그인에 실패했습니다.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Stack tokens={{ childrenGap: 12 }} styles={{ root: { maxWidth: 420, margin: '40px auto' } }}>
      <h2>로그인</h2>
      <TextField label="이메일" value={email} onChange={(_, v) => setEmail(v || '')} />
      <TextField
        label="비밀번호"
        type="password"
        value={password}
        onChange={(_, v) => setPassword(v || '')}
      />
      {error && <div style={{ color: 'crimson' }}>{error}</div>}
      <PrimaryButton onClick={onSubmit} disabled={loading}>
        {loading ? '로그인 중...' : '로그인'}
      </PrimaryButton>
    </Stack>
  );
};

export default Login;
