/**
 * Upload State Store
 * Manages file upload state, progress, and results
 */

import { writable, derived } from 'svelte/store';
import { errorHandler } from '../services/error-handler.js';
import { toastActions } from './toast.store.js';
import type { UploadState } from '../types/state.js';
import type { UploadResponse } from '../types/api.js';
import type { AppError, UploadError } from '../types/errors.js';

// Initial state
const initialUploadState: UploadState = {
  uploadProgress: 0,
  isUploading: false,
  uploadResult: null,
  selectedFile: null,
  dragActive: false
};

// Create the writable store
export const uploadStore = writable<UploadState>(initialUploadState);

// Derived stores for specific state slices
export const uploadProgress = derived(uploadStore, ($upload) => $upload.uploadProgress);
export const isUploading = derived(uploadStore, ($upload) => $upload.isUploading);
export const uploadResult = derived(uploadStore, ($upload) => $upload.uploadResult);
export const selectedFile = derived(uploadStore, ($upload) => $upload.selectedFile);
export const dragActive = derived(uploadStore, ($upload) => $upload.dragActive);

// Derived computed values
export const uploadComplete = derived(uploadStore, ($upload) => 
  $upload.uploadProgress === 100 && !$upload.isUploading
);

export const hasUploadError = derived(uploadStore, ($upload) => 
  $upload.uploadResult?.status === 'failure'
);

export const uploadSuccess = derived(uploadStore, ($upload) => 
  $upload.uploadResult?.status === 'success'
);

// Store actions
export const uploadActions = {
  setSelectedFile: (file: File | null) => {
    uploadStore.update(state => ({ 
      ...state, 
      selectedFile: file,
      uploadResult: null, // Clear previous results when new file selected
      uploadProgress: 0
    }));
  },

  setDragActive: (active: boolean) => {
    uploadStore.update(state => ({ ...state, dragActive: active }));
  },

  startUpload: () => {
    uploadStore.update(state => ({ 
      ...state, 
      isUploading: true, 
      uploadProgress: 0,
      uploadResult: null
    }));
  },

  updateProgress: (progress: number) => {
    uploadStore.update(state => ({ 
      ...state, 
      uploadProgress: Math.min(100, Math.max(0, progress))
    }));
  },

  setUploadResult: (result: UploadResponse) => {
    uploadStore.update(state => ({ 
      ...state, 
      uploadResult: result,
      isUploading: false,
      uploadProgress: result.status === 'success' ? 100 : state.uploadProgress
    }));
  },

  completeUpload: (result: UploadResponse) => {
    uploadStore.update(state => ({ 
      ...state, 
      uploadResult: result,
      isUploading: false,
      uploadProgress: 100
    }));
  },

  failUpload: (error: string | AppError, retryAction?: () => Promise<void>) => {
    let errorMessage = '';
    let errorResult: UploadResponse;

    if (typeof error === 'string') {
      errorMessage = error;
    } else {
      // Handle AppError with comprehensive error handling
      const errorContext = errorHandler.handleError(error, retryAction, false);
      errorMessage = errorContext.userMessage;
      
      // Show toast with recovery options for upload errors
      if (error.type === 'upload_error') {
        const uploadError = error as UploadError;
        if (uploadError.retryable && retryAction) {
          toastActions.error(errorMessage, {
            duration: 0, // Don't auto-dismiss
            dismissible: true
          });
        } else {
          toastActions.error(errorMessage);
        }
      }
    }

    errorResult = {
      document_id: '',
      filename: '',
      chunks_created: 0,
      processing_time_ms: 0,
      status: 'failure',
      message: errorMessage,
      timestamp: new Date().toISOString()
    };
    
    uploadStore.update(state => ({ 
      ...state, 
      uploadResult: errorResult,
      isUploading: false
    }));
  },

  reset: () => {
    uploadStore.set(initialUploadState);
  },

  clearResult: () => {
    uploadStore.update(state => ({ 
      ...state, 
      uploadResult: null,
      uploadProgress: 0
    }));
  }
};