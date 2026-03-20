<script lang="ts">
  import { chatAsk } from '$lib/services/chat';
  import type { ChatAskResponse } from '$lib/types/api';

  let query = $state('');
  let conversationId = $state<string | null>(null);
  let loading = $state(false);
  let result = $state<ChatAskResponse | null>(null);
  let error = $state<string | null>(null);

  async function onAsk() {
    if (!query.trim()) return;
    loading = true;
    error = null;
    try {
      const res = await chatAsk({ query, conversation_id: conversationId });
      result = res;
      conversationId = res.conversation_id ?? conversationId;
    } catch (e: any) {
      error = e?.message ?? '요청 중 오류가 발생했습니다.';
    } finally {
      loading = false;
    }
  }
</script>

<!-- 통합 질의응답 페이지 -->
<div class="page">
  <div class="stack">
    <h2>통합 질의응답</h2>

    <div class="field">
      <label for="query">질문</label>
      <textarea id="query" rows="3" bind:value={query}></textarea>
    </div>

    <button class="btn btn-primary" onclick={onAsk} disabled={loading}>
      {loading ? '요청 중...' : '질의'}
    </button>

    {#if error}
      <div class="error">{error}</div>
    {/if}

    {#if result}
      <div class="result">
        <h3>답변</h3>
        <pre>{result.response}</pre>

        <h4>소스</h4>
        <ul>
          {#each result.sources as s}
            <li>
              <strong>{s.type}</strong> - 점수: {s.score}
              <div>{s.content}</div>
            </li>
          {/each}
        </ul>

        <h4>그래프 경로</h4>
        <ul>
          {#each result.graph_paths as p}
            <li>{p.path}</li>
          {/each}
        </ul>

        <div>소요 시간: {result.query_time}s, 사용 토큰: {result.tokens_used}</div>
      </div>
    {/if}
  </div>
</div>

<style>
  .result {
    margin-top: 16px;
  }

  pre {
    white-space: pre-wrap;
    background: #f5f5f5;
    padding: 12px;
    border-radius: 4px;
  }
</style>
