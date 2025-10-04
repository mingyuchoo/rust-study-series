const API_BASE = '/api/todos';

let currentEditId = null;

// DOM Elements
const newTodoInput = document.getElementById('newTodoInput');
const addBtn = document.getElementById('addBtn');
const todoList = document.getElementById('todoList');
const totalCount = document.getElementById('totalCount');
const editModal = document.getElementById('editModal');
const editTodoInput = document.getElementById('editTodoInput');
const saveEditBtn = document.getElementById('saveEditBtn');
const cancelEditBtn = document.getElementById('cancelEditBtn');

// Event Listeners
addBtn.addEventListener('click', createTodo);
newTodoInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') createTodo();
});
saveEditBtn.addEventListener('click', saveEdit);
cancelEditBtn.addEventListener('click', closeModal);
editModal.addEventListener('click', (e) => {
    if (e.target === editModal) closeModal();
});

// Load todos on page load
loadTodos();

async function loadTodos() {
    try {
        todoList.innerHTML = '<div class="loading">로딩 중...</div>';
        const response = await fetch(API_BASE);
        const todos = await response.json();
        
        if (todos.length === 0) {
            todoList.innerHTML = `
                <div class="empty-state">
                    <p>📭 할 일이 없습니다</p>
                    <small>위에서 새로운 할 일을 추가해보세요!</small>
                </div>
            `;
        } else {
            todoList.innerHTML = todos.map(todo => createTodoElement(todo)).join('');
        }
        
        totalCount.textContent = `총 ${todos.length}개`;
    } catch (error) {
        console.error('Failed to load todos:', error);
        todoList.innerHTML = '<div class="empty-state"><p>❌ 로딩 실패</p></div>';
    }
}

function createTodoElement(todo) {
    return `
        <div class="todo-item" data-id="${todo.id}">
            <div class="todo-content">
                <span class="todo-id">#${todo.id}</span>
                <span class="todo-title">${escapeHtml(todo.title)}</span>
            </div>
            <div class="todo-actions">
                <button class="btn-edit" onclick="editTodo(${todo.id}, '${escapeHtml(todo.title)}')">수정</button>
                <button class="btn-delete" onclick="deleteTodo(${todo.id})">삭제</button>
            </div>
        </div>
    `;
}

async function createTodo() {
    const title = newTodoInput.value.trim();
    if (!title) {
        alert('할 일을 입력해주세요!');
        return;
    }
    
    try {
        const response = await fetch(API_BASE, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ title })
        });
        
        if (response.ok) {
            newTodoInput.value = '';
            await loadTodos();
        } else {
            alert('추가 실패');
        }
    } catch (error) {
        console.error('Failed to create todo:', error);
        alert('추가 실패');
    }
}

function editTodo(id, title) {
    currentEditId = id;
    editTodoInput.value = title;
    editModal.classList.add('active');
    editTodoInput.focus();
}

async function saveEdit() {
    const title = editTodoInput.value.trim();
    if (!title) {
        alert('할 일을 입력해주세요!');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE}/${currentEditId}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ title })
        });
        
        if (response.ok) {
            closeModal();
            await loadTodos();
        } else {
            alert('수정 실패');
        }
    } catch (error) {
        console.error('Failed to update todo:', error);
        alert('수정 실패');
    }
}

async function deleteTodo(id) {
    if (!confirm('정말 삭제하시겠습니까?')) return;
    
    try {
        const response = await fetch(`${API_BASE}/${id}`, {
            method: 'DELETE'
        });
        
        if (response.ok) {
            await loadTodos();
        } else {
            alert('삭제 실패');
        }
    } catch (error) {
        console.error('Failed to delete todo:', error);
        alert('삭제 실패');
    }
}

function closeModal() {
    editModal.classList.remove('active');
    currentEditId = null;
    editTodoInput.value = '';
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}
