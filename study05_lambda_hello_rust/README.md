# Hello Rust

## 시작하기 전에

`yarn` 명령어를 사용하려면 `npm install --global yarn`으로 먼저 설치하세요.
`yarn`이 사용된 부분을 `npm run` 으로 바꿔서 실행해도 됩니다.

- AWS 계정이 있어야 합니다.
- `aws cli`, `sam cli`를 설치하세요.
- `aws configure` 명령을 이용해서 자신의 aws 정보를 저장하세요.
- AWS 콘솔에 들어가 S3 버켓을 만들고 `package.json`파일에 있는 `lambda-rust` 부분을 본인이 AWS에서 생성한 S3 버켓이름으로 바꿉니다.

## 빌드하는 방법

- `yarn clean`: `target`디렉토리와 `package.yaml` 파일을 삭제한다.
- `yarn cargo:build`: `src` 소스 파일을 트랜스파일해서 `target/x86_64-unknown-linux-musl/release`에 빌드한다.
- `yarn zip`: 빌드한 바이너리를 `rust.zip` 파일로 압축합니다.
- `yarn sam:validate`: `template.yaml` 파일에 오류가 없는지 검증합니다.
- `yarn sam:package`: `template.yaml`을 이용하여 `package.yaml` 파일을 만듭니다.
- `yarn sam:deploy`: `package.yaml` 을 이용하여 AWS 자원을 만듭니다.

## 참고할 페이지

- https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/
- https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/building-custom-runtimes.html
- http://arun-gupta.github.io/rust-lambda/
