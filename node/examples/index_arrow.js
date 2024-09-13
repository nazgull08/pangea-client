const { Client } = require('pangea');
const dotenv = require('dotenv');
const { tableFromIPC } = require('apache-arrow');

dotenv.config();

const username = process.env.PANGEA_USERNAME;
const password = process.env.PANGEA_PASSWORD;
const endpoint = process.env.PANGEA_URL || "app.pangea.foundation";

const client = new Client({ username, password, endpoint });

async function arrow() {
    const handle = await client.get_blocks({
        chains: ["ETH"],
    }, false, "arrow");
    for await (const chunk of handle) {
        const table = await tableFromIPC(chunk);
        console.table([...table]);
    }
}

arrow()
