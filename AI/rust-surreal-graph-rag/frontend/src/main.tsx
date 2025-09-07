import React from 'react';
import ReactDOM from 'react-dom/client';
import { initializeIcons } from '@fluentui/react';
import { BrowserRouter } from 'react-router-dom';
import App from './App';

// Fluent UI 아이콘 초기화
initializeIcons();

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    {/* 라우터 및 앱 마운트 (Fluent UI v8 ThemeProvider는 App 내부에서 처리) */}
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </React.StrictMode>,
);
