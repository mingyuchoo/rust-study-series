document.addEventListener('DOMContentLoaded', () => {
    const chatBox = document.getElementById('chat-box');
    const messageInput = document.getElementById('message-input');
    const sendButton = document.getElementById('send-button');
    
    // Store conversation history
    let messages = [];
    
    // Function to add a message to the chat
    function addMessage(role, content) {
        // Add to UI
        const messageDiv = document.createElement('div');
        messageDiv.className = `message ${role}`;
        
        const messageContent = document.createElement('div');
        messageContent.className = 'message-content';
        messageContent.innerHTML = formatMessage(content);
        
        messageDiv.appendChild(messageContent);
        chatBox.appendChild(messageDiv);
        
        // Scroll to bottom
        chatBox.scrollTop = chatBox.scrollHeight;
        
        // Add to conversation history
        messages.push({ role, content });
    }
    
    // Function to format message with proper Markdown rendering
    function formatMessage(text) {
        // Use marked.js to parse Markdown
        try {
            // Set options for marked
            marked.setOptions({
                breaks: true,  // Convert line breaks to <br>
                gfm: true,     // Enable GitHub Flavored Markdown
                headerIds: false, // Don't add ids to headers
                mangle: false,  // Don't mangle email links
                sanitize: false // Don't sanitize HTML (marked handles this)
            });
            
            // Parse markdown text
            const parsed = marked.parse(text);
            
            // Return the parsed HTML
            return parsed;
        } catch (error) {
            console.error('Markdown parsing error:', error);
            
            // Fallback to basic formatting if marked fails
            text = text.replace(/https?:\/\/[^\s]+/g, url => `<a href="${url}" target="_blank">${url}</a>`);
            text = text.replace(/\n/g, '<br>');
            return text;
        }
    }
    
    // Function to show typing indicator
    function showTypingIndicator() {
        const typingDiv = document.createElement('div');
        typingDiv.className = 'typing-indicator';
        typingDiv.id = 'typing-indicator';
        
        for (let i = 0; i < 3; i++) {
            const dot = document.createElement('span');
            typingDiv.appendChild(dot);
        }
        
        chatBox.appendChild(typingDiv);
        chatBox.scrollTop = chatBox.scrollHeight;
    }
    
    // Function to remove typing indicator
    function removeTypingIndicator() {
        const typingIndicator = document.getElementById('typing-indicator');
        if (typingIndicator) {
            typingIndicator.remove();
        }
    }
    
    // Function to send message to API
    async function sendMessage() {
        const userMessage = messageInput.value.trim();
        if (!userMessage) return;
        
        // Clear input
        messageInput.value = '';
        
        // Add user message to chat
        addMessage('user', userMessage);
        
        // Show typing indicator
        showTypingIndicator();
        
        try {
            // Prepare conversation history for API
            const apiMessages = messages.map(msg => ({
                role: msg.role === 'user' ? 'user' : 'assistant',
                content: msg.content
            }));
            
            // Send request to our backend API
            const response = await fetch('/api/chat', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ messages: apiMessages })
            });
            
            if (!response.ok) {
                throw new Error('API request failed');
            }
            
            // Handle streaming response
            const reader = response.body.getReader();
            const decoder = new TextDecoder();
            let assistantResponse = '';
            
            // Remove typing indicator as we'll show the streaming text
            removeTypingIndicator();
            
            // Create a placeholder message for the assistant
            const assistantMessageDiv = document.createElement('div');
            assistantMessageDiv.className = 'message assistant';
            
            const assistantContent = document.createElement('div');
            assistantContent.className = 'message-content';
            assistantMessageDiv.appendChild(assistantContent);
            chatBox.appendChild(assistantMessageDiv);
            
            // Read the stream
            while (true) {
                const { done, value } = await reader.read();
                if (done) break;
                
                // Decode the chunk and append to our response
                const chunk = decoder.decode(value, { stream: true });
                assistantResponse += chunk;
                
                // Update the message content with formatted text
                assistantContent.innerHTML = formatMessage(assistantResponse);
                
                // Scroll to bottom
                chatBox.scrollTop = chatBox.scrollHeight;
            }
            
            // Add the complete response to our message history
            messages.push({ role: 'assistant', content: assistantResponse });
            
        } catch (error) {
            console.error('Error:', error);
            
            // Remove typing indicator
            removeTypingIndicator();
            
            // Show error message
            addMessage('assistant', '죄송합니다. 오류가 발생했습니다. 다시 시도해 주세요.');
        }
    }
    
    // Event listeners
    sendButton.addEventListener('click', sendMessage);
    
    messageInput.addEventListener('keydown', (e) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            sendMessage();
        }
    });
    
    // Focus input on load
    messageInput.focus();
});
