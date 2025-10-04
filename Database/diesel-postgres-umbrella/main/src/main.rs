use adapters::PgTodoRepository;
use application::{create_todo::CreateTodoUseCase, list_todos::ListTodosUseCase};
use infra::db::DbProvider;

fn main() {
    // 1) 인프라: DB 연결 + 마이그레이션/시드
    let mut conn = DbProvider::establish();
    DbProvider::migrate_and_seed(&mut conn);

    // 2) 리포지토리 생성
    let mut repo = PgTodoRepository::new(&mut conn);

    // 3) 유스케이스: Todo 생성
    let create_uc = CreateTodoUseCase::new();
    let todo = create_uc.execute(&mut repo, "Clean Architecture skeleton");
    println!("Created todo id={} title={}", todo.id, todo.title);

    // 4) 유스케이스: Todo 목록 조회
    let list_uc = ListTodosUseCase::new();
    let items = list_uc.execute(&mut repo);
    println!("Todos ({}):", items.len());
    for t in items {
        println!("- {}: {}", t.id, t.title);
    }
}
