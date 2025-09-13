import { mount } from 'svelte';
import './app.css';
import App from './App.svelte';
import { initServiceWorker, startPerformanceMonitoring } from './lib/services/service-worker.js';

const appElement = document.getElementById('app');
if (!appElement) {
  throw new Error('Could not find app element');
}

const app = mount(App, {
  target: appElement,
});

// Initialize service worker for caching and offline functionality
initServiceWorker().catch(error => {
  console.error('Failed to initialize service worker:', error);
});

// Start performance monitoring in production
if (import.meta.env.PROD) {
  startPerformanceMonitoring();
}

// Handle service worker updates
window.addEventListener('sw-update-available', (event: any) => {
  console.log('Service worker update available');
  
  // Show update notification to user
  const updateBanner = document.createElement('div');
  updateBanner.innerHTML = `
    <div style="
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      background: #2563eb;
      color: white;
      padding: 1rem;
      text-align: center;
      z-index: 9999;
      box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    ">
      <p style="margin: 0 0 0.5rem 0; font-weight: 600;">
        A new version of the app is available!
      </p>
      <button id="update-app" style="
        background: white;
        color: #2563eb;
        border: none;
        padding: 0.5rem 1rem;
        border-radius: 0.375rem;
        font-weight: 600;
        cursor: pointer;
        margin-right: 0.5rem;
      ">
        Update Now
      </button>
      <button id="dismiss-update" style="
        background: transparent;
        color: white;
        border: 1px solid white;
        padding: 0.5rem 1rem;
        border-radius: 0.375rem;
        font-weight: 600;
        cursor: pointer;
      ">
        Later
      </button>
    </div>
  `;
  
  document.body.appendChild(updateBanner);
  
  // Handle update button click
  document.getElementById('update-app')?.addEventListener('click', () => {
    // Skip waiting and reload
    if (event.detail.registration?.waiting) {
      event.detail.registration.waiting.postMessage({ type: 'SKIP_WAITING' });
    }
    window.location.reload();
  });
  
  // Handle dismiss button click
  document.getElementById('dismiss-update')?.addEventListener('click', () => {
    document.body.removeChild(updateBanner);
  });
});

export default app;
