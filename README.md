[![build](https://github.com/FemLolStudio/s3-tokio/workflows/build/badge.svg)](https://github.com/FemLolStudio/s3-tokio/actions)
[![](https://img.shields.io/crates/v/s3-tokio.svg)](https://crates.io/crates/s3-tokio)
![](https://img.shields.io/crates/d/s3-tokio.svg)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/FemLolStudio/s3-tokio/blob/master/LICENSE.md)
<!-- [![Join the chat at https://gitter.im/durch/rust-s3](https://badges.gitter.im/durch/rust-s3.svg)](https://gitter.im/durch/rust-s3?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge) -->
# s3-tokio [docs](https://docs.rs/s3-tokio/latest/s3/)

Rust library for working with Amazon S3 or arbitrary S3 compatible APIs, fully compatible with **async/await** and `futures ^0.3`.

**Only `tokio` and `rustls` support.**

> [!NOTE]
> This repository is a fork of [rust-s3-async](https://github.com/aalekhpatel07/rust-s3-async)*(what is fork of [rust-s3](https://github.com/durch/rust-s3))* but with only `tokio` and `rustls` support. Also with updated dependencies. *(for example `hyper ^1`)*


## Intro

Modest interface towards Amazon S3, as well as S3 compatible object storage APIs such as Backblaze B2, Wasabi, Yandex, Minio, Cloudflare R2 or Google Cloud Storage.
Supports: `put`, `get`, `list`, `delete`, operations on `tags` and `location`, as well as `head`. 

Additionally, a dedicated `presign_get` `Bucket` method is available. This means you can upload to S3, and give the link to select people without having to worry about publicly accessible files on S3. This also means that you can give people 
a `PUT` presigned URL, meaning they can upload to a specific key in S3 for the duration of the presigned URL.

### Quick Start

Read and run examples from the `examples` folder, make sure you have valid credentials for the variant you're running.

```bash
# tokio, default
cargo run --example tokio

# minio
# First, start Minio on port 9000.
AWS_ACCESS_KEY_ID="minioadmin" \
AWS_SECRET_ACCESS_KEY="minioadmin" \
    cargo run --example minio

# r2
cargo run --example r2

# google cloud
cargo run --example google-cloud
```

### Features

There are a lot of various features that enable a wide variety of use cases, refer to `s3/Cargo.toml` for an exhaustive list. Below is a table of various useful features as well as a short description for each.

+ `default` - only `fail-on-err`
+ `fail-on-err` - `panic` on any error
+ `no-verify-ssl` - disable SSL verification for endpoints, useful for custom regions
+ `http` - enable http scheme (disabled by default)

### Path or subdomain style URLs and headers

`Bucket` struct provides constructors for `path-style` paths, `subdomain` style is the default. `Bucket` exposes methods for configuring and accessing `path-style` configuration.

### Buckets

|          |                                                                                         |
|----------|-----------------------------------------------------------------------------------------|
| `create` | [async](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.create)      |
| `delete` | [async](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.delete)      |
| `list`   | [async](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.list_buckets)|
| `exists` | [async](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.exists)|


### Presign

|          |                                                                                                     |
|----------|-----------------------------------------------------------------------------------------------------|
| `POST`   | [presign_put](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.presign_post)      |
| `PUT`    | [presign_put](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.presign_put)       |
| `GET`    | [presign_get](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.presign_get)       |
| `DELETE` | [presign_delete](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.presign_delete) |

### GET

There are a few different options for getting an object and methods are generic over `tokio::io::AsyncWriteExt`.

|                             |                                                                                                                 |
|-----------------------------|-----------------------------------------------------------------------------------------------------------------|
| `async/sync/async-blocking` | [get_object](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.get_object)                     |
| `async/sync/async-blocking` | [get_object_stream](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.get_object_stream)       |
| `async/sync/async-blocking` | [get_object_to_writer](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.get_object_to_writer) |

### PUT

Each `GET` method has a `PUT` companion, and `tokio` methods are generic over `tokio::io::AsyncReadExt`.

|                             |                                                                                                                                 |
|-----------------------------|---------------------------------------------------------------------------------------------------------------------------------|
| `async/sync/async-blocking` | [put_object](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.put_object)                                     |
| `async/sync/async-blocking` | [put_object_with_content_type](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.put_object_with_content_type) |
| `async/sync/async-blocking` | [put_object_stream](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.put_object_stream)                       |

### List

|                             |                                                                                 |
|-----------------------------|---------------------------------------------------------------------------------|
| `async/sync/async-blocking` | [list](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.list) |

### DELETE

|                             |                                                                                                   |
|-----------------------------|---------------------------------------------------------------------------------------------------|
| `async/sync/async-blocking` | [delete_object](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.delete_object) |

### Location

|                             |                                                                                         |
|-----------------------------|-----------------------------------------------------------------------------------------|
| `async/sync/async-blocking` | [location](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.location) |

### Tagging

|                             |                                                                                                             |
|-----------------------------|-------------------------------------------------------------------------------------------------------------|
| `async/sync/async-blocking` | [put_object_tagging](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.put_object_tagging) |
| `async/sync/async-blocking` | [get_object_tagging](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.get_object_tagging) |

### Head

|                             |                                                                                               |
|-----------------------------|-----------------------------------------------------------------------------------------------|
| `async/sync/async-blocking` | [head_object](https://docs.rs/s3-tokio/latest/s3/bucket/struct.Bucket.html#method.head_object) |

## Usage (in `Cargo.toml`)

```toml
[dependencies]
s3-tokio = "0.39"
```

