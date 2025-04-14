pub mod repositories;

use self::repositories::PostRepository;
use self::repositories::TodoRepository;
use self::repositories::UserRepository;

use self::repositories::entities::Post;
use self::repositories::entities::Todo;
use self::repositories::entities::User;

pub struct PostService<R: PostRepository> {
    repository: R,
}

impl<R: PostRepository> PostService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn create(&self, post: Post) -> Result<Post, R::Error> {
        self.repository.create(post)
    }

    pub fn update(&self, post: Post) -> Result<Post, R::Error> {
        self.repository.update(post)
    }

    pub fn delete(&self, post: Post) -> Result<(), R::Error> {
        self.repository.delete(post)
    }

    pub fn find_by_id(&self, id: i32) -> Result<Post, R::Error> {
        self.repository.find_by_id(id)
    }

    pub fn find_all(&self) -> Result<Vec<Post>, R::Error> {
        self.repository.find_all()
    }
}

pub struct TodoService<R: TodoRepository> {
    repository: R,
}

impl<R: TodoRepository> TodoService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn create(&self, todo: Todo) -> Result<Todo, R::Error> {
        self.repository.create(todo)
    }

    pub fn update(&self, todo: Todo) -> Result<Todo, R::Error> {
        self.repository.update(todo)
    }

    pub fn delete(&self, todo: Todo) -> Result<(), R::Error> {
        self.repository.delete(todo)
    }

    pub fn find_by_id(&self, id: i32) -> Result<Todo, R::Error> {
        self.repository.find_by_id(id)
    }

    pub fn find_all(&self) -> Result<Vec<Todo>, R::Error> {
        self.repository.find_all()
    }
}

pub struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn create(&self, user: User) -> Result<User, R::Error> {
        self.repository.create(user)
    }

    pub fn update(&self, user: User) -> Result<User, R::Error> {
        self.repository.update(user)
    }

    pub fn delete(&self, user: User) -> Result<(), R::Error> {
        self.repository.delete(user)
    }

    pub fn find_by_id(&self, id: i32) -> Result<User, R::Error> {
        self.repository.find_by_id(id)
    }

    pub fn find_all(&self) -> Result<Vec<User>, R::Error> {
        self.repository.find_all()
    }
}
    