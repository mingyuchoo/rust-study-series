<script lang="ts">
  import { onMount } from 'svelte';
  import { getHealth } from '$lib/services/health';

  let data = $state<any>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      data = await getHealth();
    } catch (e: any) {
      error = e?.message ?? '헬스 체크 실패';
    }
  });
</script>

<!-- 시스템 상태 페이지 -->
<div class="page">
  <div class="stack">
    <h2>시스템 상태</h2>

    {#if error}
      <div class="error">{error}</div>
    {:else if !data}
      <div class="loading-state">로딩 중...</div>
    {:else}
      <div class="card">
        <pre class="code-block">{JSON.stringify(data, null, 2)}</pre>
      </div>
    {/if}
  </div>
</div>

<style>
  .loading-state {
    text-align: center;
    padding: 48px;
    color: var(--color-gray-400);
    font-size: 14px;
  }

  .code-block {
    white-space: pre-wrap;
    background: var(--color-gray-50);
    border: 1px solid var(--color-gray-100);
    padding: 20px;
    border-radius: var(--radius-sm);
    font-family: var(--font-mono);
    font-size: 13px;
    line-height: 1.7;
    color: var(--color-gray-700);
  }
</style>
