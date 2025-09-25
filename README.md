# pg_tiktoken

### **This is a fork of [pg_tiktoken](https://github.com/kelvich/pg_tiktoken).**

Postgres extension that does input tokenization using OpenAI's tiktoken.

## Usage

```sql
db=> create extension pg_tiktoken;
CREATE EXTENSION
db=> select tiktoken_count('p50k_edit', 'A long time ago in a galaxy far, far away');
 tiktoken_count 
----------------
             11
(1 row)

db=> select tiktoken_encode('cl100k_base', 'A long time ago in a galaxy far, far away');
                  tiktoken_encode                   
----------------------------------------------------
 {32,1317,892,4227,304,264,34261,3117,11,3117,3201}
(1 row)
```

## Supported models

| Encoding name           | Codename                                                                  |
|-------------------------|---------------------------------------------------------------------------|
| `o200k_base`            | GPT-4o, GPT-4.1, O-series (`gpt-4o`, `gpt-4o-mini`, `gpt-4.1`, `gpt-4.1-mini`, `o1`, `o1-mini`, `o3`, `o3-mini`) |
| `cl100k_base`           | GPT-3.5, GPT-4, Embeddings (`text-embedding-3-large`, `text-embedding-3-small`, `text-embedding-ada-002`) |
| `p50k_base`             | GPT-3 base models (`text-davinci-003`, `text-davinci-002`, `text-curie-001`, `text-babbage-001`, `text-ada-001`) |
| `p50k_edit`             | Edit models (`text-davinci-edit-001`, `code-davinci-edit-001`)            |
| `r50k_base` (or `gpt2`) | GPT-3 older models like `davinci`, and GPT-2                              |

`tiktoken_count` and `tiktoken_encode` functions accept both encoding name and OpenAI model name as a first argument.

## Installation

Install the necessary PostgreSQL dependencies for your OS:

```sh
sudo apt-get install build-essential libreadline-dev zlib1g-dev flex bison \
    libxml2-dev libxslt-dev libssl-dev libxml2-utils xsltproc ccache pkg-config
```

Assuming that rust toolchain is already istalled:

```sh
# install pgrx
cargo install --locked cargo-pgrx
cargo pgrx init
# build and install pg_tiktoken
git clone https://github.com/kelvich/pg_tiktoken
cd pg_tiktoken
cargo pgrx install
```

or simply: `sudo ./deb-install.sh`.

## Kudos

- https://github.com/zurawiki/tiktoken-rs
- https://github.com/openai/tiktoken
