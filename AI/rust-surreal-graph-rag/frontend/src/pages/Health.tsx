import React, { useEffect, useState } from 'react';
import { getHealth } from '@/services/health';

const Health: React.FC = () => {
  // 한국어 주석: 헬스 상태 조회
  const [data, setData] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    getHealth()
      .then(setData)
      .catch((e) => setError(e?.message ?? '헬스 체크 실패'));
  }, []);

  if (error) return <div style={{ padding: 16, color: 'crimson' }}>{error}</div>;
  if (!data) return <div style={{ padding: 16 }}>로딩 중...</div>;

  return (
    <div style={{ padding: 16 }}>
      <h2>시스템 상태</h2>
      <pre style={{ whiteSpace: 'pre-wrap' }}>{JSON.stringify(data, null, 2)}</pre>
    </div>
  );
};

export default Health;
