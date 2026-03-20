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

    <div class="field">
      <label for="query">쿼리</label>
      <input id="query" type="text" bind:value={query} />
    </div>

    <div class="field">
      <label for="topk">top_k</label>
      <input id="topk" type="number" bind:value={topK} />
    </div>

    <div class="field">
      <label for="maxhops">max_hops</label>
      <input id="maxhops" type="number" bind:value={maxHops} />
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
        {#if result.paths.length === 0}
          <div>경로 결과가 없습니다.</div>
        {:else}
          <ul>
            {#each result.paths as p}
              <li>
                <div><strong>경로:</strong> {p.path}</div>
                <pre>{JSON.stringify({ nodes: p.nodes, relationships: p.relationships }, null, 2)}</pre>
              </li>
            {/each}
          </ul>
        {/if}
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
