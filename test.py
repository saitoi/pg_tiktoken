# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "tiktoken",
# ]
# ///

import sys, tiktoken

selector = sys.argv[1] if len(sys.argv) > 1 else "o200k_base"
text = "This is a test         with a lot of spaces<|endoftext|>"

try:
    enc = tiktoken.encoding_for_model(selector)
except KeyError:
    enc = tiktoken.get_encoding(selector)

try:
    tokens = enc.encode(text, allowed_special={"all"})
except ValueError:
    try:
        tokens = enc.encode(text, allowed_special={"<|endoftext|>"})
    except ValueError:
        tokens = enc.encode(text, disallowed_special=())

print(tokens)
print(len(tokens))
