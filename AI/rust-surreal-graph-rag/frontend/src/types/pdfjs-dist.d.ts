// 타입스크립트용 임시 타입 선언 (pdfjs-dist)
// 브라우저 번들 환경에서 동적 import로 사용하므로 최소 선언만 제공

declare module 'pdfjs-dist' {
  export const GlobalWorkerOptions: { workerSrc: any };
  export function getDocument(src: any): { promise: Promise<any> };
}

declare module 'pdfjs-dist/build/pdf.worker.min.mjs' {
  const workerSrc: any;
  export default workerSrc;
}
