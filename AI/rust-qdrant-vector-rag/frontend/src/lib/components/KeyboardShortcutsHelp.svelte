<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from 'svelte';
  import { X, Keyboard, Command } from 'lucide-svelte';
  import { FocusManager } from '../utils/accessibility.js';

  import type { KeyboardShortcut } from '../utils/keyboard-shortcuts.js';

  // Props
  export let visible = false;
  export let shortcuts: KeyboardShortcut[] = [];

  // Event dispatcher
  const dispatch = createEventDispatcher<{
    close: void;
  }>();

  // Local state
  let dialogElement: HTMLElement;
  let previousFocus: HTMLElement | null = null;

  // Close dialog
  function closeDialog(): void {
    visible = false;
    dispatch('close');
  }

  // Handle keyboard events
  function handleKeyDown(event: KeyboardEvent): void {
    if (event.key === 'Escape') {
      event.preventDefault();
      closeDialog();
    } else if (event.key === 'Tab' && dialogElement) {
      FocusManager.trapFocus(dialogElement, event);
    }
  }

  // Handle backdrop click
  function handleBackdropClick(event: MouseEvent): void {
    if (event.target === event.currentTarget) {
      closeDialog();
    }
  }

  // Format keyboard shortcut for display
  function formatShortcut(shortcut: KeyboardShortcut): string {
    const parts = [];
    const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
    
    if (shortcut.ctrlKey) {
      parts.push(isMac ? '⌘' : 'Ctrl');
    }
    if (shortcut.altKey) {
      parts.push(isMac ? '⌥' : 'Alt');
    }
    if (shortcut.shiftKey) {
      parts.push(isMac ? '⇧' : 'Shift');
    }
    if (shortcut.metaKey) {
      parts.push(isMac ? '⌘' : 'Meta');
    }
    
    // Format the key
    let key = shortcut.key;
    if (key === ' ') key = 'Space';
    else if (key === 'Escape') key = 'Esc';
    else if (key === 'ArrowUp') key = '↑';
    else if (key === 'ArrowDown') key = '↓';
    else if (key === 'ArrowLeft') key = '←';
    else if (key === 'ArrowRight') key = '→';
    else if (key.length === 1) key = key.toUpperCase();
    
    parts.push(key);
    return parts.join(' + ');
  }

  // Group shortcuts by category
  function groupShortcuts(shortcuts: KeyboardShortcut[]): Record<string, KeyboardShortcut[]> {
    const groups: Record<string, KeyboardShortcut[]> = {
      'Search': [],
      'Navigation': [],
      'General': []
    };

    shortcuts.forEach(shortcut => {
      if (shortcut.description.toLowerCase().includes('search')) {
        groups['Search'].push(shortcut);
      } else if (shortcut.description.toLowerCase().includes('focus') || 
                 shortcut.description.toLowerCase().includes('toggle')) {
        groups['Navigation'].push(shortcut);
      } else {
        groups['General'].push(shortcut);
      }
    });

    // Remove empty groups
    Object.keys(groups).forEach(key => {
      if ((groups[key]?.length ?? 0) === 0) {
        delete groups[key];
      }
    });

    return groups;
  }

  $: groupedShortcuts = groupShortcuts(shortcuts);

  // Manage focus when dialog opens/closes
  $: if (visible) {
    previousFocus = document.activeElement as HTMLElement;
    // Focus the dialog after it's rendered
    setTimeout(() => {
      if (dialogElement) {
        const firstFocusable = FocusManager.getFirstFocusableElement(dialogElement);
        if (firstFocusable) {
          firstFocusable.focus();
        }
      }
    }, 0);
  } else if (previousFocus) {
    FocusManager.restoreFocus(previousFocus);
    previousFocus = null;
  }

  onMount(() => {
    document.addEventListener('keydown', handleKeyDown);
  });

  onDestroy(() => {
    document.removeEventListener('keydown', handleKeyDown);
  });
</script>

{#if visible}
  <!-- Backdrop -->
  <div 
    class="shortcuts-backdrop"
    on:click={handleBackdropClick}
    role="presentation"
  >
    <!-- Dialog -->
    <div
      bind:this={dialogElement}
      class="shortcuts-dialog"
      role="dialog"
      aria-labelledby="shortcuts-title"
      aria-modal="true"
    >
      <!-- Header -->
      <div class="shortcuts-header">
        <div class="shortcuts-title-section">
          <Keyboard size={24} class="shortcuts-icon" aria-hidden="true" />
          <h2 id="shortcuts-title" class="shortcuts-title">
            Keyboard Shortcuts
          </h2>
        </div>
        
        <button
          type="button"
          class="close-button"
          on:click={closeDialog}
          aria-label="Close keyboard shortcuts help"
        >
          <X size={20} />
        </button>
      </div>

      <!-- Content -->
      <div class="shortcuts-content">
        {#if Object.keys(groupedShortcuts).length === 0}
          <div class="no-shortcuts">
            <p>No keyboard shortcuts are currently available.</p>
          </div>
        {:else}
          {#each Object.entries(groupedShortcuts) as [category, categoryShortcuts]}
            <div class="shortcuts-category">
              <h3 class="category-title">{category}</h3>
              <div class="shortcuts-list">
                {#each categoryShortcuts as shortcut}
                  <div class="shortcut-item">
                    <div class="shortcut-keys">
                      {#each formatShortcut(shortcut).split(' + ') as key, index}
                        {#if index > 0}
                          <span class="key-separator">+</span>
                        {/if}
                        <kbd class="key">{key}</kbd>
                      {/each}
                    </div>
                    <div class="shortcut-description">
                      {shortcut.description}
                    </div>
                  </div>
                {/each}
              </div>
            </div>
          {/each}
        {/if}
      </div>

      <!-- Footer -->
      <div class="shortcuts-footer">
        <p class="shortcuts-note">
          <Command size={16} aria-hidden="true" />
          Press <kbd class="key">?</kbd> to show/hide this help
        </p>
      </div>
    </div>
  </div>
{/if}

<style>
  .shortcuts-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 1rem;
  }

  .shortcuts-dialog {
    background: white;
    border-radius: 0.75rem;
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
    max-width: 600px;
    width: 100%;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .shortcuts-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.5rem;
    border-bottom: 1px solid #e5e7eb;
    background: #f9fafb;
  }

  .shortcuts-title-section {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .shortcuts-title {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #111827;
  }

  .close-button {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem;
    background: none;
    border: none;
    border-radius: 0.375rem;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .close-button:hover {
    background: #e5e7eb;
    color: #374151;
  }

  .close-button:focus {
    outline: 2px solid #3b82f6;
    outline-offset: 2px;
  }

  .shortcuts-content {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem;
  }

  .no-shortcuts {
    text-align: center;
    padding: 2rem;
    color: #6b7280;
  }

  .shortcuts-category {
    margin-bottom: 2rem;
  }

  .shortcuts-category:last-child {
    margin-bottom: 0;
  }

  .category-title {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: #374151;
    border-bottom: 1px solid #e5e7eb;
    padding-bottom: 0.5rem;
  }

  .shortcuts-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .shortcut-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.75rem;
    background: #f9fafb;
    border-radius: 0.5rem;
    border: 1px solid #e5e7eb;
  }

  .shortcut-keys {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    flex-shrink: 0;
  }

  .key {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 1.5rem;
    height: 1.5rem;
    padding: 0 0.375rem;
    background: white;
    border: 1px solid #d1d5db;
    border-radius: 0.25rem;
    font-family: ui-monospace, SFMono-Regular, 'SF Mono', Consolas, 'Liberation Mono', Menlo, monospace;
    font-size: 0.75rem;
    font-weight: 500;
    color: #374151;
    box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
  }

  .key-separator {
    color: #9ca3af;
    font-size: 0.75rem;
    margin: 0 0.125rem;
  }

  .shortcut-description {
    flex: 1;
    font-size: 0.875rem;
    color: #4b5563;
    text-align: right;
  }

  .shortcuts-footer {
    padding: 1rem 1.5rem;
    border-top: 1px solid #e5e7eb;
    background: #f9fafb;
  }

  .shortcuts-note {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    margin: 0;
    font-size: 0.75rem;
    color: #6b7280;
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .shortcuts-dialog {
      background: #1f2937;
    }

    .shortcuts-header {
      background: #111827;
      border-bottom-color: #374151;
    }

    .shortcuts-title {
      color: #f9fafb;
    }

    :global(.shortcuts-icon) {
      color: #60a5fa;
    }

    .close-button {
      color: #9ca3af;
    }

    .close-button:hover {
      background: #374151;
      color: #f3f4f6;
    }

    .category-title {
      color: #f3f4f6;
      border-bottom-color: #374151;
    }

    .shortcut-item {
      background: #111827;
      border-color: #374151;
    }

    .key {
      background: #374151;
      border-color: #4b5563;
      color: #f3f4f6;
    }

    .shortcut-description {
      color: #d1d5db;
    }

    .shortcuts-footer {
      background: #111827;
      border-top-color: #374151;
    }

    .shortcuts-note {
      color: #9ca3af;
    }

    .key-separator {
      color: #6b7280;
    }
  }

  /* Mobile responsive */
  @media (max-width: 768px) {
    .shortcuts-backdrop {
      padding: 0.5rem;
    }

    .shortcuts-dialog {
      max-height: 90vh;
    }

    .shortcuts-header {
      padding: 1rem;
    }

    .shortcuts-content {
      padding: 1rem;
    }

    .shortcuts-footer {
      padding: 0.75rem 1rem;
    }

    .shortcut-item {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.5rem;
      padding: 1rem;
    }

    .shortcut-description {
      text-align: left;
      width: 100%;
    }

    .shortcuts-title {
      font-size: 1.125rem;
    }

    .close-button {
      min-height: 44px;
      min-width: 44px;
    }
  }

  /* High contrast mode */
  @media (prefers-contrast: high) {
    .shortcuts-dialog {
      border: 2px solid #000;
    }

    .shortcut-item {
      border-width: 2px;
    }

    .key {
      border-width: 2px;
      border-color: #000;
    }

    .close-button:focus {
      outline-width: 3px;
    }
  }

  /* Reduced motion */
  @media (prefers-reduced-motion: reduce) {
    .close-button {
      transition: none;
    }
  }

  /* Print styles */
  @media print {
    .shortcuts-backdrop {
      position: static;
      background: none;
      backdrop-filter: none;
    }

    .shortcuts-dialog {
      box-shadow: none;
      max-height: none;
    }

    .close-button {
      display: none;
    }
  }
</style>