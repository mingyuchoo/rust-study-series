import React, { useState } from 'react';
import { Stack, TextField, PrimaryButton, DefaultButton } from '@fluentui/react';
import { reindexPdfs, uploadFile } from '@/services/reindex';
import type { ReindexRequest, ReindexResponse } from '@/types/api';

// 관리자 재인덱싱 페이지
// - PDF 경로 목록 입력(줄바꿈 구분)
// - 기존 데이터 정리 여부 선택
// - 실행 결과 표시
const Reindex: React.FC = () => {
  const [pathsText, setPathsText] = useState<string>('');
  const [clearExisting, setClearExisting] = useState<boolean>(true);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [result, setResult] = useState<ReindexResponse | null>(null);
  const [uploading, setUploading] = useState<boolean>(false);

  const onUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const f = e.target.files?.[0];
    if (!f) return;
    setUploading(true);
    setError(null);
    try {
      const resp = await uploadFile(f);
      // 업로드된 서버 경로를 재인덱싱 입력에 자동 추가
      setPathsText((prev) => {
        const arr = prev ? prev.split(/\r?\n/).filter(Boolean) : [];
        if (!arr.includes(resp.path)) arr.push(resp.path);
        return arr.join('\n');
      });
    } catch (err: any) {
      setError(err?.message ?? '파일 업로드 중 오류가 발생했습니다.');
    } finally {
      setUploading(false);
      // 같은 파일을 다시 선택할 수 있도록 입력 초기화
      e.target.value = '';
    }
  };

  const onRun = async () => {
    setLoading(true);
    setError(null);
    setResult(null);
    try {
      const pdf_paths = pathsText
        .split(/\r?\n/)
        .map((s) => s.trim())
        .filter(Boolean);
      if (pdf_paths.length === 0) {
        throw new Error('PDF 경로를 한 줄에 하나씩 입력하세요.');
      }
      const payload: ReindexRequest = {
        pdf_paths,
        clear_existing: clearExisting,
      };
      const res = await reindexPdfs(payload);
      setResult(res);
    } catch (e: any) {
      setError(e?.message ?? '재인덱싱 처리 중 오류가 발생했습니다.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ padding: 16 }}>
      <Stack tokens={{ childrenGap: 12 }}>
        <h2>재인덱싱(관리자)</h2>
        <div style={{ color: '#666' }}>
          서버에 직접 파일을 업로드하고 업로드된 경로로 재인덱싱을 수행할 수 있습니다. 업로드 후 경로 입력란에 자동으로 추가됩니다.
        </div>
        <Stack horizontal tokens={{ childrenGap: 12 }} verticalAlign="end">
          <input type="file" accept="application/pdf" onChange={onUpload} />
          <DefaultButton disabled={uploading}>
            {uploading ? '업로드 중...' : '업로드 선택'}
          </DefaultButton>
        </Stack>
        <TextField
          label="PDF 파일 경로(서버 경로, 줄바꿈으로 구분)"
          value={pathsText}
          onChange={(_, v) => setPathsText(v || '')}
          multiline
          rows={6}
        />
        <Stack horizontal tokens={{ childrenGap: 16 }}>
          <label style={{ display: 'flex', alignItems: 'center' }}>
            <input
              type="checkbox"
              checked={clearExisting}
              onChange={(e) => setClearExisting(e.target.checked)}
              style={{ marginRight: 8 }}
            />
            기존 데이터 정리(같은 source 데이터 삭제 후 재인덱싱)
          </label>
        </Stack>
        <PrimaryButton onClick={onRun} disabled={loading}>
          {loading ? '처리 중...' : '재인덱싱 실행'}
        </PrimaryButton>
        {error && <div style={{ color: 'crimson' }}>{error}</div>}
        {result && (
          <div>
            <h3>실행 결과</h3>
            <div>전체 소요 시간: {result.elapsed}s</div>
            <ul>
              {result.results.map((r, idx) => (
                <li key={idx}>
                  <div>PDF: {r.pdf_path}</div>
                  <div>문서 ID: {r.document_id ?? '-'}</div>
                  <div>청크 수: {r.chunks_indexed}</div>
                  {r.error && <div style={{ color: 'crimson' }}>오류: {r.error}</div>}
                </li>
              ))}
            </ul>
          </div>
        )}
      </Stack>
    </div>
  );
};

export default Reindex;