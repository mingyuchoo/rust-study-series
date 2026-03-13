import * as grpc from '@grpc/grpc-js';
import type { GrpcObject, ServiceClientConstructor, ServiceError, ClientUnaryCall } from '@grpc/grpc-js';
import * as protoLoader from '@grpc/proto-loader';
import { resolve } from 'path';

// proto/ is bundled inside greeting-web-client for self-contained deployment.
// Override with PROTO_PATH env var if needed.
const PROTO_PATH = process.env.PROTO_PATH ?? resolve(process.cwd(), 'proto/greeting.proto');

const SERVER_ADDR = process.env.SERVER_ADDR ?? 'localhost:50051';

const packageDef = protoLoader.loadSync(PROTO_PATH, {
	keepCase: true,
	longs: String,
	enums: String,
	defaults: true,
	oneofs: true
});

interface GreetResponse {
	message: string;
}

interface VersionResponse {
	version: string;
}

interface GreetingServiceClient extends grpc.Client {
	Greet(
		req: { name: string },
		cb: (err: ServiceError | null, res: GreetResponse) => void
	): ClientUnaryCall;
	GetVersion(
		req: Record<string, never>,
		cb: (err: ServiceError | null, res: VersionResponse) => void
	): ClientUnaryCall;
}

const grpcObj = grpc.loadPackageDefinition(packageDef);
const greetingPkg = grpcObj['greeting'] as GrpcObject;
const GreetingServiceCtor = greetingPkg['GreetingService'] as ServiceClientConstructor;

// Singleton client — reuse the connection across requests.
let _client: GreetingServiceClient | null = null;

function getClient(): GreetingServiceClient {
	if (!_client) {
		_client = new GreetingServiceCtor(
			SERVER_ADDR,
			grpc.credentials.createInsecure()
		) as unknown as GreetingServiceClient;
	}
	return _client;
}

export function greet(name: string): Promise<string> {
	return new Promise((resolve, reject) => {
		getClient().Greet({ name }, (err, response) => {
			if (err) reject(err);
			else resolve(response.message);
		});
	});
}

export function getVersion(): Promise<string> {
	return new Promise((resolve, reject) => {
		getClient().GetVersion({}, (err, response) => {
			if (err) reject(err);
			else resolve(response.version);
		});
	});
}
