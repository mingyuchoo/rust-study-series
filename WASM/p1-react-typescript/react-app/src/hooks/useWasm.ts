import { useEffect, useState } from "react";
import init from "wasm-lib";

interface WasmState {
  ready: boolean;
  error: string | null;
}

export function useWasm(): WasmState {
  const [state, setState] = useState<WasmState>({
    ready: false,
    error: null,
  });

  useEffect(() => {
    let cancelled = false;
    init()
      .then(() => {
        if (!cancelled) setState({ ready: true, error: null });
      })
      .catch((err: unknown) => {
        if (!cancelled)
          setState({
            ready: false,
            error: err instanceof Error ? err.message : "WASM 초기화 실패",
          });
      });
    return () => {
      cancelled = true;
    };
  }, []);

  return state;
}
