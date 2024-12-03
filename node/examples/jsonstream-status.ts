import { Client, RequestFormats } from "pangea-client";

require("dotenv").config({ override: true });

export async function main(): Promise<void> {
  const client = await Client.build();

  const handle = await client.get_status(RequestFormats.JSON_STREAM);

  try {
    for await (const chunk of handle) {
      chunk
        .toString()
        .split("\n")
        .filter(Boolean)
        .forEach((line: any) => {
          console.log(JSON.parse(line));
        });
    }
  } finally {
    client.disconnect();
  }
}
