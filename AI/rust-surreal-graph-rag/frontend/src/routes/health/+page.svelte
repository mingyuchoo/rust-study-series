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
{#if error}
  <div class="page error">{error}</div>
{:else if !data}
  <div class="page">로딩 중...</div>
{:else}
  <div class="page">
    <h2>시스템 상태</h2>
    <pre>{JSON.stringify(data, null, 2)}</pre>
  </div>
{/if}

<style>
  pre {
    white-space: pre-wrap;
    background: #f5f5f5;
    padding: 12px;
    border-radius: 4px;
  }
</style>
