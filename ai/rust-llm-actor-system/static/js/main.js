document.addEventListener('DOMContentLoaded', () => {
    // Configure marked.js
    marked.use({
        gfm: true, // GitHub Flavored Markdown
        breaks: true, // Convert \n to <br>
        headerIds: false, // Don't add ids to headers (for security)
        mangle: false, // Don't mangle email links (for security)
        pedantic: false, // Don't be pedantic about markdown spec
        sanitize: false, // Don't sanitize (we'll use DOMPurify if needed)
        smartLists: true, // Use smarter list behavior
        smartypants: true, // Use smart typographic punctuation
        highlight: function(code, lang) {
            // Use highlight.js for code syntax highlighting
            const language = hljs.getLanguage(lang) ? lang : 'plaintext';
            return hljs.highlight(code, { language }).value;
        }
    });
    // DOM Elements
    const promptForm = document.getElementById('prompt-form');
    const promptInput = document.getElementById('prompt-input');
    const chatHistory = document.getElementById('chat-history');
    const submitBtn = document.getElementById('submit-btn');
    const refreshAgentsBtn = document.getElementById('refresh-agents');
    const refreshStatsBtn = document.getElementById('refresh-stats');
    const clearChatBtn = document.getElementById('clear-chat');
    const typingIndicator = document.getElementById('typing-indicator');
    
    // State variables
    let isProcessing = false;
    let lastMessageTime = Date.now();
    let autoRefreshEnabled = true;
    let autoRefreshInterval;
    let reconnectAttempts = 0;
    const MAX_RECONNECT_ATTEMPTS = 5;
    
    // Initialize the page
    initializePage();
    
    // Set up event listeners
    setupEventListeners();
    
    // Set up WebSocket connection for real-time updates
    setupWebSocket();
    
    /**
     * Initialize the page with data
     */
    function initializePage() {
        fetchAgentsStatus();
        fetchSystemStats();
        
        // Set up auto-refresh
        startAutoRefresh();
        
        // Focus on input
        promptInput.focus();
        
        // Enable textarea auto-resize
        promptInput.addEventListener('input', autoResizeTextarea);
    }
    
    /**
     * Set up all event listeners
     */
    function setupEventListeners() {
        // Handle form submission
        promptForm.addEventListener('submit', handlePromptSubmission);
        
        // Manual refresh buttons
        refreshAgentsBtn.addEventListener('click', handleAgentsRefresh);
        refreshStatsBtn.addEventListener('click', handleStatsRefresh);
        
        // Clear chat button
        clearChatBtn.addEventListener('click', clearChat);
        
        // Auto-resize textarea on input
        promptInput.addEventListener('keydown', handleTextareaKeydown);
    }
    
    /**
     * Set up WebSocket connection for real-time updates
     */
    function setupWebSocket() {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/ws`;
        
        try {
            const socket = new WebSocket(wsUrl);
            
            socket.onopen = () => {
                console.log('WebSocket connection established');
                reconnectAttempts = 0;
            };
            
            socket.onmessage = (event) => {
                const data = JSON.parse(event.data);
                handleWebSocketMessage(data);
            };
            
            socket.onclose = () => {
                console.log('WebSocket connection closed');
                if (reconnectAttempts < MAX_RECONNECT_ATTEMPTS) {
                    reconnectAttempts++;
                    const timeout = Math.min(1000 * reconnectAttempts, 5000);
                    setTimeout(setupWebSocket, timeout);
                }
            };
            
            socket.onerror = (error) => {
                console.error('WebSocket error:', error);
            };
        } catch (error) {
            console.error('Failed to establish WebSocket connection:', error);
        }
    }
    
    /**
     * Handle WebSocket messages
     */
    function handleWebSocketMessage(data) {
        if (!data || !data.type) return;
        
        switch (data.type) {
            case 'agent_update':
                updateAgentsList(data.agents);
                break;
            case 'stats_update':
                updateSystemStats(data.stats);
                break;
            case 'response_stream':
                handleStreamedResponse(data);
                break;
            default:
                console.log('Unknown message type:', data.type);
        }
    }
    
    /**
     * Handle streamed responses from the server
     */
    function handleStreamedResponse(data) {
        if (data.status === 'thinking') {
            showTypingIndicator();
        } else if (data.status === 'responding') {
            const messageElement = document.querySelector(`[data-message-id="${data.message_id}"]`);
            
            if (messageElement) {
                const contentElement = messageElement.querySelector('.message-content');
                if (contentElement) {
                    // For streaming, we need to accumulate the full markdown content
                    // Store the raw markdown in a data attribute
                    const currentMarkdown = contentElement.dataset.markdownContent || '';
                    const updatedMarkdown = currentMarkdown + data.chunk;
                    contentElement.dataset.markdownContent = updatedMarkdown;
                    
                    // Render the updated markdown
                    contentElement.innerHTML = marked.parse(updatedMarkdown);
                    
                    // Apply syntax highlighting to code blocks
                    contentElement.querySelectorAll('pre code').forEach((block) => {
                        hljs.highlightElement(block);
                    });
                    
                    // Make links open in a new tab
                    contentElement.querySelectorAll('a').forEach(link => {
                        link.setAttribute('target', '_blank');
                        link.setAttribute('rel', 'noopener noreferrer');
                    });
                }
            } else {
                // Create new message if it doesn't exist
                const messageId = data.message_id;
                const messageDiv = addStreamingMessage('agent', data.chunk, data.agent_id, messageId);
                
                // Store the initial markdown content
                const contentElement = messageDiv.querySelector('.message-content');
                if (contentElement) {
                    contentElement.dataset.markdownContent = data.chunk;
                }
            }
            
            hideTypingIndicator();
        } else if (data.status === 'complete') {
            hideTypingIndicator();
            
            // Final rendering of all code blocks after the complete message is received
            document.querySelectorAll('.message-content.markdown pre code').forEach((block) => {
                hljs.highlightElement(block);
            });
        }
    }
    
    /**
     * Start auto-refresh for agents and stats
     */
    function startAutoRefresh() {
        if (autoRefreshInterval) {
            clearInterval(autoRefreshInterval);
        }
        
        autoRefreshInterval = setInterval(() => {
            if (autoRefreshEnabled) {
                // Only auto-refresh if user hasn't interacted recently (10 seconds)
                if (Date.now() - lastMessageTime > 10000) {
                    fetchAgentsStatus(false);
                    fetchSystemStats(false);
                }
            }
        }, 5000); // Update every 5 seconds
    }
    
    /**
     * Handle form submission
     */
    async function handlePromptSubmission(e) {
        e.preventDefault();
        
        const prompt = promptInput.value.trim();
        if (!prompt || isProcessing) return;
        
        isProcessing = true;
        lastMessageTime = Date.now();
        
        // Update UI to show processing state
        submitBtn.disabled = true;
        submitBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i>';
        showTypingIndicator();
        
        // Add user message to chat
        addMessageToChat('user', prompt);
        
        try {
            // Send prompt to server
            const response = await fetch('/api/prompt', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ prompt })
            });
            
            if (!response.ok) {
                throw new Error(`Server responded with status: ${response.status}`);
            }
            
            const data = await response.json();
            
            // Add agent response to chat
            addMessageToChat('agent', data.response, data.agent_id);
            
            // Update stats after response
            fetchAgentsStatus();
            fetchSystemStats();
            
        } catch (error) {
            console.error('Error:', error);
            addMessageToChat('agent', `Sorry, there was an error processing your request: ${error.message}. Please try again.`);
        } finally {
            // Reset UI
            isProcessing = false;
            submitBtn.disabled = false;
            submitBtn.innerHTML = '<i class="fas fa-paper-plane"></i>';
            hideTypingIndicator();
            
            // Clear input and reset height
            promptInput.value = '';
            promptInput.style.height = 'auto';
            promptInput.focus();
        }
    }
    
    /**
     * Handle manual refresh of agents
     */
    function handleAgentsRefresh(e) {
        e.preventDefault();
        const button = e.currentTarget;
        button.classList.add('loading');
        
        fetchAgentsStatus().finally(() => {
            button.classList.remove('loading');
        });
    }
    
    /**
     * Handle manual refresh of stats
     */
    function handleStatsRefresh(e) {
        e.preventDefault();
        const button = e.currentTarget;
        button.classList.add('loading');
        
        fetchSystemStats().finally(() => {
            button.classList.remove('loading');
        });
    }
    
    /**
     * Clear the chat history
     */
    function clearChat() {
        // Add confirmation dialog
        if (chatHistory.children.length > 1 && !confirm('Are you sure you want to clear the chat history?')) {
            return;
        }
        
        // Remove all messages but keep the welcome message
        while (chatHistory.firstChild) {
            chatHistory.removeChild(chatHistory.firstChild);
        }
        
        // Add welcome message back
        const welcomeMessage = document.createElement('div');
        welcomeMessage.className = 'welcome-message';
        welcomeMessage.innerHTML = `
            <h3>Welcome to the LLM Agent System</h3>
            <p>Ask a question below to interact with our intelligent agents.</p>
        `;
        chatHistory.appendChild(welcomeMessage);
    }
    
    /**
     * Function to add a message to the chat history
     */
    function addMessageToChat(type, content, agentId = null) {
        // Remove welcome message if it exists
        const welcomeMessage = chatHistory.querySelector('.welcome-message');
        if (welcomeMessage) {
            chatHistory.removeChild(welcomeMessage);
        }
        
        const messageId = `msg-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
        const messageDiv = document.createElement('div');
        messageDiv.className = `message ${type}`;
        messageDiv.dataset.messageId = messageId;
        
        const contentDiv = document.createElement('div');
        contentDiv.className = 'message-content';
        
        // Only render agent messages as Markdown, not user messages
        if (type === 'agent') {
            contentDiv.className += ' markdown';
            // Use marked.js to render Markdown to HTML
            contentDiv.innerHTML = marked.parse(content);
            
            // Apply syntax highlighting to code blocks
            contentDiv.querySelectorAll('pre code').forEach((block) => {
                hljs.highlightElement(block);
            });
            
            // Make links open in a new tab
            contentDiv.querySelectorAll('a').forEach(link => {
                link.setAttribute('target', '_blank');
                link.setAttribute('rel', 'noopener noreferrer');
            });
        } else {
            // For user messages, just use text content
            contentDiv.textContent = content;
        }
        
        messageDiv.appendChild(contentDiv);
        
        if (agentId) {
            const agentDiv = document.createElement('div');
            agentDiv.className = 'message-agent';
            agentDiv.textContent = `Answered by: ${agentId}`;
            messageDiv.appendChild(agentDiv);
        }
        
        chatHistory.appendChild(messageDiv);
        
        // Scroll to bottom
        scrollChatToBottom();
        
        return messageId;
    }
    
    /**
     * Add a streaming message to the chat
     */
    function addStreamingMessage(type, initialContent, agentId = null, messageId) {
        // Remove welcome message if it exists
        const welcomeMessage = chatHistory.querySelector('.welcome-message');
        if (welcomeMessage) {
            chatHistory.removeChild(welcomeMessage);
        }
        
        const messageDiv = document.createElement('div');
        messageDiv.className = `message ${type}`;
        messageDiv.dataset.messageId = messageId;
        
        const contentDiv = document.createElement('div');
        contentDiv.className = 'message-content';
        
        // Only render agent messages as Markdown, not user messages
        if (type === 'agent') {
            contentDiv.className += ' markdown';
            // For streaming content, we'll start with the initial content
            contentDiv.innerHTML = marked.parse(initialContent || '');
        } else {
            // For user messages, just use text content
            contentDiv.textContent = initialContent || '';
        }
        
        messageDiv.appendChild(contentDiv);
        
        if (agentId) {
            const agentDiv = document.createElement('div');
            agentDiv.className = 'message-agent';
            agentDiv.textContent = `Answered by: ${agentId}`;
            messageDiv.appendChild(agentDiv);
        }
        
        chatHistory.appendChild(messageDiv);
        
        // Scroll to bottom
        scrollChatToBottom();
        
        return messageDiv;
    }
    
    /**
     * Scroll chat to bottom smoothly
     */
    function scrollChatToBottom() {
        chatHistory.scrollTo({
            top: chatHistory.scrollHeight,
            behavior: 'smooth'
        });
    }
    
    /**
     * Show typing indicator
     */
    function showTypingIndicator() {
        typingIndicator.classList.remove('hidden');
    }
    
    /**
     * Hide typing indicator
     */
    function hideTypingIndicator() {
        typingIndicator.classList.add('hidden');
    }
    
    /**
     * Auto-resize textarea based on content
     */
    function autoResizeTextarea() {
        promptInput.style.height = 'auto';
        promptInput.style.height = `${Math.min(promptInput.scrollHeight, 150)}px`;
    }
    
    /**
     * Handle textarea keydown events (for Enter key submission)
     */
    function handleTextareaKeydown(e) {
        // Submit on Enter (without Shift)
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            promptForm.dispatchEvent(new Event('submit'));
        }
    }
    
    /**
     * Function to fetch agents status
     */
    async function fetchAgentsStatus(showLoading = true) {
        if (showLoading) {
            document.getElementById('agents-list').innerHTML = '<div class="loading-placeholder">Loading agents...</div>';
        }
        
        try {
            const response = await fetch('/api/agents');
            if (!response.ok) {
                throw new Error(`Failed to fetch agents: ${response.status}`);
            }
            
            const agents = await response.json();
            updateAgentsList(agents);
            return agents;
        } catch (error) {
            console.error('Error fetching agents:', error);
            document.getElementById('agents-list').innerHTML = `<div class="error-message">Failed to load agents: ${error.message}</div>`;
            return [];
        }
    }
    
    /**
     * Function to update agents list in UI
     */
    function updateAgentsList(agents) {
        const agentsList = document.getElementById('agents-list');
        
        // Don't update if there's a loading placeholder and no agents
        if (agentsList.querySelector('.loading-placeholder') && (!agents || agents.length === 0)) {
            return;
        }
        
        agentsList.innerHTML = '';
        
        if (!agents || agents.length === 0) {
            agentsList.innerHTML = '<div class="info-message">No active agents found.</div>';
            return;
        }
        
        agents.forEach(agent => {
            const healthClass = agent.health.toLowerCase();
            
            const agentCard = document.createElement('div');
            agentCard.className = `agent-card ${healthClass}`;
            agentCard.setAttribute('data-agent-id', agent.id);
            
            // Format average time to be more readable
            const avgTime = typeof agent.avg_time === 'number' ? 
                (agent.avg_time > 1000 ? `${(agent.avg_time / 1000).toFixed(2)}s` : `${agent.avg_time}ms`) : 
                agent.avg_time;
            
            agentCard.innerHTML = `
                <h3>${agent.id}</h3>
                <p><strong>Model:</strong> ${agent.model}</p>
                <p><strong>Health:</strong> <span class="health-status">${agent.health}</span></p>
                <p><strong>Prompts:</strong> ${agent.prompts}</p>
                <p><strong>Avg Time:</strong> ${avgTime}</p>
            `;
            
            agentsList.appendChild(agentCard);
        });
    }
    
    /**
     * Function to fetch system stats
     */
    async function fetchSystemStats(showLoading = true) {
        if (showLoading) {
            document.getElementById('stats-container').innerHTML = '<div class="loading-placeholder">Loading statistics...</div>';
        }
        
        try {
            const response = await fetch('/api/stats');
            if (!response.ok) {
                throw new Error(`Failed to fetch stats: ${response.status}`);
            }
            
            const stats = await response.json();
            updateSystemStats(stats);
            return stats;
        } catch (error) {
            console.error('Error fetching stats:', error);
            document.getElementById('stats-container').innerHTML = `<div class="error-message">Failed to load statistics: ${error.message}</div>`;
            return {};
        }
    }
    
    /**
     * Function to update system stats in UI
     */
    function updateSystemStats(stats) {
        const statsContainer = document.getElementById('stats-container');
        
        // Don't update if there's a loading placeholder and no stats
        if (statsContainer.querySelector('.loading-placeholder') && (!stats || Object.keys(stats).length === 0)) {
            return;
        }
        
        statsContainer.innerHTML = '';
        
        if (!stats || Object.keys(stats).length === 0) {
            statsContainer.innerHTML = '<div class="info-message">No system statistics available.</div>';
            return;
        }
        
        for (const [key, value] of Object.entries(stats)) {
            const formattedKey = key.replace(/_/g, ' ');
            const statItem = document.createElement('div');
            statItem.className = 'stat-item';
            
            // Format the value based on its type
            let formattedValue = value;
            if (typeof value === 'number') {
                // Format numbers with commas for thousands
                formattedValue = value.toLocaleString();
                
                // Add units for specific stats
                if (key.includes('time') || key.includes('latency')) {
                    formattedValue = value > 1000 ? `${(value / 1000).toFixed(2)}s` : `${value}ms`;
                } else if (key.includes('memory') || key.includes('size')) {
                    formattedValue = formatBytes(value);
                }
            }
            
            statItem.innerHTML = `
                <span class="stat-label">${formattedKey}:</span>
                <span class="stat-value">${formattedValue}</span>
            `;
            
            statsContainer.appendChild(statItem);
        }
    }
    
    /**
     * Format bytes to human-readable format
     */
    function formatBytes(bytes, decimals = 2) {
        if (bytes === 0) return '0 Bytes';
        
        const k = 1024;
        const dm = decimals < 0 ? 0 : decimals;
        const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
        
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        
        return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
    }
});
