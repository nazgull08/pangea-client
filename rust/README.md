# Pangea Client

`pangea-client` is a set of libraries for different languages to interface with Pangea for access to cross chain data, fast.

When using `rust` the data is delivered in Arrow or JSON, developers are required to provide their own types.

<br>

## Getting started

Access to the API via `pangea-client` requires credentials, please apply for access [here](https://pangea.foundation/get-access)

Once you have credentials set your environment variables:

You will be given a username and password to use to access the Pangea API.
The easiest way to use these credentials is to create a `.env` file in the same folder as this `README.md` file, and populate it like so:

```sh
PANGEA_USERNAME=xxxxx
PANGEA_PASSWORD=xxxxx
PANGEA_URL=app.pangea.foundation
```

<br>

## Examples

Checkout the github repository separately for example code.

## Setup

Before proceeding, make sure to have [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed **globally**.

<br>

After node is installed, `cd` to the directory where you cloned the repo (`rust` sub-folder for megarepo).

<br>

## Running the Examples

To run any file in the `examples` folder, use the template

```sh
cargo run --example ${EXAMPLE_FILE_NAME}
```

Like so:

```sh
cargo run --example arrow-blocks
cargo run --example jsonstream-blocks
cargo run --example jsonstream-status
...
```

<br>

When adding a new example, it is sufficient to place it in the `examples` folder, and use the same template as above to run it.
