import React from 'react';
import { Text } from '@fluentui/react';

const Home: React.FC = () => {
  return (
    <div style={{ padding: 16 }}>
      {/* 홈 화면 소개 */}
      <Text variant="xLarge">RAG AI 웹 서비스</Text>
      <p>좌측 상단 메뉴를 이용해 로그인, 채팅, 벡터 검색, 헬스 상태를 확인할 수 있습니다.</p>
    </div>
  );
};

export default Home;
