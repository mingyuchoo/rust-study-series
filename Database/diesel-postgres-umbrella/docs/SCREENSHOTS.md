# Screenshots

## Web UI

### Main Interface
![Todo List Main Interface](screenshots/main-interface.png)
*The main todo list interface with add, edit, and delete functionality*

### Adding a Todo
![Adding a new todo](screenshots/add-todo.png)
*Simple input field to add new todos*

### Editing a Todo
![Edit modal](screenshots/edit-modal.png)
*Modal dialog for editing existing todos*

### Empty State
![Empty state](screenshots/empty-state.png)
*Friendly empty state when no todos exist*

## API Examples

### List All Todos
```bash
curl http://localhost:8000/api/todos
```

```json
[
  {
    "id": 1,
    "title": "Learn Diesel"
  },
  {
    "id": 2,
    "title": "Model domain entities"
  }
]
```

### Create a Todo
```bash
curl -X POST http://localhost:8000/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"New Task"}'
```

```json
{
  "id": 4,
  "title": "New Task"
}
```

### Update a Todo
```bash
curl -X PUT http://localhost:8000/api/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"title":"Updated Task"}'
```

```json
{
  "id": 1,
  "title": "Updated Task"
}
```

### Delete a Todo
```bash
curl -X DELETE http://localhost:8000/api/todos/1
```

Returns: `204 No Content`

---

**Note**: To add actual screenshots, run the application and capture images of the web interface, then save them in a `screenshots/` directory.
