// Service Worker Registration and Management
// Handles service worker lifecycle and communication

export interface ServiceWorkerManager {
  register(): Promise<ServiceWorkerRegistration | null>;
  unregister(): Promise<boolean>;
  update(): Promise<void>;
  isSupported(): boolean;
  getRegistration(): Promise<ServiceWorkerRegistration | null>;
  postMessage(message: any): void;
  onMessage(callback: (event: MessageEvent) => void): void;
  clearCache(): Promise<boolean>;
  getVersion(): Promise<string>;
}

class ServiceWorkerManagerImpl implements ServiceWorkerManager {
  private registration: ServiceWorkerRegistration | null = null;
  private messageCallbacks: ((event: MessageEvent) => void)[] = [];

  constructor() {
    // Listen for messages from service worker
    if (this.isSupported()) {
      navigator.serviceWorker.addEventListener('message', (event) => {
        this.messageCallbacks.forEach(callback => callback(event));
      });
    }
  }

  isSupported(): boolean {
    return 'serviceWorker' in navigator;
  }

  async register(): Promise<ServiceWorkerRegistration | null> {
    if (!this.isSupported()) {
      console.warn('Service Worker not supported in this browser');
      return null;
    }

    try {
      console.log('Service Worker: Registering...');
      
      this.registration = await navigator.serviceWorker.register('/sw.js', {
        scope: '/'
      });

      console.log('Service Worker: Registered successfully', this.registration);

      // Handle updates
      this.registration.addEventListener('updatefound', () => {
        console.log('Service Worker: Update found');
        this.handleUpdate();
      });

      // Check for existing service worker
      if (this.registration.active) {
        console.log('Service Worker: Already active');
      }

      // Check for waiting service worker
      if (this.registration.waiting) {
        console.log('Service Worker: Waiting for activation');
        this.showUpdateAvailable();
      }

      // Listen for controlling service worker changes
      navigator.serviceWorker.addEventListener('controllerchange', () => {
        console.log('Service Worker: Controller changed');
        window.location.reload();
      });

      return this.registration;
    } catch (error) {
      console.error('Service Worker: Registration failed', error);
      return null;
    }
  }

  async unregister(): Promise<boolean> {
    if (!this.registration) {
      return false;
    }

    try {
      const result = await this.registration.unregister();
      console.log('Service Worker: Unregistered', result);
      this.registration = null;
      return result;
    } catch (error) {
      console.error('Service Worker: Unregistration failed', error);
      return false;
    }
  }

  async update(): Promise<void> {
    if (!this.registration) {
      throw new Error('Service Worker not registered');
    }

    try {
      await this.registration.update();
      console.log('Service Worker: Update check completed');
    } catch (error) {
      console.error('Service Worker: Update failed', error);
      throw error;
    }
  }

  async getRegistration(): Promise<ServiceWorkerRegistration | null> {
    if (!this.isSupported()) {
      console.warn('Service Worker not supported in this browser');
      return null;
    }

    try {
      const registration = await navigator.serviceWorker.getRegistration();
      return registration || null;
    } catch (error) {
      console.error('Service Worker: Failed to get registration', error);
      return null;
    }
  }

  postMessage(message: any): void {
    if (!this.isSupported() || !navigator.serviceWorker.controller) {
      console.warn('Service Worker: Cannot post message - no controller');
      return;
    }

    navigator.serviceWorker.controller.postMessage(message);
  }

  onMessage(callback: (event: MessageEvent) => void): void {
    this.messageCallbacks.push(callback);
  }

  async clearCache(): Promise<boolean> {
    return new Promise((resolve) => {
      const messageChannel = new MessageChannel();
      
      messageChannel.port1.onmessage = (event) => {
        const { success } = event.data;
        resolve(success);
      };

      this.postMessage({
        type: 'CLEAR_CACHE'
      });

      // Fallback timeout
      setTimeout(() => resolve(false), 5000);
    });
  }

  async getVersion(): Promise<string> {
    return new Promise((resolve) => {
      const messageChannel = new MessageChannel();
      
      messageChannel.port1.onmessage = (event) => {
        const { version } = event.data;
        resolve(version || 'unknown');
      };

      this.postMessage({
        type: 'GET_VERSION'
      });

      // Fallback timeout
      setTimeout(() => resolve('unknown'), 2000);
    });
  }

  private handleUpdate(): void {
    if (!this.registration) return;

    const newWorker = this.registration.installing;
    if (!newWorker) return;

    newWorker.addEventListener('statechange', () => {
      if (newWorker.state === 'installed' && navigator.serviceWorker.controller) {
        console.log('Service Worker: New version available');
        this.showUpdateAvailable();
      }
    });
  }

  private showUpdateAvailable(): void {
    // Dispatch custom event for the app to handle
    const event = new CustomEvent('sw-update-available', {
      detail: {
        registration: this.registration
      }
    });
    window.dispatchEvent(event);
  }

  // Skip waiting and activate new service worker
  skipWaiting(): void {
    if (!this.registration?.waiting) {
      return;
    }

    this.registration.waiting.postMessage({ type: 'SKIP_WAITING' });
  }
}

// Singleton instance
export const serviceWorkerManager: ServiceWorkerManager = new ServiceWorkerManagerImpl();

// Auto-register service worker in production
export async function initServiceWorker(): Promise<void> {
  if (import.meta.env.PROD && serviceWorkerManager.isSupported()) {
    try {
      await serviceWorkerManager.register();
      console.log('Service Worker: Initialized successfully');
    } catch (error) {
      console.error('Service Worker: Initialization failed', error);
    }
  } else if (!serviceWorkerManager.isSupported()) {
    console.log('Service Worker: Skipped (not supported by browser)');
  } else {
    console.log('Service Worker: Skipped (development mode - service workers disabled for development)');
  }
}

// Utility functions for offline detection
export function isOnline(): boolean {
  return navigator.onLine;
}

export function onOnlineStatusChange(callback: (isOnline: boolean) => void): () => void {
  const handleOnline = () => callback(true);
  const handleOffline = () => callback(false);

  window.addEventListener('online', handleOnline);
  window.addEventListener('offline', handleOffline);

  // Return cleanup function
  return () => {
    window.removeEventListener('online', handleOnline);
    window.removeEventListener('offline', handleOffline);
  };
}

// Background sync utilities
export async function registerBackgroundSync(tag: string): Promise<void> {
  const registration = await serviceWorkerManager.getRegistration();
  if (registration && 'sync' in registration) {
    try {
      await (registration as any).sync.register(tag);
      console.log(`Background sync registered: ${tag}`);
    } catch (error) {
      console.error(`Background sync registration failed: ${tag}`, error);
    }
  }
}

// Performance monitoring
export interface PerformanceMetrics {
  loadTime: number;
  domContentLoaded: number;
  firstContentfulPaint: number;
  largestContentfulPaint: number;
  cumulativeLayoutShift: number;
  firstInputDelay: number;
}

export function getPerformanceMetrics(): PerformanceMetrics | null {
  if (!('performance' in window)) {
    return null;
  }

  const navigation = performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming;
  const paint = performance.getEntriesByType('paint');

  const fcp = paint.find(entry => entry.name === 'first-contentful-paint');
  
  return {
    loadTime: navigation.loadEventEnd - navigation.loadEventStart,
    domContentLoaded: navigation.domContentLoadedEventEnd - navigation.domContentLoadedEventStart,
    firstContentfulPaint: fcp ? fcp.startTime : 0,
    largestContentfulPaint: 0, // Would need PerformanceObserver for LCP
    cumulativeLayoutShift: 0, // Would need PerformanceObserver for CLS
    firstInputDelay: 0 // Would need PerformanceObserver for FID
  };
}

// Advanced performance monitoring with PerformanceObserver
export function startPerformanceMonitoring(): void {
  if (!('PerformanceObserver' in window)) {
    console.warn('PerformanceObserver not supported');
    return;
  }

  // Monitor LCP
  try {
    const lcpObserver = new PerformanceObserver((list) => {
      const entries = list.getEntries();
      const lastEntry = entries[entries.length - 1];
      if (lastEntry) {
        console.log('LCP:', (lastEntry as PerformanceEntry).startTime);
      }
    });
    lcpObserver.observe({ entryTypes: ['largest-contentful-paint'] });
  } catch (error) {
    console.warn('LCP monitoring failed:', error);
  }

  // Monitor FID
  try {
    const fidObserver = new PerformanceObserver((list) => {
      const entries = list.getEntries();
      entries.forEach((entry) => {
        const fidEntry = entry as PerformanceEntry & { processingStart?: number };
        if (fidEntry.processingStart !== undefined) {
          console.log('FID:', fidEntry.processingStart - fidEntry.startTime);
        }
      });
    });
    fidObserver.observe({ entryTypes: ['first-input'] });
  } catch (error) {
    console.warn('FID monitoring failed:', error);
  }

  // Monitor CLS
  try {
    let clsValue = 0;
    const clsObserver = new PerformanceObserver((list) => {
      const entries = list.getEntries();
      entries.forEach((entry: any) => {
        if (!entry.hadRecentInput) {
          clsValue += entry.value;
        }
      });
      console.log('CLS:', clsValue);
    });
    clsObserver.observe({ entryTypes: ['layout-shift'] });
  } catch (error) {
    console.warn('CLS monitoring failed:', error);
  }
}