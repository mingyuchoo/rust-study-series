/**
 * Local storage utilities with type safety and error handling
 */

export class LocalStorageManager {
  private static readonly PREFIX = 'rag-app-';

  // Generic get method with type safety
  static get<T>(key: string, defaultValue: T): T {
    try {
      const item = localStorage.getItem(this.PREFIX + key);
      if (item === null) return defaultValue;
      return JSON.parse(item) as T;
    } catch (error) {
      console.warn(`Failed to get item from localStorage: ${key}`, error);
      return defaultValue;
    }
  }

  // Generic set method
  static set<T>(key: string, value: T): boolean {
    try {
      localStorage.setItem(this.PREFIX + key, JSON.stringify(value));
      return true;
    } catch (error) {
      console.warn(`Failed to set item in localStorage: ${key}`, error);
      return false;
    }
  }

  // Remove item
  static remove(key: string): boolean {
    try {
      localStorage.removeItem(this.PREFIX + key);
      return true;
    } catch (error) {
      console.warn(`Failed to remove item from localStorage: ${key}`, error);
      return false;
    }
  }

  // Clear all app data
  static clear(): boolean {
    try {
      const keys = Object.keys(localStorage).filter(key => key.startsWith(this.PREFIX));
      keys.forEach(key => localStorage.removeItem(key));
      return true;
    } catch (error) {
      console.warn('Failed to clear localStorage', error);
      return false;
    }
  }

  // Check if localStorage is available
  static isAvailable(): boolean {
    try {
      const test = '__localStorage_test__';
      localStorage.setItem(test, 'test');
      localStorage.removeItem(test);
      return true;
    } catch {
      return false;
    }
  }

  // Get storage usage info
  static getStorageInfo(): { used: number; available: number; percentage: number } {
    if (!this.isAvailable()) {
      return { used: 0, available: 0, percentage: 0 };
    }

    try {
      let used = 0;
      for (let key in localStorage) {
        if (localStorage.hasOwnProperty(key) && key.startsWith(this.PREFIX)) {
          used += localStorage[key].length + key.length;
        }
      }

      // Estimate available space (most browsers have ~5-10MB limit)
      const estimated = 5 * 1024 * 1024; // 5MB
      const percentage = (used / estimated) * 100;

      return {
        used,
        available: estimated - used,
        percentage: Math.min(percentage, 100)
      };
    } catch (error) {
      console.warn('Failed to get storage info', error);
      return { used: 0, available: 0, percentage: 0 };
    }
  }
}

// Specific storage keys for the application
export const STORAGE_KEYS = {
  SEARCH_HISTORY: 'search-history',
  SEARCH_CONFIG: 'search-config',
  UI_PREFERENCES: 'ui-preferences',
  BOOKMARKED_RESPONSES: 'bookmarked-responses'
} as const;