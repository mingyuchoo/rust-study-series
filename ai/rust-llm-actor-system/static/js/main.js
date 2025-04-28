document.addEventListener('DOMContentLoaded', () => {
    const promptForm = document.getElementById('prompt-form');
    const promptInput = document.getElementById('prompt-input');
    const chatHistory = document.getElementById('chat-history');
    const submitBtn = document.getElementById('submit-btn');

    // Initialize the page
    fetchAgentsStatus();
    fetchSystemStats();

    // Set up polling for updates
    setInterval(() => {
        fetchAgentsStatus();
        fetchSystemStats();
    }, 5000); // Update every 5 seconds

    // Handle form submission
    promptForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        
        const prompt = promptInput.value.trim();
        if (!prompt) return;
        
        // Disable the submit button and show loading state
        submitBtn.disabled = true;
        submitBtn.textContent = 'Sending...';
        
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
                throw new Error('Failed to get response');
            }
            
            const data = await response.json();
            
            // Add agent response to chat
            addMessageToChat('agent', data.response, data.agent_id);
            
            // Clear input
            promptInput.value = '';
            
            // Update stats after response
            fetchAgentsStatus();
            fetchSystemStats();
            
        } catch (error) {
            console.error('Error:', error);
            addMessageToChat('agent', 'Sorry, there was an error processing your request. Please try again.');
        } finally {
            // Re-enable the submit button
            submitBtn.disabled = false;
            submitBtn.textContent = 'Send';
        }
    });

    // Function to add a message to the chat history
    function addMessageToChat(type, content, agentId = null) {
        const messageDiv = document.createElement('div');
        messageDiv.className = `message ${type}`;
        
        const contentDiv = document.createElement('div');
        contentDiv.className = 'message-content';
        contentDiv.textContent = content;
        messageDiv.appendChild(contentDiv);
        
        if (agentId) {
            const agentDiv = document.createElement('div');
            agentDiv.className = 'message-agent';
            agentDiv.textContent = `Answered by: ${agentId}`;
            messageDiv.appendChild(agentDiv);
        }
        
        chatHistory.appendChild(messageDiv);
        
        // Scroll to bottom
        chatHistory.scrollTop = chatHistory.scrollHeight;
    }

    // Function to fetch agents status
    async function fetchAgentsStatus() {
        try {
            const response = await fetch('/api/agents');
            if (!response.ok) {
                throw new Error('Failed to fetch agents');
            }
            
            const agents = await response.json();
            updateAgentsList(agents);
        } catch (error) {
            console.error('Error fetching agents:', error);
        }
    }

    // Function to update agents list in UI
    function updateAgentsList(agents) {
        const agentsList = document.getElementById('agents-list');
        agentsList.innerHTML = '';
        
        agents.forEach(agent => {
            const healthClass = agent.health.toLowerCase();
            
            const agentCard = document.createElement('div');
            agentCard.className = `agent-card ${healthClass}`;
            
            agentCard.innerHTML = `
                <h3>${agent.id}</h3>
                <p><strong>Model:</strong> ${agent.model}</p>
                <p><strong>Health:</strong> <span class="health-status">${agent.health}</span></p>
                <p><strong>Prompts:</strong> ${agent.prompts}</p>
                <p><strong>Avg Time:</strong> ${agent.avg_time}ms</p>
            `;
            
            agentsList.appendChild(agentCard);
        });
    }

    // Function to fetch system stats
    async function fetchSystemStats() {
        try {
            const response = await fetch('/api/stats');
            if (!response.ok) {
                throw new Error('Failed to fetch stats');
            }
            
            const stats = await response.json();
            updateSystemStats(stats);
        } catch (error) {
            console.error('Error fetching stats:', error);
        }
    }

    // Function to update system stats in UI
    function updateSystemStats(stats) {
        const statsContainer = document.getElementById('stats-container');
        statsContainer.innerHTML = '';
        
        for (const [key, value] of Object.entries(stats)) {
            const statItem = document.createElement('div');
            statItem.className = 'stat-item';
            
            statItem.innerHTML = `
                <span class="stat-label">${key}:</span>
                <span class="stat-value">${value}</span>
            `;
            
            statsContainer.appendChild(statItem);
        }
    }
});
