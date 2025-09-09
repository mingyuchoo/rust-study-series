import React, { useState } from 'react';
import { Stack, TextField, PrimaryButton } from '@fluentui/react';
import { vectorSearch } from '@/services/search';
import { VectorSearchResponse } from '@/types/api';

const VectorSearch: React.FC = () => {
  // 벡터 검색 상태
  const [query, setQuery] = useState('');
  const [topK, setTopK] = useState<number | undefined>(5);
  const [threshold, setThreshold] = useState<number | undefined>(0.2);
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<VectorSearchResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  const onSearch = async () => {
    if (!query.trim()) return;
    setLoading(true);
    setError(null);
    try {
      const res = await vectorSearch({ query, top_k: topK, threshold });
      setResult(res);
    } catch (e: any) {
      setError(e?.message ?? '요청 중 오류가 발생했습니다.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ padding: 16 }}>
      <Stack tokens={{ childrenGap: 12 }}>
        <h2>벡터 검색</h2>
        <TextField label="쿼리" value={query} onChange={(_, v) => setQuery(v || '')} />
        <TextField
          label="top_k"
          type="number"
          value={String(topK ?? '')}
          onChange={(_, v) => setTopK(v ? Number(v) : undefined)}
        />
        <TextField
          label="threshold"
          type="number"
          value={String(threshold ?? '')}
          onChange={(_, v) => setThreshold(v ? Number(v) : undefined)}
        />
        <PrimaryButton onClick={onSearch} disabled={loading}>
          {loading ? '검색 중...' : '검색'}
        </PrimaryButton>

        {error && <div style={{ color: 'crimson' }}>{error}</div>}

        {result && (
          <div style={{ marginTop: 16 }}>
            <div>
              총 {result.total}건, 소요 시간: {result.query_time}s
            </div>
            <ul>
              {result.results.map((r) => (
                <li key={r.id}>
                  <strong>{r.id}</strong> - 점수: {r.score}
                  <div>{r.content}</div>
                </li>
              ))}
            </ul>
          </div>
        )}
      </Stack>
    </div>
  );
};

export default VectorSearch;
