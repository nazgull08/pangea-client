from jsonschema import validate
from unittest.mock import patch
import importlib
import pytest


# List of examples to test
examples = ["jsonstream-blocks", "jsonstream-status"]


@pytest.mark.parametrize("data_source", examples)
@pytest.mark.asyncio
async def test_json_stream_examples_schema(data_source):
    """
    Test JSON stream examples against their respective schemas.
    """

    example_module = importlib.import_module(f"examples.{data_source}")
    schema_module = importlib.import_module(f"tests.schemas.{data_source}")
    main = getattr(example_module, "main")
    schema = getattr(schema_module, "schema")

    # Spy on console output
    with patch("builtins.print") as mock_print:
        await main()

        for call_args in mock_print.call_args_list:
            logged_obj = call_args[0][0]
            validate(instance=logged_obj, schema=schema)
