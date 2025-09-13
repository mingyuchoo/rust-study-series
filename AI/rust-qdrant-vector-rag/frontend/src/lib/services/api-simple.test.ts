/**
 * Simple API Service Tests
 * Basic tests to verify core functionality
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { apiService } from './api.js';
import type { UploadResponse, RAGResponse, HealthResponse } from '../types/api.js';

// Mock fetch globally
const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('ApiService - Core Functionality', () => {
    beforeEach(() => {
        vi.clearAllMocks();
        mockFetch.mockClear();
    });

    it('should create API service instance', () => {
        expect(apiService).toBeDefined();
        expect(typeof apiService.uploadDocument).toBe('function');
        expect(typeof apiService.queryDocuments).toBe('function');
        expect(typeof apiService.getHealth).toBe('function');
    });

    it('should make successful upload request', async () => {
        const mockFile = new File(['test content'], 'test.pdf', { type: 'application/pdf' });
        const mockResponse: UploadResponse = {
            document_id: 'doc-123',
            filename: 'test.pdf',
            chunks_created: 5,
            processing_time_ms: 1500,
            status: 'success',
            message: 'Document uploaded successfully',
            timestamp: '2024-01-01T00:00:00Z'
        };

        mockFetch.mockResolvedValueOnce({
            ok: true,
            status: 200,
            statusText: 'OK',
            headers: new Headers({ 'content-type': 'application/json' }),
            json: async () => mockResponse
        });

        const result = await apiService.uploadDocument(mockFile);

        expect(result).toEqual(mockResponse);
        expect(mockFetch).toHaveBeenCalledTimes(1);

        const [url, options] = mockFetch.mock.calls[0] as [string, RequestInit];
        expect(url).toContain('/upload');
        expect(options.method).toBe('POST');
        expect(options.body).toBeInstanceOf(FormData);
    });

    it('should make successful query request', async () => {
        const queryRequest = {
            question: 'What is the main topic?',
            config: { max_chunks: 10, similarity_threshold: 0.7 }
        };

        const mockResponse: RAGResponse = {
            answer: 'The main topic is artificial intelligence.',
            sources: [
                {
                    document_id: 'doc-123',
                    chunk_id: 'chunk-1',
                    relevance_score: 0.95,
                    snippet: 'This document discusses artificial intelligence as the main topic.',
                    source_file: 'test.pdf',
                    chunk_index: 0,
                    headers: ['Introduction']
                }
            ],
            confidence: 0.92,
            query: 'What is the main topic?',
            response_time_ms: 2500,
            timestamp: '2024-01-01T00:00:00Z'
        };

        mockFetch.mockResolvedValueOnce({
            ok: true,
            status: 200,
            statusText: 'OK',
            headers: new Headers({ 'content-type': 'application/json' }),
            json: async () => mockResponse
        });

        const result = await apiService.queryDocuments(queryRequest);

        expect(result).toEqual(mockResponse);
        expect(mockFetch).toHaveBeenCalledTimes(1);

        const [url, options] = mockFetch.mock.calls[0] as [string, RequestInit];
        expect(url).toContain('/query');
        expect(options.method).toBe('POST');
        expect(options.body).toBe(JSON.stringify(queryRequest));
    });

    it('should make successful health check request', async () => {
        const mockResponse: HealthResponse = {
            status: 'healthy',
            timestamp: '2024-01-01T00:00:00Z',
            services: {
                qdrant: true,
                azure_openai: true
            },
            uptime_seconds: 3600
        };

        mockFetch.mockResolvedValueOnce({
            ok: true,
            status: 200,
            statusText: 'OK',
            headers: new Headers({ 'content-type': 'application/json' }),
            json: async () => mockResponse
        });

        const result = await apiService.getHealth();

        expect(result).toEqual(mockResponse);
        expect(mockFetch).toHaveBeenCalledTimes(1);

        const [url, options] = mockFetch.mock.calls[0] as [string, RequestInit];
        expect(url).toContain('/health');
        expect(options.method).toBe('GET');
    });

    it('should handle API errors properly', async () => {
        mockFetch.mockResolvedValueOnce({
            ok: false,
            status: 400,
            statusText: 'Bad Request',
            headers: new Headers({ 'content-type': 'application/json' }),
            json: async () => ({ error: 'Invalid request', message: 'Bad request parameters' })
        });

        await expect(apiService.getHealth()).rejects.toThrow();
    });
});