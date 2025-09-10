import React, { useState } from 'react';
import { Stack, TextField, PrimaryButton } from '@fluentui/react';
import { graphSearch } from '@/services/graphSearch';
import { GraphSearchResponse } from '@/types/api';

const GraphSearch: React.FC = () => {
  const [query, setQuery] = useState('');
  const [topK, setTopK] = useState<number | undefined>(5);
  const [maxHops, setMaxHops] = useState<number | undefined>(2);
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<GraphSearchResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  const onSearch = async () => {
    if (!query.trim()) return;
    setLoading(true);
    setError(null);
    try {
      const res = await graphSearch({ query, top_k: topK, max_hops: maxHops });
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
        <h2>그래프 검색</h2>
        <TextField label="쿼리" value={query} onChange={(_, v) => setQuery(v || '')} />
        <TextField
          label="top_k"
          type="number"
          value={String(topK ?? '')}
          onChange={(_, v) => setTopK(v ? Number(v) : undefined)}
        />
        <TextField
          label="max_hops"
          type="number"
          value={String(maxHops ?? '')}
          onChange={(_, v) => setMaxHops(v ? Number(v) : undefined)}
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
            {result.paths.length === 0 ? (
              <div>경로 결과가 없습니다.</div>
            ) : (
              <ul>
                {result.paths.map((p, idx) => (
                  <li key={idx}>
                    <div><strong>경로:</strong> {p.path}</div>
                    <pre style={{ whiteSpace: 'pre-wrap' }}>{JSON.stringify({ nodes: p.nodes, relationships: p.relationships }, null, 2)}</pre>
                  </li>
                ))}
              </ul>
            )}
          </div>
        )}
      </Stack>
    </div>
  );
};

export default GraphSearch;
