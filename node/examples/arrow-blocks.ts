import { Client, RequestFormats } from "pangea-client";
import { tableFromIPC } from "apache-arrow";

require("dotenv").config({ override: true });

export async function main(): Promise<void> {
  const client = await Client.build();

  try {
    const handle = await client.get_blocks(
      {
        chains: ["ETH"],
        from_block: -10,
        to_block: "latest",
      },
      RequestFormats.ARROW,
    );

    for await (const chunk of handle) {
      const table = tableFromIPC(chunk);
      console.table([...table]);
    }
  } finally {
    await client.disconnect();
  }
}
