/**
 * API Service Usage Examples
 * Demonstrates how to use the API service in components
 */

import { apiService } from './api.js';
import type { QueryRequest } from '../types/api.js';

/**
 * Example: Upload a PDF document
 */
export async function uploadDocumentExample(file: File) {
  try {
    const result = await apiService.uploadDocument(file);
    console.log('Upload successful:', result);
    return result;
  } catch (error) {
    console.error('Upload failed:', error);
    throw error;
  }
}

/**
 * Example: Query documents with basic configuration
 */
export async function queryDocumentsExample(question: string) {
  try {
    const request: QueryRequest = {
      question,
      config: {
        max_chunks: 10,
        similarity_threshold: 0.7,
        temperature: 0.3
      }
    };

    const result = await apiService.queryDocuments(request);
    console.log('Query successful:', result);
    return result;
  } catch (error) {
    console.error('Query failed:', error);
    throw error;
  }
}

/**
 * Example: Check system health
 */
export async function checkHealthExample() {
  try {
    const result = await apiService.getHealth();
    console.log('Health check successful:', result);
    return result;
  } catch (error) {
    console.error('Health check failed:', error);
    throw error;
  }
}

/**
 * Example: Advanced query with custom configuration
 */
export async function advancedQueryExample(question: string, maxChunks: number, threshold: number) {
  try {
    const request: QueryRequest = {
      question,
      config: {
        max_chunks: maxChunks,
        similarity_threshold: threshold,
        temperature: 0.2,
        include_low_confidence: false
      }
    };

    const result = await apiService.queryDocuments(request);
    console.log('Advanced query successful:', result);
    return result;
  } catch (error) {
    console.error('Advanced query failed:', error);
    throw error;
  }
}