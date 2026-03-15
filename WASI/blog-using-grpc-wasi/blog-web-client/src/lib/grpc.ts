import * as grpc from '@grpc/grpc-js';
import type {
	GrpcObject,
	ServiceClientConstructor,
	ServiceError,
	ClientUnaryCall
} from '@grpc/grpc-js';
import * as protoLoader from '@grpc/proto-loader';
import { resolve } from 'node:path';

const PROTO_PATH = process.env.PROTO_PATH ?? resolve(process.cwd(), 'proto/blog.proto');
const SERVER_ADDR = process.env.SERVER_ADDR ?? 'localhost:50051';

const packageDef = protoLoader.loadSync(PROTO_PATH, {
	keepCase: true,
	longs: String,
	enums: String,
	defaults: true,
	oneofs: true
});

// --- Types ---

export interface UserInfo {
	id: string;
	username: string;
	email: string;
	created_at: string;
}

export interface AuthResult {
	token: string;
	user: UserInfo;
}

export interface Post {
	id: string;
	title: string;
	content: string;
	author: UserInfo;
	created_at: string;
	updated_at: string;
	comment_count: number;
}

export interface Comment {
	id: string;
	content: string;
	author: UserInfo;
	post_id: string;
	created_at: string;
}

// --- gRPC Client ---

interface BlogServiceClient extends grpc.Client {
	Register(
		req: { username: string; email: string; password: string },
		cb: (err: ServiceError | null, res: AuthResult) => void
	): ClientUnaryCall;
	Login(
		req: { email: string; password: string },
		cb: (err: ServiceError | null, res: AuthResult) => void
	): ClientUnaryCall;
	CreatePost(
		req: { token: string; title: string; content: string },
		cb: (err: ServiceError | null, res: { post: Post }) => void
	): ClientUnaryCall;
	GetPost(
		req: { id: string },
		cb: (err: ServiceError | null, res: { post: Post }) => void
	): ClientUnaryCall;
	ListPosts(
		req: { page: number; per_page: number },
		cb: (err: ServiceError | null, res: { posts: Post[]; total: number }) => void
	): ClientUnaryCall;
	UpdatePost(
		req: { token: string; id: string; title: string; content: string },
		cb: (err: ServiceError | null, res: { post: Post }) => void
	): ClientUnaryCall;
	DeletePost(
		req: { token: string; id: string },
		cb: (err: ServiceError | null, res: { success: boolean }) => void
	): ClientUnaryCall;
	CreateComment(
		req: { token: string; post_id: string; content: string },
		cb: (err: ServiceError | null, res: { comment: Comment }) => void
	): ClientUnaryCall;
	ListComments(
		req: { post_id: string },
		cb: (err: ServiceError | null, res: { comments: Comment[] }) => void
	): ClientUnaryCall;
	DeleteComment(
		req: { token: string; id: string },
		cb: (err: ServiceError | null, res: { success: boolean }) => void
	): ClientUnaryCall;
	GetVersion(
		req: Record<string, never>,
		cb: (err: ServiceError | null, res: { version: string }) => void
	): ClientUnaryCall;
}

const grpcObj = grpc.loadPackageDefinition(packageDef);
const blogPkg = grpcObj['blog'] as GrpcObject;
const BlogServiceCtor = blogPkg['BlogService'] as ServiceClientConstructor;

let _client: BlogServiceClient | null = null;

function getClient(): BlogServiceClient {
	if (!_client) {
		_client = new BlogServiceCtor(
			SERVER_ADDR,
			grpc.credentials.createInsecure()
		) as unknown as BlogServiceClient;
	}
	return _client;
}

// --- Auth ---

export function register(
	username: string,
	email: string,
	password: string
): Promise<AuthResult> {
	return new Promise((resolve, reject) => {
		getClient().Register({ username, email, password }, (err, res) => {
			if (err) reject(err);
			else resolve(res);
		});
	});
}

export function login(email: string, password: string): Promise<AuthResult> {
	return new Promise((resolve, reject) => {
		getClient().Login({ email, password }, (err, res) => {
			if (err) reject(err);
			else resolve(res);
		});
	});
}

// --- Posts ---

export function createPost(
	token: string,
	title: string,
	content: string
): Promise<Post> {
	return new Promise((resolve, reject) => {
		getClient().CreatePost({ token, title, content }, (err, res) => {
			if (err) reject(err);
			else resolve(res.post);
		});
	});
}

export function getPost(id: string): Promise<Post> {
	return new Promise((resolve, reject) => {
		getClient().GetPost({ id }, (err, res) => {
			if (err) reject(err);
			else resolve(res.post);
		});
	});
}

export function listPosts(
	page: number,
	perPage: number
): Promise<{ posts: Post[]; total: number }> {
	return new Promise((resolve, reject) => {
		getClient().ListPosts({ page, per_page: perPage }, (err, res) => {
			if (err) reject(err);
			else resolve(res);
		});
	});
}

export function updatePost(
	token: string,
	id: string,
	title: string,
	content: string
): Promise<Post> {
	return new Promise((resolve, reject) => {
		getClient().UpdatePost({ token, id, title, content }, (err, res) => {
			if (err) reject(err);
			else resolve(res.post);
		});
	});
}

export function deletePost(token: string, id: string): Promise<boolean> {
	return new Promise((resolve, reject) => {
		getClient().DeletePost({ token, id }, (err, res) => {
			if (err) reject(err);
			else resolve(res.success);
		});
	});
}

// --- Comments ---

export function createComment(
	token: string,
	postId: string,
	content: string
): Promise<Comment> {
	return new Promise((resolve, reject) => {
		getClient().CreateComment({ token, post_id: postId, content }, (err, res) => {
			if (err) reject(err);
			else resolve(res.comment);
		});
	});
}

export function listComments(postId: string): Promise<Comment[]> {
	return new Promise((resolve, reject) => {
		getClient().ListComments({ post_id: postId }, (err, res) => {
			if (err) reject(err);
			else resolve(res.comments ?? []);
		});
	});
}

export function deleteComment(token: string, id: string): Promise<boolean> {
	return new Promise((resolve, reject) => {
		getClient().DeleteComment({ token, id }, (err, res) => {
			if (err) reject(err);
			else resolve(res.success);
		});
	});
}

// --- System ---

export function getVersion(): Promise<string> {
	return new Promise((resolve, reject) => {
		getClient().GetVersion({}, (err, res) => {
			if (err) reject(err);
			else resolve(res.version);
		});
	});
}
