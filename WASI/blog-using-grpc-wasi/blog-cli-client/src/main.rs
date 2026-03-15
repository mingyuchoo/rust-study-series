use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use serde::Deserialize;
use tonic::transport::Channel;

pub mod proto {
    tonic::include_proto!("blog");
}

use proto::blog_service_client::BlogServiceClient;
use proto::{
    ChangePasswordRequest, CreateCommentRequest, CreatePostRequest, DeleteCommentRequest,
    DeleteMyAccountRequest, DeletePostRequest, DeleteUserRequest, GetMyProfileRequest,
    GetPostRequest, GetUserRequest, ListCommentsRequest, ListPostsRequest, ListUsersRequest,
    LoginRequest, RegisterRequest, SearchPostsRequest, UpdateCommentRequest, UpdatePostRequest,
    UpdatePostVisibilityRequest, UpdateUserRoleRequest, VersionRequest,
};

// ─── JSON 입력 타입 ───────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct RegisterInput {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginInput {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct PostCreateInput {
    title: String,
    content: String,
    #[serde(default = "default_visibility")]
    visibility: String,
}

fn default_visibility() -> String {
    "private".to_string()
}

#[derive(Deserialize)]
struct PostListInput {
    #[serde(default = "default_page")]
    page: u32,
    #[serde(default = "default_per_page")]
    per_page: u32,
    #[serde(default)]
    filter: String,
}

fn default_page() -> u32 {
    1
}
fn default_per_page() -> u32 {
    10
}

#[derive(Deserialize)]
struct IdInput {
    id: String,
}

#[derive(Deserialize)]
struct PostUpdateInput {
    id: String,
    title: String,
    content: String,
    #[serde(default)]
    visibility: String,
}

#[derive(Deserialize)]
struct CommentCreateInput {
    post_id: String,
    content: String,
    #[serde(default = "default_visibility")]
    visibility: String,
}

#[derive(Deserialize)]
struct CommentListInput {
    post_id: String,
}

#[derive(Deserialize)]
struct CommentUpdateInput {
    id: String,
    content: String,
    #[serde(default)]
    visibility: String,
}

#[derive(Deserialize)]
struct AdminUserIdInput {
    user_id: String,
}

#[derive(Deserialize)]
struct AdminUpdateRoleInput {
    user_id: String,
    role: String,
}

#[derive(Deserialize)]
struct AdminUpdateVisibilityInput {
    post_id: String,
    visibility: String,
}

#[derive(Deserialize)]
struct ChangePasswordInput {
    current_password: String,
    new_password: String,
}

#[derive(Deserialize)]
struct DeleteAccountInput {
    password: String,
}

#[derive(Deserialize)]
struct SearchInput {
    query: String,
    #[serde(default = "default_page")]
    page: u32,
    #[serde(default = "default_per_page")]
    per_page: u32,
}

// ─── JSON 파싱 ────────────────────────────────────────────────────────────────

fn parse_json<T: serde::de::DeserializeOwned>(json: &str, example: &str) -> Result<T> {
    serde_json::from_str(json).with_context(|| format!("JSON 파싱 실패. 예시: {}", example))
}

// ─── CLI 정의 ─────────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "blog-cli", about = "블로그 gRPC CLI 클라이언트", version)]
struct Cli {
    /// gRPC 서버 주소
    #[arg(long, env = "SERVER_ADDR", default_value = "http://127.0.0.1:50051")]
    server: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 서버 버전 확인
    Version,
    /// 회원가입
    /// 예: register '{"username":"alice","email":"a@b.com","password":"pw123"}'
    Register { json: String },
    /// 로그인 (토큰을 로컬에 저장)
    /// 예: login '{"email":"a@b.com","password":"pw123"}'
    Login { json: String },
    /// 프로필 관리
    Profile {
        #[command(subcommand)]
        command: ProfileCommands,
    },
    /// 포스트 관리
    Post {
        #[command(subcommand)]
        command: PostCommands,
    },
    /// 댓글 관리
    Comment {
        #[command(subcommand)]
        command: CommentCommands,
    },
    /// 관리자 전용 (admin 역할 필요)
    Admin {
        #[command(subcommand)]
        command: AdminCommands,
    },
}

#[derive(Subcommand)]
enum PostCommands {
    /// 포스트 작성
    /// 예: post create '{"title":"제목","content":"내용","visibility":"public"}'
    Create { json: String },
    /// 포스트 목록 조회
    /// 예: post list '{"page":1,"per_page":10}'
    List { json: Option<String> },
    /// 포스트 상세 조회
    /// 예: post get '{"id":"post:xxx"}'
    Get { json: String },
    /// 포스트 수정
    /// 예: post update '{"id":"post:xxx","title":"새제목","content":"새내용"}'
    Update { json: String },
    /// 포스트 삭제
    /// 예: post delete '{"id":"post:xxx"}'
    Delete { json: String },
    /// 포스트 검색
    /// 예: post search '{"query":"WASI","page":1,"per_page":10}'
    Search { json: String },
}

#[derive(Subcommand)]
enum ProfileCommands {
    /// 내 프로필 조회
    Me,
    /// 비밀번호 변경
    /// 예: profile change-password '{"current_password":"old","new_password":"new123"}'
    ChangePassword { json: String },
    /// 회원 탈퇴
    /// 예: profile delete-account '{"password":"mypassword"}'
    DeleteAccount { json: String },
}

#[derive(Subcommand)]
enum CommentCommands {
    /// 댓글 작성
    /// 예: comment create '{"post_id":"post:xxx","content":"댓글 내용"}'
    Create { json: String },
    /// 특정 포스트의 댓글 목록 조회
    /// 예: comment list '{"post_id":"post:xxx"}'
    List { json: String },
    /// 댓글 수정
    /// 예: comment update '{"id":"comment:xxx","content":"수정된 댓글"}'
    Update { json: String },
    /// 댓글 삭제
    /// 예: comment delete '{"id":"comment:xxx"}'
    Delete { json: String },
}

#[derive(Subcommand)]
enum AdminCommands {
    /// 사용자 목록 조회
    /// 예: admin list-users '{"page":1,"per_page":10}'
    ListUsers { json: Option<String> },
    /// 사용자 상세 조회
    /// 예: admin get-user '{"user_id":"xxx"}'
    GetUser { json: String },
    /// 사용자 역할 변경
    /// 예: admin update-role '{"user_id":"xxx","role":"admin"}'
    UpdateRole { json: String },
    /// 사용자 삭제 (관련 포스트/댓글 모두 삭제)
    /// 예: admin delete-user '{"user_id":"xxx"}'
    DeleteUser { json: String },
    /// 포스트 공개범위 변경
    /// 예: admin update-visibility '{"post_id":"xxx","visibility":"private"}'
    UpdateVisibility { json: String },
}

// ─── 토큰 관리 ────────────────────────────────────────────────────────────────

fn token_path() -> std::path::PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".blog-cli-token")
}

fn save_token(token: &str) -> Result<()> {
    std::fs::write(token_path(), token).context("토큰 저장 실패")
}

fn load_token() -> Result<String> {
    std::fs::read_to_string(token_path())
        .context("로그인이 필요합니다. 먼저 'login' 명령을 실행하세요")
}

// ─── gRPC 연결 ────────────────────────────────────────────────────────────────

async fn connect(server: &str) -> Result<BlogServiceClient<Channel>> {
    let channel = Channel::from_shared(server.to_string())?
        .connect()
        .await
        .context("gRPC 서버 연결 실패")?;
    Ok(BlogServiceClient::new(channel))
}

// ─── 출력 포맷팅 ──────────────────────────────────────────────────────────────

fn format_author(author: Option<&proto::UserInfo>) -> &str {
    author.map(|a| a.username.as_str()).unwrap_or("?")
}

fn print_post_summary(post: &proto::Post) {
    println!(
        "  [{}] {} (by {}, 댓글 {}개, {})",
        post.id,
        post.title,
        format_author(post.author.as_ref()),
        post.comment_count,
        post.visibility,
    );
}

fn print_post_detail(post: &proto::Post) {
    println!("제목:      {}", post.title);
    println!("작성자:    {}", format_author(post.author.as_ref()));
    println!("공개범위:  {}", post.visibility);
    println!("작성일:    {}", post.created_at);
    println!("수정일:    {}", post.updated_at);
    println!("댓글 수:   {}", post.comment_count);
    println!("───────────────────────────────");
    println!("{}", post.content);
}

fn print_comment(comment: &proto::Comment) {
    println!(
        "  [{}] {} - by {} ({})",
        comment.id,
        comment.content,
        format_author(comment.author.as_ref()),
        comment.created_at,
    );
}

fn print_user(user: &proto::UserInfo) {
    println!(
        "  [{}] {} ({}) - 역할: {}",
        user.id, user.username, user.email, user.role,
    );
}

// ─── 명령 핸들러 ──────────────────────────────────────────────────────────────

async fn handle_version(client: &mut BlogServiceClient<Channel>) -> Result<()> {
    let res = client.get_version(VersionRequest {}).await?.into_inner();
    println!("서버 버전: {}", res.version);
    Ok(())
}

async fn handle_register(client: &mut BlogServiceClient<Channel>, json: &str) -> Result<()> {
    let input: RegisterInput = parse_json(
        json,
        r#"'{"username":"alice","email":"a@b.com","password":"pw123"}'"#,
    )?;
    let res = client
        .register(RegisterRequest {
            username: input.username,
            email: input.email,
            password: input.password,
        })
        .await?
        .into_inner();
    let user = res.user.context("사용자 정보 없음")?;
    println!(
        "회원가입 완료: {} ({}) [역할: {}]",
        user.username, user.email, user.role
    );
    Ok(())
}

async fn handle_login(client: &mut BlogServiceClient<Channel>, json: &str) -> Result<()> {
    let input: LoginInput = parse_json(json, r#"'{"email":"a@b.com","password":"pw123"}'"#)?;
    let res = client
        .login(LoginRequest {
            email: input.email,
            password: input.password,
        })
        .await?
        .into_inner();
    save_token(&res.token)?;
    let user = res.user.context("사용자 정보 없음")?;
    println!("로그인 성공: {} [역할: {}]", user.username, user.role);
    println!("토큰이 저장되었습니다.");
    Ok(())
}

async fn handle_profile_me(client: &mut BlogServiceClient<Channel>) -> Result<()> {
    let token = load_token()?;
    let res = client
        .get_my_profile(GetMyProfileRequest { token })
        .await?
        .into_inner();
    let user = res.user.context("사용자 정보 없음")?;
    println!("내 프로필:");
    println!("  ID:       {}", user.id);
    println!("  사용자명: {}", user.username);
    println!("  이메일:   {}", user.email);
    println!("  역할:     {}", user.role);
    println!("  가입일:   {}", user.created_at);
    Ok(())
}

async fn handle_change_password(
    client: &mut BlogServiceClient<Channel>,
    json: &str,
) -> Result<()> {
    let input: ChangePasswordInput = parse_json(
        json,
        r#"'{"current_password":"old","new_password":"newPw123"}'"#,
    )?;
    let token = load_token()?;
    let res = client
        .change_password(ChangePasswordRequest {
            token,
            current_password: input.current_password,
            new_password: input.new_password,
        })
        .await?
        .into_inner();
    println!("{}", res.message);
    Ok(())
}

async fn handle_delete_account(
    client: &mut BlogServiceClient<Channel>,
    json: &str,
) -> Result<()> {
    let input: DeleteAccountInput = parse_json(json, r#"'{"password":"mypassword"}'"#)?;
    let token = load_token()?;
    let res = client
        .delete_my_account(DeleteMyAccountRequest {
            token,
            password: input.password,
        })
        .await?
        .into_inner();
    if res.success {
        println!("회원 탈퇴가 완료되었습니다.");
        let _ = std::fs::remove_file(token_path());
    } else {
        println!("회원 탈퇴에 실패했습니다.");
    }
    Ok(())
}

async fn handle_post_search(client: &mut BlogServiceClient<Channel>, json: &str) -> Result<()> {
    let input: SearchInput =
        parse_json(json, r#"'{"query":"WASI","page":1,"per_page":10}'"#)?;
    let token = load_token().unwrap_or_default();
    let res = client
        .search_posts(SearchPostsRequest {
            query: input.query.clone(),
            page: input.page,
            per_page: input.per_page,
            token,
        })
        .await?
        .into_inner();
    println!(
        "검색 결과 '{}' (총 {}건, 페이지 {}):",
        input.query, res.total, input.page
    );
    for post in &res.posts {
        print_post_summary(post);
    }
    Ok(())
}

async fn handle_post_create(client: &mut BlogServiceClient<Channel>, json: &str) -> Result<()> {
    let input: PostCreateInput = parse_json(
        json,
        r#"'{"title":"제목","content":"내용","visibility":"public"}'"#,
    )?;
    let token = load_token()?;
    let res = client
        .create_post(CreatePostRequest {
            token,
            title: input.title,
            content: input.content,
            visibility: input.visibility,
        })
        .await?
        .into_inner();
    let post = res.post.context("포스트 정보 없음")?;
    println!("포스트 생성 완료");
    println!("  ID:       {}", post.id);
    println!("  제목:     {}", post.title);
    println!("  공개범위: {}", post.visibility);
    Ok(())
}

async fn handle_post_list(
    client: &mut BlogServiceClient<Channel>,
    json: Option<&str>,
) -> Result<()> {
    let input: PostListInput = match json {
        Some(j) => parse_json(j, r#"'{"page":1,"per_page":10}'"#)?,
        None => PostListInput {
            page: default_page(),
            per_page: default_per_page(),
            filter: String::new(),
        },
    };
    let token = load_token().unwrap_or_default();
    let res = client
        .list_posts(ListPostsRequest {
            page: input.page,
            per_page: input.per_page,
            token,
            filter: input.filter,
        })
        .await?
        .into_inner();
    println!("포스트 목록 (총 {}건, 페이지 {}):", res.total, input.page);
    for post in &res.posts {
        print_post_summary(post);
    }
    Ok(())
}

async fn handle_post_get(client: &mut BlogServiceClient<Channel>, json: &str) -> Result<()> {
    let input: IdInput = parse_json(json, r#"'{"id":"post:xxx"}'"#)?;
    let token = load_token().unwrap_or_default();
    let res = client
        .get_post(GetPostRequest {
            id: input.id,
            token,
        })
        .await?
        .into_inner();
    let post = res.post.context("포스트를 찾을 수 없습니다")?;
    print_post_detail(&post);
    Ok(())
}

async fn handle_post_update(client: &mut BlogServiceClient<Channel>, json: &str) -> Result<()> {
    let input: PostUpdateInput = parse_json(
        json,
        r#"'{"id":"post:xxx","title":"새 제목","content":"새 내용"}'"#,
    )?;
    let token = load_token()?;
    let res = client
        .update_post(UpdatePostRequest {
            token,
            id: input.id,
            title: input.title,
            content: input.content,
            visibility: input.visibility,
        })
        .await?
        .into_inner();
    let post = res.post.context("포스트 정보 없음")?;
    println!("포스트 수정 완료: [{}] {}", post.id, post.title);
    Ok(())
}

async fn handle_post_delete(client: &mut BlogServiceClient<Channel>, json: &str) -> Result<()> {
    let input: IdInput = parse_json(json, r#"'{"id":"post:xxx"}'"#)?;
    let token = load_token()?;
    let res = client
        .delete_post(DeletePostRequest {
            token,
            id: input.id,
        })
        .await?
        .into_inner();
    println!(
        "{}",
        if res.success {
            "포스트가 삭제되었습니다."
        } else {
            "포스트 삭제에 실패했습니다."
        }
    );
    Ok(())
}

async fn handle_comment_create(client: &mut BlogServiceClient<Channel>, json: &str) -> Result<()> {
    let input: CommentCreateInput =
        parse_json(json, r#"'{"post_id":"post:xxx","content":"댓글 내용"}'"#)?;
    let token = load_token()?;
    let res = client
        .create_comment(CreateCommentRequest {
            token,
            post_id: input.post_id,
            content: input.content,
            visibility: input.visibility,
        })
        .await?
        .into_inner();
    let comment = res.comment.context("댓글 정보 없음")?;
    println!("댓글 생성 완료: [{}] {}", comment.id, comment.content);
    Ok(())
}

async fn handle_comment_list(client: &mut BlogServiceClient<Channel>, json: &str) -> Result<()> {
    let input: CommentListInput = parse_json(json, r#"'{"post_id":"post:xxx"}'"#)?;
    let token = load_token().unwrap_or_default();
    let res = client
        .list_comments(ListCommentsRequest {
            post_id: input.post_id,
            token,
        })
        .await?
        .into_inner();
    println!("댓글 목록 ({}건):", res.comments.len());
    for c in &res.comments {
        print_comment(c);
    }
    Ok(())
}

async fn handle_comment_update(client: &mut BlogServiceClient<Channel>, json: &str) -> Result<()> {
    let input: CommentUpdateInput =
        parse_json(json, r#"'{"id":"comment:xxx","content":"수정된 댓글"}'"#)?;
    let token = load_token()?;
    let res = client
        .update_comment(UpdateCommentRequest {
            token,
            id: input.id,
            content: input.content,
            visibility: input.visibility,
        })
        .await?
        .into_inner();
    let comment = res.comment.context("댓글 정보 없음")?;
    println!("댓글 수정 완료: [{}] {}", comment.id, comment.content);
    Ok(())
}

async fn handle_comment_delete(client: &mut BlogServiceClient<Channel>, json: &str) -> Result<()> {
    let input: IdInput = parse_json(json, r#"'{"id":"comment:xxx"}'"#)?;
    let token = load_token()?;
    let res = client
        .delete_comment(DeleteCommentRequest {
            token,
            id: input.id,
        })
        .await?
        .into_inner();
    println!(
        "{}",
        if res.success {
            "댓글이 삭제되었습니다."
        } else {
            "댓글 삭제에 실패했습니다."
        }
    );
    Ok(())
}

// ─── 관리자 핸들러 ────────────────────────────────────────────────────────────

async fn handle_admin_list_users(
    client: &mut BlogServiceClient<Channel>,
    json: Option<&str>,
) -> Result<()> {
    let input: PostListInput = match json {
        Some(j) => parse_json(j, r#"'{"page":1,"per_page":10}'"#)?,
        None => PostListInput {
            page: default_page(),
            per_page: default_per_page(),
            filter: String::new(),
        },
    };
    let token = load_token()?;
    let res = client
        .list_users(ListUsersRequest {
            token,
            page: input.page,
            per_page: input.per_page,
        })
        .await?
        .into_inner();
    println!("사용자 목록 (총 {}명, 페이지 {}):", res.total, input.page);
    for user in &res.users {
        print_user(user);
    }
    Ok(())
}

async fn handle_admin_get_user(client: &mut BlogServiceClient<Channel>, json: &str) -> Result<()> {
    let input: AdminUserIdInput = parse_json(json, r#"'{"user_id":"xxx"}'"#)?;
    let token = load_token()?;
    let res = client
        .get_user(GetUserRequest {
            token,
            user_id: input.user_id,
        })
        .await?
        .into_inner();
    let user = res.user.context("사용자 정보 없음")?;
    println!("사용자 정보:");
    print_user(&user);
    Ok(())
}

async fn handle_admin_update_role(
    client: &mut BlogServiceClient<Channel>,
    json: &str,
) -> Result<()> {
    let input: AdminUpdateRoleInput = parse_json(json, r#"'{"user_id":"xxx","role":"admin"}'"#)?;
    let token = load_token()?;
    let res = client
        .update_user_role(UpdateUserRoleRequest {
            token,
            user_id: input.user_id,
            role: input.role,
        })
        .await?
        .into_inner();
    let user = res.user.context("사용자 정보 없음")?;
    println!("역할 변경 완료: {} -> {}", user.username, user.role);
    Ok(())
}

async fn handle_admin_delete_user(
    client: &mut BlogServiceClient<Channel>,
    json: &str,
) -> Result<()> {
    let input: AdminUserIdInput = parse_json(json, r#"'{"user_id":"xxx"}'"#)?;
    let token = load_token()?;
    let res = client
        .delete_user(DeleteUserRequest {
            token,
            user_id: input.user_id,
        })
        .await?
        .into_inner();
    println!(
        "{}",
        if res.success {
            "사용자가 삭제되었습니다. (관련 포스트/댓글 포함)"
        } else {
            "사용자 삭제에 실패했습니다."
        }
    );
    Ok(())
}

async fn handle_admin_update_visibility(
    client: &mut BlogServiceClient<Channel>,
    json: &str,
) -> Result<()> {
    let input: AdminUpdateVisibilityInput =
        parse_json(json, r#"'{"post_id":"xxx","visibility":"private"}'"#)?;
    let token = load_token()?;
    let res = client
        .update_post_visibility(UpdatePostVisibilityRequest {
            token,
            post_id: input.post_id,
            visibility: input.visibility,
        })
        .await?
        .into_inner();
    let post = res.post.context("포스트 정보 없음")?;
    println!(
        "공개범위 변경 완료: [{}] {} -> {}",
        post.id, post.title, post.visibility
    );
    Ok(())
}

// ─── main ─────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let command = match cli.command {
        Some(cmd) => cmd,
        None => {
            Cli::command().print_help()?;
            return Ok(());
        }
    };

    let mut client = connect(&cli.server).await?;

    match command {
        Commands::Version => handle_version(&mut client).await,
        Commands::Register { json } => handle_register(&mut client, &json).await,
        Commands::Login { json } => handle_login(&mut client, &json).await,
        Commands::Profile { command } => match command {
            ProfileCommands::Me => handle_profile_me(&mut client).await,
            ProfileCommands::ChangePassword { json } => {
                handle_change_password(&mut client, &json).await
            }
            ProfileCommands::DeleteAccount { json } => {
                handle_delete_account(&mut client, &json).await
            }
        },
        Commands::Post { command } => match command {
            PostCommands::Create { json } => handle_post_create(&mut client, &json).await,
            PostCommands::List { json } => handle_post_list(&mut client, json.as_deref()).await,
            PostCommands::Get { json } => handle_post_get(&mut client, &json).await,
            PostCommands::Update { json } => handle_post_update(&mut client, &json).await,
            PostCommands::Delete { json } => handle_post_delete(&mut client, &json).await,
            PostCommands::Search { json } => handle_post_search(&mut client, &json).await,
        },
        Commands::Comment { command } => match command {
            CommentCommands::Create { json } => handle_comment_create(&mut client, &json).await,
            CommentCommands::List { json } => handle_comment_list(&mut client, &json).await,
            CommentCommands::Update { json } => handle_comment_update(&mut client, &json).await,
            CommentCommands::Delete { json } => handle_comment_delete(&mut client, &json).await,
        },
        Commands::Admin { command } => match command {
            AdminCommands::ListUsers { json } => {
                handle_admin_list_users(&mut client, json.as_deref()).await
            }
            AdminCommands::GetUser { json } => handle_admin_get_user(&mut client, &json).await,
            AdminCommands::UpdateRole { json } => {
                handle_admin_update_role(&mut client, &json).await
            }
            AdminCommands::DeleteUser { json } => {
                handle_admin_delete_user(&mut client, &json).await
            }
            AdminCommands::UpdateVisibility { json } => {
                handle_admin_update_visibility(&mut client, &json).await
            }
        },
    }
}
