/**
 * Keyboard shortcuts utility for common actions
 */

export interface KeyboardShortcut {
  key: string;
  ctrlKey?: boolean;
  altKey?: boolean;
  shiftKey?: boolean;
  metaKey?: boolean;
  description: string;
  action: () => void;
}

export class KeyboardShortcutManager {
  private shortcuts: Map<string, KeyboardShortcut> = new Map();
  private isEnabled = true;

  constructor() {
    this.handleKeyDown = this.handleKeyDown.bind(this);
    document.addEventListener('keydown', this.handleKeyDown);
  }

  // Register a keyboard shortcut
  register(shortcut: KeyboardShortcut): void {
    const key = this.getShortcutKey(shortcut);
    this.shortcuts.set(key, shortcut);
  }

  // Unregister a keyboard shortcut
  unregister(shortcut: Partial<KeyboardShortcut>): void {
    const key = this.getShortcutKey(shortcut);
    this.shortcuts.delete(key);
  }

  // Enable/disable all shortcuts
  setEnabled(enabled: boolean): void {
    this.isEnabled = enabled;
  }

  // Get all registered shortcuts
  getShortcuts(): KeyboardShortcut[] {
    return Array.from(this.shortcuts.values());
  }

  // Generate a unique key for the shortcut
  private getShortcutKey(shortcut: Partial<KeyboardShortcut>): string {
    const parts = [];
    if (shortcut.ctrlKey) parts.push('ctrl');
    if (shortcut.altKey) parts.push('alt');
    if (shortcut.shiftKey) parts.push('shift');
    if (shortcut.metaKey) parts.push('meta');
    parts.push(shortcut.key?.toLowerCase() || '');
    return parts.join('+');
  }

  // Handle keydown events
  private handleKeyDown(event: KeyboardEvent): void {
    if (!this.isEnabled) return;

    // Don't trigger shortcuts when typing in form elements
    const target = event.target as HTMLElement;
    if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.contentEditable === 'true') {
      return;
    }

    const shortcutKey = this.getShortcutKey({
      key: event.key,
      ctrlKey: event.ctrlKey,
      altKey: event.altKey,
      shiftKey: event.shiftKey,
      metaKey: event.metaKey
    });

    const shortcut = this.shortcuts.get(shortcutKey);
    if (shortcut) {
      event.preventDefault();
      shortcut.action();
    }
  }

  // Cleanup
  destroy(): void {
    document.removeEventListener('keydown', this.handleKeyDown);
    this.shortcuts.clear();
  }
}

// Default keyboard shortcuts for the application
export const DEFAULT_SHORTCUTS = {
  SEARCH_FOCUS: { key: '/', description: 'Focus search input' },
  NEW_SEARCH: { key: 'n', ctrlKey: true, description: 'Start new search' },
  TOGGLE_ADVANCED: { key: 'a', ctrlKey: true, description: 'Toggle advanced options' },
  COPY_ANSWER: { key: 'c', ctrlKey: true, shiftKey: true, description: 'Copy AI answer' },
  CLEAR_SEARCH: { key: 'Escape', description: 'Clear search results' },
  HELP: { key: '?', description: 'Show keyboard shortcuts help' }
} as const;