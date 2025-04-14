pub mod repositories;

use crate::application::services::PostApplicationService;
use crate::application::services::TodoApplicationService;
use crate::application::services::UserApplicationService;

pub struct PostApiController<R: PostRepository> {
    application_service: PostApplicationService<R>,
}

impl<R: PostRepository> PostApiController<R> {
    pub fn new(application_service: PostApplicationService<R>) -> Self {
        Self { application_service }
    }

    pub fn create(&self, post: Post) -> Result<Post, R::Error> {
        self.application_service.create(post)
    }

    pub fn update(&self, post: Post) -> Result<Post, R::Error> {
        self.application_service.update(post)
    }

    pub fn delete(&self, post: Post) -> Result<(), R::Error> {
        self.application_service.delete(post)
    }

    pub fn find_by_id(&self, id: i32) -> Result<Post, R::Error> {
        self.application_service.find_by_id(id)
    }

    pub fn find_all(&self) -> Result<Vec<Post>, R::Error> {
        self.application_service.find_all()
    }
}

pub struct TodoApiController<R: TodoRepository> {
    application_service: TodoApplicationService<R>,
}

impl<R: TodoRepository> TodoApiController<R> {
    pub fn new(application_service: TodoApplicationService<R>) -> Self {
        Self { application_service }
    }

    pub fn create(&self, todo: Todo) -> Result<Todo, R::Error> {
        self.application_service.create(todo)
    }

    pub fn update(&self, todo: Todo) -> Result<Todo, R::Error> {
        self.application_service.update(todo)
    }

    pub fn delete(&self, todo: Todo) -> Result<(), R::Error> {
        self.application_service.delete(todo)
    }

    pub fn find_by_id(&self, id: i32) -> Result<Todo, R::Error> {
        self.application_service.find_by_id(id)
    }

    pub fn find_all(&self) -> Result<Vec<Todo>, R::Error> {
        self.application_service.find_all()
    }
}

pub struct UserApiController<R: UserRepository> {
    application_service: UserApplicationService<R>,
}

impl<R: UserRepository> UserApiController<R> {
    pub fn new(application_service: UserApplicationService<R>) -> Self {
        Self { application_service }
    }

    pub fn create(&self, user: User) -> Result<User, R::Error> {
        self.application_service.create(user)
    }

    pub fn update(&self, user: User) -> Result<User, R::Error> {
        self.application_service.update(user)
    }

    pub fn delete(&self, user: User) -> Result<(), R::Error> {
        self.application_service.delete(user)
    }

    pub fn find_by_id(&self, id: i32) -> Result<User, R::Error> {
        self.application_service.find_by_id(id)
    }

    pub fn find_all(&self) -> Result<Vec<User>, R::Error> {
        self.application_service.find_all()
    }
}