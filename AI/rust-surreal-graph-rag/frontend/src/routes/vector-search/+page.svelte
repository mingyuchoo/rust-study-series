<script lang="ts">
  import { vectorSearch } from '$lib/services/vectorSearch';
  import type { VectorSearchResponse } from '$lib/types/api';

  let query = $state('');
  let topK = $state(5);
  let threshold = $state(0.2);
  let loading = $state(false);
  let result = $state<VectorSearchResponse | null>(null);
  let error = $state<string | null>(null);

  async function onSearch() {
    if (!query.trim()) return;
    loading = true;
    error = null;
    try {
      result = await vectorSearch({ query, top_k: topK, threshold });
    } catch (e: any) {
      error = e?.message ?? '요청 중 오류가 발생했습니다.';
    } finally {
      loading = false;
    }
  }
</script>

<!-- 벡터 검색 페이지 -->
<div class="page">
  <div class="stack">
    <h2>벡터 검색</h2>

    <div class="card">
      <div class="stack">
        <div class="field">
          <label for="query">쿼리</label>
          <input id="query" type="text" placeholder="검색할 내용을 입력하세요..." bind:value={query} />
        </div>

        <div class="field-row">
          <div class="field">
            <label for="topk">top_k</label>
            <input id="topk" type="number" bind:value={topK} />
          </div>

          <div class="field">
            <label for="threshold">threshold</label>
            <input id="threshold" type="number" step="0.1" bind:value={threshold} />
          </div>
        </div>

        <button class="btn btn-primary" onclick={onSearch} disabled={loading}>
          {loading ? '검색 중...' : '검색'}
        </button>
      </div>
    </div>

    {#if error}
      <div class="error">{error}</div>
    {/if}

    {#if result}
      <div class="card">
        <div class="stack">
          <div class="result-meta">
            총 {result.total}건 &middot; 소요 시간: {result.query_time}s
          </div>
          <div class="result-list">
            {#each result.results as r}
              <div class="result-item">
                <div class="result-header">
                  <span class="result-id">{r.id}</span>
                  <span class="result-score">점수: {r.score}</span>
                </div>
                <p class="result-content">{r.content}</p>
              </div>
            {/each}
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .field-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
  }

  .result-meta {
    font-size: 13px;
    font-weight: 500;
    color: var(--color-gray-500);
  }

  .result-list {
    display: flex;
    flex-direction: column;
    gap: 1px;
    background: var(--color-gray-100);
    border-radius: var(--radius-sm);
    overflow: hidden;
    border: 1px solid var(--color-gray-200);
  }

  .result-item {
    padding: 16px;
    background: var(--color-white);
  }

  .result-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 8px;
  }

  .result-id {
    font-size: 13px;
    font-weight: 600;
    font-family: var(--font-mono);
    color: var(--color-gray-800);
  }

  .result-score {
    font-size: 12px;
    color: var(--color-gray-400);
  }

  .result-content {
    font-size: 13px;
    color: var(--color-gray-600);
    line-height: 1.6;
  }
</style>
