import os

root_index = os.path.join(os.path.dirname(__file__), "..", "..", "index.html")
root_index = os.path.abspath(root_index)

with open(root_index, "r", encoding="utf-8") as f:
    text = f.read()

target = '<a href="./portal/"'
replacement = '<a href="./portal/login.html" style="display: flex; align-items: center; gap: 6px; padding: 6px 14px; border-radius: 20px; background: rgba(30, 41, 59, 0.88); backdrop-filter: blur(10px); color: #38bdf8; text-decoration: none; border: 1px solid rgba(56, 189, 248, 0.35); box-shadow: 0 4px 12px rgba(0,0,0,0.25); font-weight: 600;"><span>🔑 Sign In</span></a>\n    <a href="./portal/"'

if target in text:
    text = text.replace(target, replacement, 1)
    with open(root_index, "w", encoding="utf-8") as f:
        f.write(text)
    print("index.html updated successfully with Sign In button")
else:
    print("Target not found")
