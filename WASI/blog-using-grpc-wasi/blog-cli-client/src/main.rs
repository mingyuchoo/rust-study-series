use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use tonic::transport::Channel;

pub mod proto {
    tonic::include_proto!("blog");
}

use proto::blog_service_client::BlogServiceClient;
use proto::{
    CreateCommentRequest, CreatePostRequest, DeleteCommentRequest, DeletePostRequest,
    GetPostRequest, ListCommentsRequest, ListPostsRequest, LoginRequest, RegisterRequest,
    UpdatePostRequest, VersionRequest,
};

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
    Register {
        /// 사용자 이름
        #[arg(long)]
        username: String,
        /// 이메일
        #[arg(long)]
        email: String,
        /// 비밀번호
        #[arg(long)]
        password: String,
    },

    /// 로그인 (토큰을 로컬에 저장)
    Login {
        /// 이메일
        #[arg(long)]
        email: String,
        /// 비밀번호
        #[arg(long)]
        password: String,
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
}

#[derive(Subcommand)]
enum PostCommands {
    /// 포스트 작성 (인증 필요)
    Create {
        /// 포스트 제목
        #[arg(long)]
        title: String,
        /// 포스트 내용
        #[arg(long)]
        content: String,
    },

    /// 포스트 목록 조회
    List {
        /// 페이지 번호
        #[arg(long, default_value = "1")]
        page: u32,
        /// 페이지당 항목 수
        #[arg(long, default_value = "10")]
        per_page: u32,
    },

    /// 포스트 상세 조회
    Get {
        /// 포스트 ID
        id: String,
    },

    /// 포스트 수정 (인증 필요, 본인 포스트만)
    Update {
        /// 포스트 ID
        id: String,
        /// 새 제목
        #[arg(long)]
        title: String,
        /// 새 내용
        #[arg(long)]
        content: String,
    },

    /// 포스트 삭제 (인증 필요, 본인 포스트만)
    Delete {
        /// 포스트 ID
        id: String,
    },
}

#[derive(Subcommand)]
enum CommentCommands {
    /// 댓글 작성 (인증 필요)
    Create {
        /// 대상 포스트 ID
        #[arg(long)]
        post_id: String,
        /// 댓글 내용
        #[arg(long)]
        content: String,
    },

    /// 특정 포스트의 댓글 목록 조회
    List {
        /// 대상 포스트 ID
        post_id: String,
    },

    /// 댓글 삭제 (인증 필요, 본인 댓글만)
    Delete {
        /// 댓글 ID
        id: String,
    },
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

// ─── 출력 포맷팅 (순수 함수) ──────────────────────────────────────────────────

fn format_author(author: Option<&proto::UserInfo>) -> &str {
    author.map(|a| a.username.as_str()).unwrap_or("?")
}

fn print_post_summary(post: &proto::Post) {
    println!(
        "  [{}] {} (by {}, 댓글 {}개)",
        post.id,
        post.title,
        format_author(post.author.as_ref()),
        post.comment_count,
    );
}

fn print_post_detail(post: &proto::Post) {
    println!("제목:    {}", post.title);
    println!("작성자:  {}", format_author(post.author.as_ref()));
    println!("작성일:  {}", post.created_at);
    println!("수정일:  {}", post.updated_at);
    println!("댓글 수: {}", post.comment_count);
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

// ─── 명령 핸들러 ──────────────────────────────────────────────────────────────

async fn handle_version(client: &mut BlogServiceClient<Channel>) -> Result<()> {
    let res = client.get_version(VersionRequest {}).await?.into_inner();
    println!("서버 버전: {}", res.version);
    Ok(())
}

async fn handle_register(
    client: &mut BlogServiceClient<Channel>,
    username: String,
    email: String,
    password: String,
) -> Result<()> {
    let res = client
        .register(RegisterRequest {
            username,
            email,
            password,
        })
        .await?
        .into_inner();
    let user = res.user.context("사용자 정보 없음")?;
    println!("회원가입 완료: {} ({})", user.username, user.email);
    Ok(())
}

async fn handle_login(
    client: &mut BlogServiceClient<Channel>,
    email: String,
    password: String,
) -> Result<()> {
    let res = client
        .login(LoginRequest { email, password })
        .await?
        .into_inner();
    save_token(&res.token)?;
    let user = res.user.context("사용자 정보 없음")?;
    println!("로그인 성공: {}", user.username);
    println!("토큰이 저장되었습니다.");
    Ok(())
}

async fn handle_post_create(
    client: &mut BlogServiceClient<Channel>,
    title: String,
    content: String,
) -> Result<()> {
    let token = load_token()?;
    let res = client
        .create_post(CreatePostRequest {
            token,
            title,
            content,
        })
        .await?
        .into_inner();
    let post = res.post.context("포스트 정보 없음")?;
    println!("포스트 생성 완료");
    println!("  ID:   {}", post.id);
    println!("  제목: {}", post.title);
    Ok(())
}

async fn handle_post_list(
    client: &mut BlogServiceClient<Channel>,
    page: u32,
    per_page: u32,
) -> Result<()> {
    let res = client
        .list_posts(ListPostsRequest { page, per_page })
        .await?
        .into_inner();
    println!("포스트 목록 (총 {}건, 페이지 {}):", res.total, page);
    for post in &res.posts {
        print_post_summary(post);
    }
    Ok(())
}

async fn handle_post_get(client: &mut BlogServiceClient<Channel>, id: String) -> Result<()> {
    let res = client.get_post(GetPostRequest { id }).await?.into_inner();
    let post = res.post.context("포스트를 찾을 수 없습니다")?;
    print_post_detail(&post);
    Ok(())
}

async fn handle_post_update(
    client: &mut BlogServiceClient<Channel>,
    id: String,
    title: String,
    content: String,
) -> Result<()> {
    let token = load_token()?;
    let res = client
        .update_post(UpdatePostRequest {
            token,
            id,
            title,
            content,
        })
        .await?
        .into_inner();
    let post = res.post.context("포스트 정보 없음")?;
    println!("포스트 수정 완료");
    println!("  ID:   {}", post.id);
    println!("  제목: {}", post.title);
    Ok(())
}

async fn handle_post_delete(client: &mut BlogServiceClient<Channel>, id: String) -> Result<()> {
    let token = load_token()?;
    let res = client
        .delete_post(DeletePostRequest { token, id })
        .await?
        .into_inner();
    if res.success {
        println!("포스트가 삭제되었습니다.");
    } else {
        println!("포스트 삭제에 실패했습니다.");
    }
    Ok(())
}

async fn handle_comment_create(
    client: &mut BlogServiceClient<Channel>,
    post_id: String,
    content: String,
) -> Result<()> {
    let token = load_token()?;
    let res = client
        .create_comment(CreateCommentRequest {
            token,
            post_id,
            content,
        })
        .await?
        .into_inner();
    let comment = res.comment.context("댓글 정보 없음")?;
    println!("댓글 생성 완료");
    println!("  ID:   {}", comment.id);
    println!("  내용: {}", comment.content);
    Ok(())
}

async fn handle_comment_list(
    client: &mut BlogServiceClient<Channel>,
    post_id: String,
) -> Result<()> {
    let res = client
        .list_comments(ListCommentsRequest { post_id })
        .await?
        .into_inner();
    println!("댓글 목록 ({}건):", res.comments.len());
    for c in &res.comments {
        print_comment(c);
    }
    Ok(())
}

async fn handle_comment_delete(client: &mut BlogServiceClient<Channel>, id: String) -> Result<()> {
    let token = load_token()?;
    let res = client
        .delete_comment(DeleteCommentRequest { token, id })
        .await?
        .into_inner();
    if res.success {
        println!("댓글이 삭제되었습니다.");
    } else {
        println!("댓글 삭제에 실패했습니다.");
    }
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
        Commands::Register {
            username,
            email,
            password,
        } => handle_register(&mut client, username, email, password).await,
        Commands::Login { email, password } => handle_login(&mut client, email, password).await,
        Commands::Post { command } => match command {
            PostCommands::Create { title, content } => {
                handle_post_create(&mut client, title, content).await
            }
            PostCommands::List { page, per_page } => {
                handle_post_list(&mut client, page, per_page).await
            }
            PostCommands::Get { id } => handle_post_get(&mut client, id).await,
            PostCommands::Update { id, title, content } => {
                handle_post_update(&mut client, id, title, content).await
            }
            PostCommands::Delete { id } => handle_post_delete(&mut client, id).await,
        },
        Commands::Comment { command } => match command {
            CommentCommands::Create { post_id, content } => {
                handle_comment_create(&mut client, post_id, content).await
            }
            CommentCommands::List { post_id } => handle_comment_list(&mut client, post_id).await,
            CommentCommands::Delete { id } => handle_comment_delete(&mut client, id).await,
        },
    }
}
