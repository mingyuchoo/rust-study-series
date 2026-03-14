use anyhow::Result;
use tonic::transport::Channel;
use tonic::Code;
use tracing::info;

pub mod proto {
    tonic::include_proto!("blog");
}

use proto::blog_service_client::BlogServiceClient;
use proto::{
    CreateCommentRequest, CreatePostRequest, GetPostRequest, ListCommentsRequest,
    ListPostsRequest, LoginRequest, RegisterRequest, VersionRequest,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("blog_client=info".parse()?),
        )
        .init();

    let server_addr =
        std::env::var("SERVER_ADDR").unwrap_or_else(|_| "http://127.0.0.1:50051".into());

    info!("Connecting to gRPC server at {}", server_addr);
    let channel = Channel::from_shared(server_addr)?.connect().await?;
    let mut client = BlogServiceClient::new(channel);

    // 1. 버전 확인
    println!("=== 버전 확인 ===");
    let version = client.get_version(VersionRequest {}).await?;
    println!("Version: {}\n", version.into_inner().version);

    // 2. 회원가입
    println!("=== 회원가입 ===");
    match client
        .register(RegisterRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        })
        .await
    {
        Ok(res) => {
            let register_res = res.into_inner();
            let user = register_res.user.unwrap();
            println!("회원가입 완료: {} ({})\n", user.username, user.email);
        }
        Err(status) if status.code() == Code::AlreadyExists => {
            println!("이미 등록된 사용자입니다. 로그인으로 진행합니다.\n");
        }
        Err(e) => return Err(e.into()),
    }

    // 3. 로그인
    println!("=== 로그인 ===");
    let login_res = client
        .login(LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        })
        .await?
        .into_inner();
    let token = login_res.token;
    println!("로그인 성공: {}\n", login_res.user.unwrap().username);

    // 4. 포스트 작성
    println!("=== 포스트 작성 ===");
    let post1 = client
        .create_post(CreatePostRequest {
            token: token.clone(),
            title: "첫 번째 블로그 포스트".to_string(),
            content: "WASI 0.2와 gRPC를 사용한 블로그 서비스입니다.".to_string(),
        })
        .await?
        .into_inner()
        .post
        .unwrap();
    println!("포스트 생성: [{}] {}", post1.id, post1.title);

    let post2 = client
        .create_post(CreatePostRequest {
            token: token.clone(),
            title: "SurrealDB 사용기".to_string(),
            content: "SurrealDB는 뛰어난 멀티모델 데이터베이스입니다.".to_string(),
        })
        .await?
        .into_inner()
        .post
        .unwrap();
    println!("포스트 생성: [{}] {}\n", post2.id, post2.title);

    // 5. 포스트 목록 조회
    println!("=== 포스트 목록 ===");
    let list_res = client
        .list_posts(ListPostsRequest {
            page: 1,
            per_page: 10,
        })
        .await?
        .into_inner();
    println!("총 {} 건:", list_res.total);
    for post in &list_res.posts {
        println!(
            "  [{}] {} (by {}, 댓글 {}개)",
            post.id,
            post.title,
            post.author.as_ref().map(|a| a.username.as_str()).unwrap_or("?"),
            post.comment_count,
        );
    }
    println!();

    // 6. 포스트 상세 조회
    println!("=== 포스트 상세 ===");
    let detail = client
        .get_post(GetPostRequest {
            id: post1.id.clone(),
        })
        .await?
        .into_inner()
        .post
        .unwrap();
    println!("제목: {}", detail.title);
    println!("내용: {}", detail.content);
    println!(
        "작성자: {}",
        detail.author.as_ref().map(|a| a.username.as_str()).unwrap_or("?")
    );
    println!();

    // 7. 댓글 작성
    println!("=== 댓글 작성 ===");
    let comment1 = client
        .create_comment(CreateCommentRequest {
            token: token.clone(),
            post_id: post1.id.clone(),
            content: "좋은 글이네요!".to_string(),
        })
        .await?
        .into_inner()
        .comment
        .unwrap();
    println!("댓글 생성: [{}] {}", comment1.id, comment1.content);

    let comment2 = client
        .create_comment(CreateCommentRequest {
            token: token.clone(),
            post_id: post1.id.clone(),
            content: "WASI에 대해 더 알고 싶습니다!".to_string(),
        })
        .await?
        .into_inner()
        .comment
        .unwrap();
    println!("댓글 생성: [{}] {}\n", comment2.id, comment2.content);

    // 8. 댓글 목록 조회
    println!("=== 댓글 목록 ===");
    let comments = client
        .list_comments(ListCommentsRequest {
            post_id: post1.id.clone(),
        })
        .await?
        .into_inner();
    for c in &comments.comments {
        println!(
            "  [{}] {} - by {}",
            c.id,
            c.content,
            c.author.as_ref().map(|a| a.username.as_str()).unwrap_or("?"),
        );
    }

    println!("\n블로그 API 데모 완료!");
    Ok(())
}
