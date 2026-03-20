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

    <div class="card">
      <div class="stack">
        <div class="field">
          <label for="query">질문</label>
          <textarea id="query" rows="3" placeholder="궁금한 내용을 입력하세요..." bind:value={query}></textarea>
        </div>

        <button class="btn btn-primary" onclick={onAsk} disabled={loading}>
          {loading ? '요청 중...' : '질의'}
        </button>
      </div>
    </div>

    {#if error}
      <div class="error">{error}</div>
    {/if}

    {#if result}
      <div class="card">
        <div class="stack">
          <div class="section">
            <h3 class="section-title">답변</h3>
            <pre class="code-block">{result.response}</pre>
          </div>

          <div class="section">
            <h3 class="section-title">소스</h3>
            <div class="source-list">
              {#each result.sources as s}
                <div class="source-item">
                  <div class="source-meta">
                    <span class="badge">{s.type}</span>
                    <span class="score">점수: {s.score}</span>
                  </div>
                  <p class="source-content">{s.content}</p>
                </div>
              {/each}
            </div>
          </div>

          {#if result.graph_paths.length > 0}
            <div class="section">
              <h3 class="section-title">그래프 경로</h3>
              <ul class="path-list">
                {#each result.graph_paths as p}
                  <li>{p.path}</li>
                {/each}
              </ul>
            </div>
          {/if}

          <div class="meta-info">
            소요 시간: {result.query_time}s &middot; 사용 토큰: {result.tokens_used}
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .section {
    padding-bottom: 16px;
    border-bottom: 1px solid var(--color-gray-100);
  }

  .section:last-child {
    border-bottom: none;
    padding-bottom: 0;
  }

  .section-title {
    font-size: 13px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--color-gray-500);
    margin-bottom: 12px;
  }

  .code-block {
    white-space: pre-wrap;
    background: var(--color-gray-50);
    border: 1px solid var(--color-gray-200);
    padding: 16px;
    border-radius: var(--radius-sm);
    font-family: var(--font-mono);
    font-size: 13px;
    line-height: 1.6;
    color: var(--color-gray-800);
  }

  .source-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .source-item {
    padding: 12px;
    background: var(--color-gray-50);
    border-radius: var(--radius-sm);
    border: 1px solid var(--color-gray-100);
  }

  .source-meta {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 8px;
  }

  .badge {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    padding: 2px 8px;
    background: var(--color-gray-900);
    color: var(--color-white);
    border-radius: 4px;
  }

  .score {
    font-size: 12px;
    color: var(--color-gray-500);
  }

  .source-content {
    font-size: 13px;
    color: var(--color-gray-700);
    line-height: 1.5;
  }

  .path-list {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .path-list li {
    font-size: 13px;
    font-family: var(--font-mono);
    color: var(--color-gray-700);
    padding: 8px 12px;
    background: var(--color-gray-50);
    border-radius: var(--radius-sm);
    border: 1px solid var(--color-gray-100);
  }

  .meta-info {
    font-size: 12px;
    color: var(--color-gray-400);
    padding-top: 8px;
  }
</style>
