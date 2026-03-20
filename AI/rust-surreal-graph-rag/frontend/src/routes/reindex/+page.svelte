<script lang="ts">
  import { reindexPdfs, uploadFile } from '$lib/services/reindex';
  import type { ReindexResponse } from '$lib/types/api';

  let pathsText = $state('');
  let clearExisting = $state(true);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let result = $state<ReindexResponse | null>(null);
  let uploading = $state(false);

  async function onUpload(e: Event) {
    const target = e.target as HTMLInputElement;
    const f = target.files?.[0];
    if (!f) return;
    uploading = true;
    error = null;
    try {
      const resp = await uploadFile(f);
      // 업로드된 서버 경로를 재인덱싱 입력에 자동 추가
      const arr = pathsText ? pathsText.split(/\r?\n/).filter(Boolean) : [];
      if (!arr.includes(resp.path)) arr.push(resp.path);
      pathsText = arr.join('\n');
    } catch (err: any) {
      error = err?.message ?? '파일 업로드 중 오류가 발생했습니다.';
    } finally {
      uploading = false;
      // 같은 파일을 다시 선택할 수 있도록 입력 초기화
      target.value = '';
    }
  }

  async function onRun() {
    loading = true;
    error = null;
    result = null;
    try {
      const pdf_paths = pathsText
        .split(/\r?\n/)
        .map((s) => s.trim())
        .filter(Boolean);
      if (pdf_paths.length === 0) {
        throw new Error('PDF 경로를 한 줄에 하나씩 입력하세요.');
      }
      result = await reindexPdfs({ pdf_paths, clear_existing: clearExisting });
    } catch (e: any) {
      error = e?.message ?? '재인덱싱 처리 중 오류가 발생했습니다.';
    } finally {
      loading = false;
    }
  }
</script>

<!-- 재인덱싱(관리자) 페이지 -->
<div class="page">
  <div class="stack">
    <h2>재인덱싱(관리자)</h2>

    <div class="desc">
      서버에 직접 파일을 업로드하고 업로드된 경로로 재인덱싱을 수행할 수 있습니다. 업로드 후 경로 입력란에 자동으로
      추가됩니다.
    </div>

    <div class="stack-horizontal">
      <input type="file" accept="application/pdf" onchange={onUpload} />
      <button class="btn btn-default" disabled={uploading}>
        {uploading ? '업로드 중...' : '업로드 선택'}
      </button>
    </div>

    <div class="field">
      <label for="paths">PDF 파일 경로(서버 경로, 줄바꿈으로 구분)</label>
      <textarea id="paths" rows="6" bind:value={pathsText}></textarea>
    </div>

    <label class="checkbox-label">
      <input type="checkbox" bind:checked={clearExisting} />
      기존 데이터 정리(같은 source 데이터 삭제 후 재인덱싱)
    </label>

    <button class="btn btn-primary" onclick={onRun} disabled={loading}>
      {loading ? '처리 중...' : '재인덱싱 실행'}
    </button>

    {#if error}
      <div class="error">{error}</div>
    {/if}

    {#if result}
      <div>
        <h3>실행 결과</h3>
        <div>전체 소요 시간: {result.elapsed}s</div>
        <ul>
          {#each result.results as r}
            <li>
              <div>PDF: {r.pdf_path}</div>
              <div>문서 ID: {r.document_id ?? '-'}</div>
              <div>청크 수: {r.chunks_indexed}</div>
              {#if r.error}
                <div class="error">오류: {r.error}</div>
              {/if}
            </li>
          {/each}
        </ul>
      </div>
    {/if}
  </div>
</div>

<style>
  .desc {
    color: var(--color-text-muted);
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
  }
</style>
