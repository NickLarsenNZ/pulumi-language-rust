# Pulumi for Rust

_I have no idea what I'm doing_.

See [pulumi/pulumi#3622][language-request]

## Strategy

_Off the cuff_

1. Implement a [gRPC Server][grpc-server] in Rust to see if that is simple enough
   - Code generation to reduce human burden. See [logrocket-grpc-guide]
2. Implement a bare minimum [gRPC Client][grpc-client]
3. Make it resolve a single output
   ```rust
   use pulumi::pulumi;

   fn main() {
      pulumi.export("foo", "bar");
   }
   ```
4. Make it apply a resource
   - Maybe use S3 bucket as an example
5. Build up the sdk (like the imports in other languages)
   - `pulumi::pulumi`
   - `pulumi::aws`
   - ...
6. Make macros
   - macro rules for resource dsls (inspired by [clap])
     ```rust
     let cluster = aws::eks::cluster!(my_cluster =>
        (roleArn: "arn:aws::")
        (vpcConfig =>
            (vpc_id: "vpc-xyz")
            (subnet_ids: vec!["s-abc", "s-def", "s-ghi"])
        )
        (tags =>
            (Environment: "production")
            (Purpose: "monitoring")
        )
     )
     ```
   - proc macros for things like outputs/exports
     ```rust
     #[pulumi::main]
     async fn stack() {
        let bucket = ...;

        #[output]
        let bucket_name = bucket.name;
     }
     ```

See [wiki][language-implementation-guide] as a guide

[grpc-server]: https://github.com/pulumi/pulumi/blob/master/sdk/proto/language.proto
[grpc-client]: https://github.com/pulumi/pulumi/blob/master/sdk/proto/resource.proto
[language-request]: https://github.com/pulumi/pulumi/issues/3622
[clap]: https://github.com/clap-rs/clap#using-macros
[language-implementation-guide]: https://github.com/pulumi/pulumi/wiki/New-Language-Bring-Up
[logrocket-grpc-guide]: https://blog.logrocket.com/rust-and-grpc-a-complete-guide/
