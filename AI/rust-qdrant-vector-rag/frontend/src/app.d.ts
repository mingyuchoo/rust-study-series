// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
  namespace App {
    // interface Error {}
    // interface Locals {}
    // interface PageData {}
    // interface Platform {}
  }

  // Environment variables
  interface ImportMetaEnv {
    readonly VITE_API_BASE_URL: string;
    readonly VITE_API_TIMEOUT: string;
    readonly VITE_APP_NAME: string;
    readonly VITE_APP_VERSION: string;
    readonly VITE_MAX_FILE_SIZE: string;
    readonly VITE_SUPPORTED_FILE_TYPES: string;
    readonly VITE_MAX_FILES_PER_UPLOAD: string;
    readonly VITE_DEFAULT_MAX_CHUNKS: string;
    readonly VITE_DEFAULT_SIMILARITY_THRESHOLD: string;
    readonly VITE_DEFAULT_TEMPERATURE: string;
    readonly VITE_MAX_QUERY_LENGTH: string;
    readonly VITE_THEME: string;
    readonly VITE_ENABLE_ANIMATIONS: string;
  }

  interface ImportMeta {
    readonly env: ImportMetaEnv;
  }
}

export {};
