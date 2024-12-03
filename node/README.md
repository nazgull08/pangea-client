# Pangea Client

`pangea-client` is a set of libraries for different languages to interface with Pangea for access to cross chain data, fast.

When using `node` the data is delivered in Arrow or JSON, developers are required to provide their own types.

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

Before proceeding, make sure to have node installed **globally**.
Easiest way is to install `nvm`, and use that to install the `node` version listed in `.nvmrc`. See the following:

- [`nvm`](https://github.com/nvm-sh/nvm/blob/master/README.md)
- [`node.js` ](https://nodejs.org/en/learn/getting-started/introduction-to-nodejs)

<br>

After node is installed, `cd` to the directory where you cloned the repo (`node` sub-folder for megarepo) and run:

```sh
npm i
```

<br>

## Running the Examples

To run any file in the `examples` folder, use the template

```sh
npm run example:${EXAMPLE_FILE_NAME}
```

Like so:

```sh
npm run example:arrow-blocks
npm run example:jsonstream-blocks
npm run example:jsonstream-status
...
```

<br>

NOTE: When adding a new example file, make sure to add the corresponding `package.json` script task to be able to run it.
