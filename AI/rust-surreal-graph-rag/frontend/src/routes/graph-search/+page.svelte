<script lang="ts">
  import { graphSearch } from '$lib/services/graphSearch';
  import type { GraphSearchResponse } from '$lib/types/api';

  let query = $state('');
  let topK = $state(5);
  let maxHops = $state(2);
  let loading = $state(false);
  let result = $state<GraphSearchResponse | null>(null);
  let error = $state<string | null>(null);

  async function onSearch() {
    if (!query.trim()) return;
    loading = true;
    error = null;
    try {
      result = await graphSearch({ query, top_k: topK, max_hops: maxHops });
    } catch (e: any) {
      error = e?.message ?? '요청 중 오류가 발생했습니다.';
    } finally {
      loading = false;
    }
  }
</script>

<!-- 그래프 검색 페이지 -->
<div class="page">
  <div class="stack">
    <h2>그래프 검색</h2>

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
            <label for="maxhops">max_hops</label>
            <input id="maxhops" type="number" bind:value={maxHops} />
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

          {#if result.paths.length === 0}
            <div class="empty-state">경로 결과가 없습니다.</div>
          {:else}
            <div class="path-list">
              {#each result.paths as p}
                <div class="path-item">
                  <div class="path-label">{p.path}</div>
                  <pre class="code-block">{JSON.stringify({ nodes: p.nodes, relationships: p.relationships }, null, 2)}</pre>
                </div>
              {/each}
            </div>
          {/if}
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

  .empty-state {
    text-align: center;
    padding: 32px;
    color: var(--color-gray-400);
    font-size: 14px;
  }

  .path-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .path-item {
    padding: 16px;
    background: var(--color-gray-50);
    border: 1px solid var(--color-gray-100);
    border-radius: var(--radius-sm);
  }

  .path-label {
    font-size: 13px;
    font-weight: 600;
    color: var(--color-gray-800);
    margin-bottom: 12px;
  }

  .code-block {
    white-space: pre-wrap;
    background: var(--color-white);
    border: 1px solid var(--color-gray-200);
    padding: 16px;
    border-radius: var(--radius-sm);
    font-family: var(--font-mono);
    font-size: 12px;
    line-height: 1.6;
    color: var(--color-gray-700);
  }
</style>
