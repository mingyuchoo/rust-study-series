// WASM 모듈을 비동기로 로딩한다
import("./index.js").catch((e) =>
  console.error("WASM 모듈 로딩 실패:", e)
);
