import React, { useMemo, useState } from 'react';
import { Stack, PrimaryButton, TextField } from '@fluentui/react';
import { createIndexing } from '@/services/indexing';
import { IndexCreateResponse } from '@/types/api';

// PDF 텍스트 추출 유틸리티 (pdfjs-dist 이용)
// Vite 환경에서 동작하도록 동적 import 사용
async function extractPdfText(file: File): Promise<string> {
  const pdfjsLib = await import('pdfjs-dist');
  // 워커 경로 설정: Vite의 asset import 기능(?url)로 문자열 URL을 주입해야 함
  // 모듈 객체를 그대로 할당하면 Invalid `workerSrc` type 오류가 발생하므로 URL 문자열을 사용한다.
  // @ts-ignore
  const workerUrl = (await import('pdfjs-dist/build/pdf.worker.min.mjs?url')).default as string;
  // @ts-ignore
  pdfjsLib.GlobalWorkerOptions.workerSrc = workerUrl;

  const arrayBuffer = await file.arrayBuffer();
  // @ts-ignore
  const loadingTask = pdfjsLib.getDocument({ data: arrayBuffer });
  const pdf = await loadingTask.promise;
  let fullText = '';
  for (let pageNum = 1; pageNum <= pdf.numPages; pageNum++) {
    const page = await pdf.getPage(pageNum);
    const content = await page.getTextContent();
    const strings = content.items.map((it: any) => (it.str ? it.str : ''));
    fullText += strings.join(' ') + '\n';
  }
  return fullText;
}

// 간단 청크 분할기: 문단 기준 + 길이 제한
function chunkText(text: string, maxLen = 800): string[] {
  const paras = text
    .split(/\n{2,}/)
    .map((s) => s.trim())
    .filter(Boolean);
  const chunks: string[] = [];
  for (const p of paras) {
    if (p.length <= maxLen) {
      chunks.push(p);
    } else {
      // 너무 길면 문장 단위로 재분할
      const sents = p.split(/(?<=[.!?\u3002\uFF01\uFF1F])\s+/);
      let buf = '';
      for (const s of sents) {
        if ((buf + ' ' + s).trim().length > maxLen) {
          if (buf.trim()) chunks.push(buf.trim());
          buf = s;
        } else {
          buf = (buf + ' ' + s).trim();
        }
      }
      if (buf.trim()) chunks.push(buf.trim());
    }
  }
  // 빈 줄 제거
  return chunks.filter((c) => c.length >= 20);
}

const Indexing: React.FC = () => {
  // 상태 관리
  const [file, setFile] = useState<File | null>(null);
  const [title, setTitle] = useState<string>('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [result, setResult] = useState<IndexCreateResponse | null>(null);

  const fileName = useMemo(() => (file ? file.name : '선택된 파일 없음'), [file]);

  const onFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const f = e.target.files?.[0] ?? null;
    setFile(f);
    if (f && !title) setTitle(f.name.replace(/\.pdf$/i, ''));
  };

  const onIndexing = async () => {
    if (!file) return;
    setLoading(true);
    setError(null);
    setResult(null);
    try {
      // 1) PDF에서 텍스트 추출
      const text = await extractPdfText(file);
      // 2) 텍스트 청크 분할
      const chunks = chunkText(text).map((content) => ({ content }));
      if (chunks.length === 0) {
        throw new Error('PDF에서 텍스트를 추출하지 못했거나 유효한 청크가 없습니다.');
      }
      // 3) 서버에 인덱싱 생성 요청
      const res = await createIndexing({ title, chunks });
      setResult(res);
    } catch (e: any) {
      setError(e?.message ?? '인덱싱 처리 중 오류가 발생했습니다.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ padding: 16 }}>
      <Stack tokens={{ childrenGap: 12 }}>
        <h2>인덱싱 생성</h2>
        <input type="file" accept="application/pdf" onChange={onFileChange} />
        <div>선택 파일: {fileName}</div>
        <TextField label="문서 제목" value={title} onChange={(_, v) => setTitle(v || '')} />
        <PrimaryButton onClick={onIndexing} disabled={loading || !file}>
          {loading ? '처리 중...' : '인덱싱 생성'}
        </PrimaryButton>

        {error && <div style={{ color: 'crimson' }}>{error}</div>}

        {result && (
          <div>
            <h3>생성 완료</h3>
            <div>문서 ID: {result.document_id}</div>
            <div>청크 수: {result.chunks_indexed}</div>
            <div>소요 시간: {result.elapsed}s</div>
          </div>
        )}
      </Stack>
    </div>
  );
};

export default Indexing;
