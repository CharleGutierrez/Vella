import re

emoji_map = {
    'What is Vella?': '🤔',
    '1. The Autonomous Brain (Artificial Intelligence)': '🧠',
    '2. The Global Economy (Web3 & Cryptography)': '🌐',
    '3. The Financial Superweapon (High-Frequency Trading)': '⚡',
    '4. The Physical World (SCADA & DePIN)': '🏭',
    'Code Examples': '💻',
    'Spinning up a Headless CMS': '📝',
    'High-Frequency Trading Engine': '📈',
    'Web3 Deployer': '🔗',
    'Architecture': '🏗️',
    'Enterprise Security & Resilience': '🛡️',
    'NEW: The Vella VS Code Extension is Here!': '🚀',
    'Extreme Features': '✨',
    'Manual & Feature List': '📚',
    'Installation Guide': '📥',
    'Get Started': '🏁',
    'Run with Docker (Zero Setup)': '🐳',
    'Community & Contributing': '🤝',
    'What the AIs are saying about Vella': '🤖',
    'Claude (Anthropic)': '🧠',
    'Grok (xAI)': '✖️',
    'ChatGPT (OpenAI)': '💬',
    'GitHub Copilot': '🐙',
}

with open('README.md', 'rb') as f:
    lines = f.readlines()

out_lines = []
for line in lines:
    try:
        # try decoding as utf-8, replace bad chars
        text_line = line.decode('utf-8', errors='replace')
    except Exception:
        text_line = line.decode('utf-8', errors='ignore')
    
    match = re.match(r'^(#+)\s+(.*)', text_line)
    if match:
        hashes = match.group(1)
        text = match.group(2)
        
        # Find first alphanumeric char to strip out old emojis/garbage
        m_text = re.search(r'[A-Za-z0-9]', text)
        if m_text:
            idx = m_text.start()
            clean_text = text[idx:].strip()
            
            # special map
            emoji = '📌' # default
            for k, v in emoji_map.items():
                if clean_text.startswith(k):
                    emoji = v
                    break
            
            new_line_str = f"{hashes} {emoji} {clean_text}\n"
            out_lines.append(new_line_str.encode('utf-8'))
        else:
            out_lines.append(line)
    else:
        out_lines.append(line)

with open('README.md', 'wb') as f:
    f.writelines(out_lines)
