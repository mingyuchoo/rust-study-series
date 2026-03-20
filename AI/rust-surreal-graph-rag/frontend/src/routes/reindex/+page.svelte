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
      const arr = pathsText ? pathsText.split(/\r?\n/).filter(Boolean) : [];
      if (!arr.includes(resp.path)) arr.push(resp.path);
      pathsText = arr.join('\n');
    } catch (err: any) {
      error = err?.message ?? '파일 업로드 중 오류가 발생했습니다.';
    } finally {
      uploading = false;
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
    <h2>재인덱싱</h2>

    <div class="card">
      <div class="stack">
        <p class="desc">
          서버에 파일을 업로드하고 업로드된 경로로 재인덱싱을 수행합니다.
        </p>

        <div class="upload-area">
          <label class="upload-label" for="file-upload">
            <span class="upload-icon">+</span>
            <span>{uploading ? '업로드 중...' : 'PDF 파일 선택'}</span>
          </label>
          <input id="file-upload" type="file" accept="application/pdf" onchange={onUpload} class="file-input" />
        </div>

        <div class="field">
          <label for="paths">PDF 파일 경로 (줄바꿈 구분)</label>
          <textarea id="paths" rows="6" placeholder="/path/to/file.pdf" bind:value={pathsText}></textarea>
        </div>

        <label class="checkbox-label">
          <input type="checkbox" bind:checked={clearExisting} />
          <span>기존 데이터 정리 후 재인덱싱</span>
        </label>

        <button class="btn btn-primary" onclick={onRun} disabled={loading}>
          {loading ? '처리 중...' : '재인덱싱 실행'}
        </button>
      </div>
    </div>

    {#if error}
      <div class="error">{error}</div>
    {/if}

    {#if result}
      <div class="card">
        <div class="stack">
          <div class="result-header">
            <h3 class="section-title">실행 결과</h3>
            <span class="result-meta">전체 소요 시간: {result.elapsed}s</span>
          </div>
          <div class="result-list">
            {#each result.results as r}
              <div class="result-item">
                <div class="result-row">
                  <span class="result-label">PDF</span>
                  <span class="result-value">{r.pdf_path}</span>
                </div>
                <div class="result-row">
                  <span class="result-label">문서 ID</span>
                  <span class="result-value">{r.document_id ?? '-'}</span>
                </div>
                <div class="result-row">
                  <span class="result-label">청크 수</span>
                  <span class="result-value">{r.chunks_indexed}</span>
                </div>
                {#if r.error}
                  <div class="error">{r.error}</div>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .desc {
    font-size: 13px;
    color: var(--color-gray-500);
    line-height: 1.5;
  }

  .upload-area {
    position: relative;
  }

  .file-input {
    position: absolute;
    inset: 0;
    opacity: 0;
    cursor: pointer;
  }

  .upload-label {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 20px;
    border: 2px dashed var(--color-gray-200);
    border-radius: var(--radius-md);
    font-size: 13px;
    font-weight: 500;
    color: var(--color-gray-500);
    cursor: pointer;
    transition:
      border-color var(--transition),
      color var(--transition);
  }

  .upload-label:hover {
    border-color: var(--color-gray-400);
    color: var(--color-gray-700);
  }

  .upload-icon {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-gray-100);
    border-radius: 50%;
    font-size: 16px;
    font-weight: 600;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    font-size: 13px;
    color: var(--color-gray-700);
  }

  .checkbox-label input[type='checkbox'] {
    width: 16px;
    height: 16px;
    accent-color: var(--color-gray-900);
  }

  .result-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .section-title {
    font-size: 13px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--color-gray-500);
  }

  .result-meta {
    font-size: 12px;
    color: var(--color-gray-400);
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
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .result-row {
    display: flex;
    align-items: baseline;
    gap: 12px;
    font-size: 13px;
  }

  .result-label {
    color: var(--color-gray-400);
    min-width: 60px;
    flex-shrink: 0;
  }

  .result-value {
    color: var(--color-gray-800);
    font-family: var(--font-mono);
    word-break: break-all;
  }
</style>
