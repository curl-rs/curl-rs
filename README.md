> [!WARNING]
> This is not ready for production yet or usage, still in development! In the meanwhile use mainstream `curl`

# Curl-RS
A cross-platform rewrite of curl in rust.

## Features
- Memory Safety Rust (Reduces buffer overflows, runtime race conditions)
- Built with Rust (Uses exisiting libraries like [Hyper][Hyper]) 
- No openssl needed, unless [rustls][Rustls] [doesn't supports it][RustlsSupport] for everything
- Built with modern features in mind with compatility for older features.


[Hyper]: https://github.com/hyperium/hyper
[Rustls]: https://github.com/rustls/rustls
[RustlsSupport]: https://docs.rs/rustls/latest/rustls/manual/_04_features/index.html
