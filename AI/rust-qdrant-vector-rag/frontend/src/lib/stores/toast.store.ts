/**
 * Toast Notification Store
 * Manages toast messages for user feedback
 */

import { writable, derived } from 'svelte/store';
import type { ToastMessage } from '../types/state.js';

// Create the writable store
export const toastStore = writable<ToastMessage[]>([]);

// Derived stores
export const activeToasts = derived(toastStore, $toasts => $toasts);
export const toastCount = derived(toastStore, $toasts => $toasts.length);
export const hasToasts = derived(toastStore, $toasts => $toasts.length > 0);
export const errorToasts = derived(toastStore, $toasts => $toasts.filter(t => t.type === 'error'));
export const successToasts = derived(toastStore, $toasts => $toasts.filter(t => t.type === 'success'));
export const warningToasts = derived(toastStore, $toasts => $toasts.filter(t => t.type === 'warning'));
export const infoToasts = derived(toastStore, $toasts => $toasts.filter(t => t.type === 'info'));

// Toast queue and utilities
export const toastQueue = writable<ToastMessage[]>([]);
export const toastUtils = {
  getToastById: (id: string) => derived(toastStore, $toasts => $toasts.find(t => t.id === id)),
  getToastsByType: (type: string) => derived(toastStore, $toasts => $toasts.filter(t => t.type === type))
};

// Toast actions
export const toastActions = {
  // Add a new toast
  add: (toast: Omit<ToastMessage, 'id'>): string => {
    const id = `toast_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    const newToast: ToastMessage = {
      id,
      duration: 5000,
      dismissible: true,
      ...toast
    };

    toastStore.update(toasts => [...toasts, newToast]);

    // Auto-dismiss if duration is set
    if (newToast.duration && newToast.duration > 0) {
      window.setTimeout(() => {
        toastActions.remove(id);
      }, newToast.duration);
    }

    return id;
  },

  // Remove a specific toast
  remove: (id: string): void => {
    toastStore.update(toasts => toasts.filter(toast => toast.id !== id));
  },

  // Clear all toasts
  clear: (): void => {
    toastStore.set([]);
  },

  // Convenience methods for different toast types
  success: (message: string, options?: Partial<Omit<ToastMessage, 'id' | 'type' | 'message'>>): string => {
    return toastActions.add({ type: 'success', message, ...options });
  },

  error: (message: string, options?: Partial<Omit<ToastMessage, 'id' | 'type' | 'message'>>): string => {
    return toastActions.add({ 
      type: 'error', 
      message, 
      duration: 0, // Error toasts don't auto-dismiss by default
      ...options 
    });
  },

  warning: (message: string, options?: Partial<Omit<ToastMessage, 'id' | 'type' | 'message'>>): string => {
    return toastActions.add({ type: 'warning', message, ...options });
  },

  info: (message: string, options?: Partial<Omit<ToastMessage, 'id' | 'type' | 'message'>>): string => {
    return toastActions.add({ type: 'info', message, ...options });
  }
};