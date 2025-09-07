// 타입스크립트용 임시 타입 선언 (pdfjs-dist)
// 브라우저 번들 환경에서 동적 import로 사용하므로 최소 선언만 제공

declare module 'pdfjs-dist' {
  // workerSrc는 문자열 URL이어야 함
  export const GlobalWorkerOptions: { workerSrc: string };
  export function getDocument(src: any): { promise: Promise<any> };
}

declare module 'pdfjs-dist/build/pdf.worker.min.mjs' {
  const workerSrc: any;
  export default workerSrc;
}

// Vite의 asset import를 통한 URL 문자열 임포트 지원
declare module 'pdfjs-dist/build/pdf.worker.min.mjs?url' {
  const url: string;
  export default url;
}
