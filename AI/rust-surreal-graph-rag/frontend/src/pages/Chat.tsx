import React, { useState } from 'react';
import { Stack, TextField, PrimaryButton } from '@fluentui/react';
import { chatAsk } from '@/services/chat';
import { ChatAskResponse } from '@/types/api';

const Chat: React.FC = () => {
  // 채팅 질의 상태
  const [query, setQuery] = useState('');
  const [conversationId, setConversationId] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<ChatAskResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  const onAsk = async () => {
    if (!query.trim()) return;
    setLoading(true);
    setError(null);
    try {
      const res = await chatAsk({ query, conversation_id: conversationId });
      setResult(res);
      setConversationId(res.conversation_id ?? conversationId);
    } catch (e: any) {
      setError(e?.message ?? '요청 중 오류가 발생했습니다.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ padding: 16 }}>
      <Stack tokens={{ childrenGap: 12 }}>
        <h2>통합 질의응답</h2>
        <TextField
          label="질문"
          value={query}
          onChange={(_, v) => setQuery(v || '')}
          multiline
          rows={3}
        />
        <PrimaryButton onClick={onAsk} disabled={loading}>
          {loading ? '요청 중...' : '질의'}
        </PrimaryButton>

        {error && <div style={{ color: 'crimson' }}>{error}</div>}

        {result && (
          <div style={{ marginTop: 16 }}>
            <h3>답변</h3>
            <pre style={{ whiteSpace: 'pre-wrap' }}>{result.response}</pre>
            <h4>소스</h4>
            <ul>
              {result.sources.map((s, idx) => (
                <li key={idx}>
                  <strong>{s.type}</strong> - 점수: {s.score}
                  <div>{s.content}</div>
                </li>
              ))}
            </ul>
            <h4>그래프 경로</h4>
            <ul>
              {result.graph_paths.map((p, idx) => (
                <li key={idx}>{p.path}</li>
              ))}
            </ul>
            <div>
              소요 시간: {result.query_time}s, 사용 토큰: {result.tokens_used}
            </div>
          </div>
        )}
      </Stack>
    </div>
  );
};

export default Chat;
