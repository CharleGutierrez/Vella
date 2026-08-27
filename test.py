with open('README.md', 'r', encoding='utf-8') as f:
    text = f.read()

text = text.replace('\ufffd', '—')

with open('README.md', 'w', encoding='utf-8') as f:
    f.write(text)
