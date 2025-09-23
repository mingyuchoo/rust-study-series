import { useState, useEffect } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

interface Todo {
  id: string;
  title: string;
  description?: string;
  completed: boolean;
  created_at: string;
  updated_at: string;
}

interface CreateTodo {
  title: string;
  description?: string;
}

function App() {
  const [todos, setTodos] = useState<Todo[]>([])
  const [loading, setLoading] = useState(false)
  const [newTodo, setNewTodo] = useState<CreateTodo>({ title: '', description: '' })
  const [editingTodo, setEditingTodo] = useState<Todo | null>(null)

  const fetchTodos = async () => {
    setLoading(true)
    try {
      const response = await fetch('/api/todos')
      const data = await response.json()
      setTodos(data)
    } catch (error) {
      console.error('Failed to fetch todos:', error)
    } finally {
      setLoading(false)
    }
  }

  const createTodo = async () => {
    if (!newTodo.title.trim()) return

    try {
      const response = await fetch('/api/todos', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(newTodo),
      })

      if (response.ok) {
        setNewTodo({ title: '', description: '' })
        fetchTodos()
      }
    } catch (error) {
      console.error('Failed to create todo:', error)
    }
  }

  const updateTodo = async (id: string, updates: Partial<Todo>) => {
    try {
      const response = await fetch(`/api/todos/${id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(updates),
      })

      if (response.ok) {
        fetchTodos()
        setEditingTodo(null)
      }
    } catch (error) {
      console.error('Failed to update todo:', error)
    }
  }

  const deleteTodo = async (id: string) => {
    try {
      const response = await fetch(`/api/todos/${id}`, {
        method: 'DELETE',
      })

      if (response.ok) {
        fetchTodos()
      }
    } catch (error) {
      console.error('Failed to delete todo:', error)
    }
  }

  const toggleComplete = (todo: Todo) => {
    updateTodo(todo.id, { completed: !todo.completed })
  }

  useEffect(() => {
    fetchTodos()
  }, [])

  return (
    <>
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>TODO Manager</h1>

      <div className="card">
        <h2>Add New TODO</h2>
        <div style={{ marginBottom: '1rem' }}>
          <input
            type="text"
            placeholder="Todo title"
            value={newTodo.title}
            onChange={(e) => setNewTodo({ ...newTodo, title: e.target.value })}
            style={{
              marginRight: '0.5rem',
              padding: '0.5rem',
              backgroundColor: '#1a1a1a',
              border: '1px solid #404040',
              borderRadius: '4px',
              color: 'rgba(255, 255, 255, 0.87)'
            }}
          />
          <input
            type="text"
            placeholder="Description (optional)"
            value={newTodo.description}
            onChange={(e) => setNewTodo({ ...newTodo, description: e.target.value })}
            style={{
              marginRight: '0.5rem',
              padding: '0.5rem',
              backgroundColor: '#1a1a1a',
              border: '1px solid #404040',
              borderRadius: '4px',
              color: 'rgba(255, 255, 255, 0.87)'
            }}
          />
          <button onClick={createTodo}>Add TODO</button>
        </div>
      </div>

      <div className="card">
        <h2>TODO List</h2>
        {loading ? (
          <p>Loading...</p>
        ) : todos.length > 0 ? (
          <div>
            {todos.map(todo => (
              <div key={todo.id} style={{
                border: '1px solid #404040',
                margin: '0.5rem 0',
                padding: '1rem',
                borderRadius: '8px',
                backgroundColor: todo.completed ? '#1a2e1a' : '#2a2a2a',
                boxShadow: '0 2px 4px rgba(0, 0, 0, 0.3)'
              }}>
                {editingTodo?.id === todo.id ? (
                  <div>
                    <input
                      type="text"
                      value={editingTodo.title}
                      onChange={(e) => setEditingTodo({ ...editingTodo, title: e.target.value })}
                      style={{
                        marginRight: '0.5rem',
                        padding: '0.5rem',
                        backgroundColor: '#1a1a1a',
                        border: '1px solid #404040',
                        borderRadius: '4px',
                        color: 'rgba(255, 255, 255, 0.87)'
                      }}
                    />
                    <input
                      type="text"
                      value={editingTodo.description || ''}
                      onChange={(e) => setEditingTodo({ ...editingTodo, description: e.target.value })}
                      style={{
                        marginRight: '0.5rem',
                        padding: '0.5rem',
                        backgroundColor: '#1a1a1a',
                        border: '1px solid #404040',
                        borderRadius: '4px',
                        color: 'rgba(255, 255, 255, 0.87)'
                      }}
                    />
                    <button onClick={() => updateTodo(todo.id, {
                      title: editingTodo.title,
                      description: editingTodo.description
                    })}>
                      Save
                    </button>
                    <button onClick={() => setEditingTodo(null)} style={{ marginLeft: '0.5rem' }}>
                      Cancel
                    </button>
                  </div>
                ) : (
                  <div>
                    <h3 style={{
                      textDecoration: todo.completed ? 'line-through' : 'none',
                      margin: '0 0 0.5rem 0'
                    }}>
                      {todo.title}
                    </h3>
                    {todo.description && (
                      <p style={{
                        textDecoration: todo.completed ? 'line-through' : 'none',
                        margin: '0 0 0.5rem 0',
                        color: todo.completed ? '#888' : '#b8b8b8'
                      }}>
                        {todo.description}
                      </p>
                    )}
                    <div style={{ fontSize: '0.8rem', color: '#888', marginBottom: '0.5rem' }}>
                      Created: {new Date(todo.created_at).toLocaleString()}
                    </div>
                    <div>
                      <button
                        onClick={() => toggleComplete(todo)}
                        style={{
                          marginRight: '0.5rem',
                          backgroundColor: todo.completed ? '#2d5a2d' : '#4a4a4a',
                          border: '1px solid #555',
                          color: 'rgba(255, 255, 255, 0.87)'
                        }}
                      >
                        {todo.completed ? 'Mark Incomplete' : 'Mark Complete'}
                      </button>
                      <button
                        onClick={() => setEditingTodo(todo)}
                        style={{
                          marginRight: '0.5rem',
                          backgroundColor: '#3a5998',
                          border: '1px solid #555',
                          color: 'rgba(255, 255, 255, 0.87)'
                        }}
                      >
                        Edit
                      </button>
                      <button
                        onClick={() => deleteTodo(todo.id)}
                        style={{
                          backgroundColor: '#8b2635',
                          border: '1px solid #555',
                          color: 'rgba(255, 255, 255, 0.87)'
                        }}
                      >
                        Delete
                      </button>
                    </div>
                  </div>
                )}
              </div>
            ))}
            <button onClick={fetchTodos} style={{ marginTop: '1rem' }}>
              Refresh TODOs
            </button>
          </div>
        ) : (
          <p>No todos yet. Add one above!</p>
        )}
      </div>

      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App
