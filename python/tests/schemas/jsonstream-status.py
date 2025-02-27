schema = {
    "type": "object",
    "properties": {
        "type": {"type": "string"},
        "chain": {"type": "integer"},
        "chain_code": {"type": "string"},
        "chain_name": {"type": "string"},
        "entity": {"type": "string"},
        "latest_block_height": {"type": "integer"},
        "service": {"type": "string"},
        "status": {"type": "string"},
        "timestamp": {"type": "integer"},
    },
    "required": [
        "type",
        "chain",
        "chain_code",
        "chain_name",
        "entity",
        "latest_block_height",
        "service",
        "status",
        "timestamp",
    ],
}
