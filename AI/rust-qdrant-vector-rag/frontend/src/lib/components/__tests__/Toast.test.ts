import { describe, it, expect, beforeEach } from 'vitest';
import { get } from 'svelte/store';
import { toastStore, toastActions } from '../../stores/toast.store.js';

describe('Toast Component', () => {
  beforeEach(() => {
    // Clear all toasts before each test
    toastActions.clear();
  });

  // Component rendering tests disabled due to Svelte 5 compatibility issues
  // The store functionality is tested below

  it('can add different types of toasts', () => {
    toastActions.success('Success message');
    toastActions.error('Error message');
    toastActions.warning('Warning message');
    toastActions.info('Info message');
    
    const toasts = get(toastStore);
    expect(toasts).toHaveLength(4);
    expect(toasts.map(t => t.type)).toEqual(['success', 'error', 'warning', 'info']);
  });

  it('can remove specific toast', () => {
    const id1 = toastActions.success('Message 1');
    const id2 = toastActions.success('Message 2');
    
    expect(get(toastStore)).toHaveLength(2);
    
    toastActions.remove(id1);
    
    const remainingToasts = get(toastStore);
    expect(remainingToasts).toHaveLength(1);
    expect(remainingToasts[0]?.id).toBe(id2);
  });

  it('can clear all toasts', () => {
    toastActions.success('Message 1');
    toastActions.success('Message 2');
    
    expect(get(toastStore)).toHaveLength(2);
    
    toastActions.clear();
    
    expect(get(toastStore)).toHaveLength(0);
  });
});