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

    <div class="field">
      <label for="query">쿼리</label>
      <input id="query" type="text" bind:value={query} />
    </div>

    <div class="field">
      <label for="topk">top_k</label>
      <input id="topk" type="number" bind:value={topK} />
    </div>

    <div class="field">
      <label for="threshold">threshold</label>
      <input id="threshold" type="number" step="0.1" bind:value={threshold} />
    </div>

    <button class="btn btn-primary" onclick={onSearch} disabled={loading}>
      {loading ? '검색 중...' : '검색'}
    </button>

    {#if error}
      <div class="error">{error}</div>
    {/if}

    {#if result}
      <div class="result">
        <div>총 {result.total}건, 소요 시간: {result.query_time}s</div>
        <ul>
          {#each result.results as r}
            <li>
              <strong>{r.id}</strong> - 점수: {r.score}
              <div>{r.content}</div>
            </li>
          {/each}
        </ul>
      </div>
    {/if}
  </div>
</div>

<style>
  .result {
    margin-top: 16px;
  }
</style>
