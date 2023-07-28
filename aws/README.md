# Rust for AWS

## SAM

### Prerequsites

- `rustup`
- AWS accout 
- `aws` CLI
- `sam` CLI

### How to create Lambda project with `sam` CLI

```bash
$ sam init

        SAM CLI now collects telemetry to better understand customer needs.

        You can OPT OUT and disable telemetry collection by setting the
        environment variable SAM_CLI_TELEMETRY=0 in your shell.
        Thanks for your help!

        Learn More: https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-telemetry.html


You can preselect a particular runtime or package type when using the `sam init` experience.
Call `sam init --help` to learn more.

Which template source would you like to use?
        1 - AWS Quick Start Templates
        2 - Custom Template Location
Choice: 1

Choose an AWS Quick Start application template
        1 - Hello World Example
        2 - Data processing
        3 - Hello World Example with Powertools for AWS Lambda
        4 - Multi-step workflow
        5 - Scheduled task
        6 - Standalone function
        7 - Serverless API
        8 - Infrastructure event management
        9 - Lambda Response Streaming
        10 - Serverless Connector Hello World Example
        11 - Multi-step workflow with Connectors
        12 - Full Stack
        13 - Lambda EFS example
        14 - Hello World Example With Powertools
        15 - DynamoDB Example
        16 - Machine Learning
Template: 1

Use the most popular runtime and package type? (Python and zip) [y/N]: N

Which runtime would you like to use?
        1 - aot.dotnet7 (provided.al2)
        2 - dotnet6
        3 - go1.x
        4 - go (provided.al2)
        5 - graalvm.java11 (provided.al2)
        6 - graalvm.java17 (provided.al2)
        7 - java17
        8 - java11
        9 - java8.al2
        10 - java8
        11 - nodejs18.x
        12 - nodejs16.x
        13 - nodejs14.x
        14 - nodejs12.x
        15 - python3.9
        16 - python3.8
        17 - python3.7
        18 - python3.11
        19 - python3.10
        20 - ruby3.2
        21 - ruby2.7
        22 - rust (provided.al2)
Runtime: 22

Based on your selections, the only Package type available is Zip.
We will proceed to selecting the Package type as Zip.

Based on your selections, the only dependency manager available is cargo.
We will proceed copying the template using cargo.

Would you like to enable X-Ray tracing on the function(s) in your application?  [y/N]: N

Would you like to enable monitoring using CloudWatch Application Insights?
For more info, please view https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch-application-insights.html [y/N]: N

Project name [sam-app]: rust-sam-app
                                                                                                                                                               
Cloning from https://github.com/aws/aws-sam-cli-app-templates (process may take a moment)                                                                      

    -----------------------
    Generating application:
    -----------------------
    Name: rust-sam-app
    Runtime: rust (provided.al2)
    Architectures: x86_64
    Dependency Manager: cargo
    Application Template: hello-world
    Output Directory: .
    Configuration file: rust-sam-app/samconfig.toml
    
    Next steps can be found in the README file at rust-sam-app/README.md
        

Commands you can use next
=========================
[*] Create pipeline: cd rust-sam-app && sam pipeline init --bootstrap
[*] Validate SAM template: cd rust-sam-app && sam validate
[*] Test Function in the Cloud: cd rust-sam-app && sam sync --stack-name {stack-name} --watch
```

### How to build Lambda project

```bash
$ vi samconfig.toml
```

```toml
# samconfig.toml
[default.build.parameters]
beta_features = true

[default.sync.paramters]
beta_features = true
```

```bash
$sam build
$ sam validate
```

### How to test Lambda project locally

```bash
$ sam local invoke

# or

$ sam local invoke --event events/event.json
```

### How to test Lambda project with API Gateway locally

```bash
$ sam local start-api
```

and run command below in another terminal

```bash
$ curl http://localhost:3000/hello
```


### How to deploy Lambda project to AWS

```bash
$ sam sync --stack-name rust-sam-app --watch # for sync 

# or 

$ sam deploy 

# or

$ sam deploy --guided

Configuring SAM deploy
======================

        Looking for config file [samconfig.toml] :  Found
        Reading default arguments  :  Success

        Setting default arguments for 'sam deploy'
        =========================================
        Stack Name [rust-sam-app]: 
        AWS Region [ap-northeast-2]: 
        #Shows you resources changes to be deployed and require a 'Y' to initiate deploy
        Confirm changes before deploy [Y/n]: N
        #SAM needs permission to be able to create roles to connect to the resources in your template
        Allow SAM CLI IAM role creation [Y/n]: 
        #Preserves the state of previously provisioned resources when an operation fails
        Disable rollback [y/N]: 
        HelloWorldFunction has no authentication. Is this okay? [y/N]: Y
        Save arguments to configuration file [Y/n]: N
...
```

### How to test Lambda project

```bash
$ aws lambda invoke
  --cli-binary-format raw-in-base64-out \
  --function-name HelloWorldFunction-XXXXXXXX \ # Replace with the actual function name
  --payload '{"command": "Say Hi!"}' \
  output.json
$ cat output.json  # Prints: {"msg": "Command Say Hi! executed."}
```

### How to delete Lambda project

```bash
$ sam delete
        Are you sure you want to delete the stack rust-sam-app in the region None ? [y/N]: y
        Are you sure you want to delete the folder rust-sam-app in S3 which contains the artifacts? [y/N]: y
```
