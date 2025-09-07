import React from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Stack, IStackStyles } from '@fluentui/react';
import { DefaultButton, PrimaryButton } from '@fluentui/react';
import { useAuth } from '@/store/auth';

// 상단 네비게이션 바 컴포넌트
const NavBar: React.FC = () => {
  const { isAuthenticated, logout, me } = useAuth();
  const navigate = useNavigate();

  const onLogout = async () => {
    await logout();
    navigate('/');
  };

  const styles: IStackStyles = {
    root: {
      padding: 12,
      borderBottom: '1px solid #eee',
      marginBottom: 12,
    },
  };

  return (
    <Stack horizontal horizontalAlign="space-between" styles={styles}>
      <Stack horizontal tokens={{ childrenGap: 8 }}>
        <Link to="/">홈</Link>
        <Link to="/index">인덱싱 생성</Link>
        <Link to="/chat">채팅</Link>
        <Link to="/search">벡터검색</Link>
        <Link to="/health">헬스</Link>
      </Stack>
      <Stack horizontal tokens={{ childrenGap: 8 }}>
        {isAuthenticated ? (
          <>
            <div style={{ alignSelf: 'center', color: '#555' }}>{me?.email}</div>
            <PrimaryButton onClick={onLogout}>로그아웃</PrimaryButton>
          </>
        ) : (
          <DefaultButton onClick={() => navigate('/login')}>로그인</DefaultButton>
        )}
      </Stack>
    </Stack>
  );
};

export default NavBar;
