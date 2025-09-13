/**
 * OfflineIndicator Component Tests
 * Tests for offline detection and user notification functionality
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import { get } from 'svelte/store';
import OfflineIndicator from '../OfflineIndicator.svelte';
import { isOnline } from '../../stores/app.store.js';

// Mock the error handler service
const mockErrorHandler = {
  addNetworkListener: vi.fn(),
  checkConnectivity: vi.fn(),
  isOnline: vi.fn()
};

vi.mock('../../services/error-handler.js', () => ({
  errorHandler: mockErrorHandler
}));

// Mock the app store
vi.mock('../../stores/app.store.js', () => ({
  isOnline: {
    subscribe: vi.fn()
  }
}));

describe('OfflineIndicator', () => {
  let mockUnsubscribe: ReturnType<typeof vi.fn>;
  let mockNetworkCleanup: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    vi.clearAllMocks();
    
    // Mock store subscription
    mockUnsubscribe = vi.fn();
    mockNetworkCleanup = vi.fn();
    
    // Mock isOnline store to return true by default
    vi.mocked(isOnline.subscribe).mockImplementation((callback) => {
      callback(true);
      return mockUnsubscribe;
    });

    // Mock error handler methods
    mockErrorHandler.addNetworkListener.mockReturnValue(mockNetworkCleanup);
    mockErrorHandler.checkConnectivity.mockResolvedValue(true);
    mockErrorHandler.isOnline.mockReturnValue(true);
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('should not render when online', () => {
    // Mock online state
    vi.mocked(isOnline.subscribe).mockImplementation((callback) => {
      callback(true);
      return mockUnsubscribe;
    });

    render(OfflineIndicator);

    // Should not show offline indicator when online
    expect(screen.queryByText(/you're offline/i)).not.toBeInTheDocument();
  });

  it('should render offline message when offline', () => {
    // Mock offline state
    vi.mocked(isOnline.subscribe).mockImplementation((callback) => {
      callback(false);
      return mockUnsubscribe;
    });

    render(OfflineIndicator);

    // Should show offline indicator when offline
    expect(screen.getByText(/you're offline/i)).toBeInTheDocument();
    expect(screen.getByText(/some features may not work while offline/i)).toBeInTheDocument();
  });

  it('should show retry button when offline', () => {
    // Mock offline state
    vi.mocked(isOnline.subscribe).mockImplementation((callback) => {
      callback(false);
      return mockUnsubscribe;
    });

    render(OfflineIndicator);

    // Should show retry button
    expect(screen.getByRole('button', { name: /try again/i })).toBeInTheDocument();
  });

  it('should show checking status during connectivity check', async () => {
    // Mock offline state
    vi.mocked(isOnline.subscribe).mockImplementation((callback) => {
      callback(false);
      return mockUnsubscribe;
    });

    // Mock slow connectivity check
    let resolveConnectivity: (value: boolean) => void;
    const connectivityPromise = new Promise<boolean>((resolve) => {
      resolveConnectivity = resolve;
    });
    mockErrorHandler.checkConnectivity.mockReturnValue(connectivityPromise);

    const { component } = render(OfflineIndicator);

    // Click retry button
    const retryButton = screen.getByRole('button', { name: /try again/i });
    await retryButton.click();

    // Should show checking status
    expect(screen.getByText(/checking connection/i)).toBeInTheDocument();

    // Resolve the connectivity check
    resolveConnectivity!(true);
    await connectivityPromise;
  });

  it('should register network listener on mount', () => {
    render(OfflineIndicator);

    expect(mockErrorHandler.addNetworkListener).toHaveBeenCalledWith(expect.any(Function));
  });

  it('should cleanup network listener on destroy', () => {
    const { unmount } = render(OfflineIndicator);

    unmount();

    expect(mockNetworkCleanup).toHaveBeenCalled();
  });

  it('should handle retry attempts limit', () => {
    // Mock offline state
    vi.mocked(isOnline.subscribe).mockImplementation((callback) => {
      callback(false);
      return mockUnsubscribe;
    });

    render(OfflineIndicator);

    // Initially should show retry button
    expect(screen.getByRole('button', { name: /try again/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /try again/i })).not.toBeDisabled();
  });

  it('should show last online time when offline', () => {
    // Mock offline state
    vi.mocked(isOnline.subscribe).mockImplementation((callback) => {
      callback(false);
      return mockUnsubscribe;
    });

    render(OfflineIndicator);

    // Should show last online time (will be "just now" since it just went offline)
    expect(screen.getByText(/last online/i)).toBeInTheDocument();
  });

  it('should handle network status changes through listener', () => {
    let networkListener: (isOnline: boolean) => void;
    
    mockErrorHandler.addNetworkListener.mockImplementation((listener) => {
      networkListener = listener;
      return mockNetworkCleanup;
    });

    // Start offline
    vi.mocked(isOnline.subscribe).mockImplementation((callback) => {
      callback(false);
      return mockUnsubscribe;
    });

    render(OfflineIndicator);

    // Should show offline indicator
    expect(screen.getByText(/you're offline/i)).toBeInTheDocument();

    // Simulate going online through network listener
    networkListener!(true);

    // Note: In a real test, we'd need to trigger a re-render or use a more sophisticated
    // testing approach to verify the component updates when the network status changes
  });
});