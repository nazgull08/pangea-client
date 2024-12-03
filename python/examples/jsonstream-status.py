import os
from dotenv import load_dotenv
from pangea_client import Client
from pangea_client.types import Format
import asyncio
import json

load_dotenv(override=True)


async def main():
    async with Client() as client:
        handle = await client.get_status(format=Format.JsonStream)

        async for chunk in handle:
            lines = chunk.strip().split("\n")
            for line in lines:
                if line:
                    print(json.loads(line))


if __name__ == "__main__":
    asyncio.run(main())
