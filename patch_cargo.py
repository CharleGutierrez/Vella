import re
with open('Cargo.toml', 'r', encoding='utf-8') as f:
    text = f.read()

text = text.replace('rhai = "1.18"', 'rhai = { version = "1.18", features = ["serde", "sync"] }')

with open('Cargo.toml', 'w', encoding='utf-8') as f:
    f.write(text)
