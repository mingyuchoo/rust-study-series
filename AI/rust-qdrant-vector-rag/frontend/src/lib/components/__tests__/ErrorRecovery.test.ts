/**
 * ErrorRecovery Component Tests
 * Tests for error recovery UI and user interactions
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import ErrorRecovery from '../ErrorRecovery.svelte';
import { ErrorTypeValues, ErrorSeverityValues } from '../../types/errors.js';
import type { ErrorContext, ErrorRecovery as ErrorRecoveryType } from '../../types/errors.js';

describe('ErrorRecovery', () => {
  let mockErrorContext: ErrorContext;
  let mockRecoveryActions: ErrorRecoveryType[];

  beforeEach(() => {
    vi.clearAllMocks();

    // Create mock recovery actions
    mockRecoveryActions = [
      {
        action: 'retry',
        label: 'Try Again',
        handler: vi.fn()
      },
      {
        action: 'refresh',
        label: 'Refresh Page',
        handler: vi.fn()
      },
      {
        action: 'navigate_home',
        label: 'Go Home',
        handler: vi.fn()
      },
      {
        action: 'dismiss',
        label: 'Dismiss',
        handler: vi.fn()
      }
    ];

    // Create mock error context
    mockErrorContext = {
      error: {
        type: ErrorTypeValues.NETWORK_ERROR,
        message: 'Network connection failed',
        retryable: true,
        severity: ErrorSeverityValues.HIGH,
        timestamp: new Date('2024-01-01T12:00:00Z')
      },
      recoveryOptions: mockRecoveryActions,
      userMessage: 'Unable to connect to the server. Please check your internet connection.',
      technicalMessage: 'Network connection failed'
    };
  });

  it('should render error message and recovery options', () => {
    render(ErrorRecovery, { errorContext: mockErrorContext });

    // Should show user-friendly error message
    expect(screen.getByText('Unable to connect to the server. Please check your internet connection.')).toBeInTheDocument();

    // Should show recovery action buttons
    expect(screen.getByRole('button', { name: /try again/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /refresh page/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /go home/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /dismiss/i })).toBeInTheDocument();
  });

  it('should display appropriate error title based on severity', () => {
    // Test critical error
    const criticalErrorContext = {
      ...mockErrorContext,
      error: {
        ...mockErrorContext.error,
        severity: ErrorSeverityValues.CRITICAL
      }
    };

    const { unmount } = render(ErrorRecovery, { errorContext: criticalErrorContext });
    expect(screen.getByText('Critical Error')).toBeInTheDocument();
    unmount();

    // Test high severity error
    const highErrorContext = {
      ...mockErrorContext,
      error: {
        ...mockErrorContext.error,
        severity: ErrorSeverityValues.HIGH
      }
    };

    render(ErrorRecovery, { errorContext: highErrorContext });
    expect(screen.getByText('Error Occurred')).toBeInTheDocument();
  });

  it('should show error metadata in non-compact mode', () => {
    render(ErrorRecovery, { 
      errorContext: mockErrorContext,
      compact: false 
    });

    // Should show error type and timestamp
    expect(screen.getByText('network error')).toBeInTheDocument();
    expect(screen.getByText(/2024/)).toBeInTheDocument(); // Part of timestamp
  });

  it('should hide metadata in compact mode', () => {
    render(ErrorRecovery, { 
      errorContext: mockErrorContext,
      compact: true 
    });

    // Should not show error metadata in compact mode
    expect(screen.queryByText('network error')).not.toBeInTheDocument();
  });

  it('should show dismiss button in non-compact mode', () => {
    render(ErrorRecovery, { 
      errorContext: mockErrorContext,
      compact: false 
    });

    // Should show dismiss button in header
    expect(screen.getByLabelText('Dismiss error')).toBeInTheDocument();
  });

  it('should hide dismiss button in compact mode', () => {
    render(ErrorRecovery, { 
      errorContext: mockErrorContext,
      compact: true 
    });

    // Should not show dismiss button in header for compact mode
    expect(screen.queryByLabelText('Dismiss error')).not.toBeInTheDocument();
  });

  it('should execute recovery action when button is clicked', async () => {
    const { component } = render(ErrorRecovery, { errorContext: mockErrorContext });

    // Mock event listener for recovery action
    const recoverySpy = vi.fn();
    component.$on('recover', recoverySpy);

    // Click retry button
    const retryButton = screen.getByRole('button', { name: /try again/i });
    await fireEvent.click(retryButton);

    // Should execute the handler
    expect(mockRecoveryActions[0].handler).toHaveBeenCalled();

    // Should dispatch recover event
    expect(recoverySpy).toHaveBeenCalledWith(
      expect.objectContaining({
        detail: { action: 'retry' }
      })
    );
  });

  it('should handle dismiss action', async () => {
    const { component } = render(ErrorRecovery, { 
      errorContext: mockErrorContext,
      compact: false 
    });

    // Mock event listener for dismiss action
    const dismissSpy = vi.fn();
    component.$on('dismiss', dismissSpy);

    // Click dismiss button in header
    const dismissButton = screen.getByLabelText('Dismiss error');
    await fireEvent.click(dismissButton);

    // Should dispatch dismiss event
    expect(dismissSpy).toHaveBeenCalled();
  });

  it('should show technical details when enabled', () => {
    render(ErrorRecovery, { 
      errorContext: mockErrorContext,
      showTechnicalDetails: true 
    });

    // Should show technical details section
    expect(screen.getByText('Technical Details')).toBeInTheDocument();
  });

  it('should hide technical details by default', () => {
    render(ErrorRecovery, { 
      errorContext: mockErrorContext,
      showTechnicalDetails: false 
    });

    // Should not show technical details section
    expect(screen.queryByText('Technical Details')).not.toBeInTheDocument();
  });

  it('should expand technical details when clicked', async () => {
    render(ErrorRecovery, { 
      errorContext: mockErrorContext,
      showTechnicalDetails: true 
    });

    // Click to expand technical details
    const detailsToggle = screen.getByText('Technical Details');
    await fireEvent.click(detailsToggle);

    // Should show technical information
    expect(screen.getByText('Error Type:')).toBeInTheDocument();
    expect(screen.getByText('Severity:')).toBeInTheDocument();
    expect(screen.getByText('Retryable:')).toBeInTheDocument();
    expect(screen.getByText('Timestamp:')).toBeInTheDocument();
  });

  it('should show different technical message when different from user message', () => {
    const contextWithDifferentMessages = {
      ...mockErrorContext,
      technicalMessage: 'Technical error: Connection timeout after 30s'
    };

    render(ErrorRecovery, { 
      errorContext: contextWithDifferentMessages,
      showTechnicalDetails: true 
    });

    // Expand technical details
    const detailsToggle = screen.getByText('Technical Details');
    fireEvent.click(detailsToggle);

    // Should show both user and technical messages
    expect(screen.getByText('Technical error: Connection timeout after 30s')).toBeInTheDocument();
  });

  it('should handle recovery action failure gracefully', async () => {
    // Mock a failing recovery action
    const failingRecoveryAction: ErrorRecoveryType = {
      action: 'retry',
      label: 'Try Again',
      handler: vi.fn().mockRejectedValue(new Error('Recovery failed'))
    };

    const errorContextWithFailingAction = {
      ...mockErrorContext,
      recoveryOptions: [failingRecoveryAction]
    };

    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(ErrorRecovery, { errorContext: errorContextWithFailingAction });

    // Click retry button
    const retryButton = screen.getByRole('button', { name: /try again/i });
    await fireEvent.click(retryButton);

    // Should log the error
    expect(consoleSpy).toHaveBeenCalledWith('Recovery action failed:', expect.any(Error));

    consoleSpy.mockRestore();
  });

  it('should apply correct CSS classes based on error severity', () => {
    render(ErrorRecovery, { errorContext: mockErrorContext });

    // Should have high severity class
    const errorIcon = document.querySelector('.severity-high');
    expect(errorIcon).toBeInTheDocument();
  });

  it('should render with no recovery options', () => {
    const errorContextWithoutRecovery = {
      ...mockErrorContext,
      recoveryOptions: []
    };

    render(ErrorRecovery, { errorContext: errorContextWithoutRecovery });

    // Should still render error message
    expect(screen.getByText('Unable to connect to the server. Please check your internet connection.')).toBeInTheDocument();

    // Should not show recovery actions section
    expect(screen.queryByRole('button')).not.toBeInTheDocument();
  });
});