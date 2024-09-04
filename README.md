# Redis Prototype

This project is a basic implementation of a Redis-like in-memory key-value store written in Rust. It supports basic operations such as storing and retrieving strings, lists, and sets, along with concurrency support using Tokio for asynchronous tasks.

## Features

- **String Operations**: Set and get string values by key.
- **List Operations**: Push and pop elements from the left or right of lists.
- **Set Operations**: Add and remove elements from sets, retrieve all members.
- **Persistence**: Save and load database snapshots to and from a file.
- **Concurrency**: Supports concurrent access and modification of the database.
- **Asynchronous Server**: Runs an asynchronous server using Tokio to handle client requests.

## Getting Started
### Installation

Clone the repository:

```sh
git clone https://github.com/yourusername/redis-prototype.git
cd redis-prototype
cargo run --bin server
cargo test
```
## Usage

The server accepts commands via a TCP connection. You can connect to the server using a tool like `netcat` or write a client script.

Example commands:

- **SET**: `SET key1 value1\n`
- **GET**: `GET key1\n`
- **LPUSH**: `LPUSH mylist value1\n`
- **RPUSH**: `RPUSH mylist value2\n`
- **LPOP**: `LPOP mylist\n`
- **RPOP**: `RPOP mylist\n`
- **SADD**: `SADD myset value1\n`
- **SMEMBERS**: `SMEMBERS myset\n`


