import os

portal_index = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "portal", "index.html"))
portal_login = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "portal", "login.html"))

# 1. Update portal/index.html
with open(portal_index, "r", encoding="utf-8") as f:
    text = f.read()

# Change default body to light theme
text = text.replace('<body data-theme="dark">', '<body data-theme="light">')

# Enhance [data-theme="light"] CSS with complete styling
light_css_target = '''    [data-theme="light"] {
      --bg-body: #F1F5F9;
      --bg-surface: #FFFFFF;
      --bg-card: #F8FAFC;
      --bg-card-hover: #EDF2F7;
      --border-color: #E2E8F0;
      --border-accent: #CBD5E1;
      --primary: #0284C7;
      --primary-hover: #0369A1;
      --primary-dark: #075985;
      --primary-rgb: 2, 132, 199;
      --text-main: #0F172A;
      --text-muted: #475569;
      --text-dim: #94A3B8;
      --shadow-sm: 0 2px 4px rgba(0, 0, 0, 0.05);
      --shadow-md: 0 4px 12px rgba(0, 0, 0, 0.08);
      --shadow-lg: 0 8px 24px rgba(0, 0, 0, 0.12);
    }'''

light_css_replacement = '''    [data-theme="light"] {
      --bg-body: #F8FAFC;
      --bg-surface: #FFFFFF;
      --bg-card: #FFFFFF;
      --bg-card-hover: #F1F5F9;
      --border-color: #E2E8F0;
      --border-accent: #CBD5E1;
      --primary: #0284C7;
      --primary-hover: #0369A1;
      --primary-dark: #075985;
      --primary-rgb: 2, 132, 199;
      --accent: #D97706;
      --success: #059669;
      --warning: #D97706;
      --danger: #DC2626;
      --critical: #B91C1C;
      --text-main: #0F172A;
      --text-muted: #475569;
      --text-dim: #64748B;
      --shadow-sm: 0 1px 3px rgba(0, 0, 0, 0.08);
      --shadow-md: 0 4px 12px rgba(0, 0, 0, 0.08);
      --shadow-lg: 0 10px 30px rgba(0, 0, 0, 0.12);
    }

    /* Light Theme Dedicated Refinements */
    [data-theme="light"] .btn-primary {
      background: #0284C7;
      color: #FFFFFF;
    }
    [data-theme="light"] .btn-primary:hover {
      background: #0369A1;
      color: #FFFFFF;
    }
    [data-theme="light"] .btn-secondary {
      background: #FFFFFF;
      color: #0F172A;
      border-color: #CBD5E1;
    }
    [data-theme="light"] .btn-secondary:hover {
      background: #F1F5F9;
      border-color: #0284C7;
    }
    [data-theme="light"] .map-card {
      background: rgba(255, 255, 255, 0.95);
      border-color: #CBD5E1;
      color: #0F172A;
      box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
    }
    [data-theme="light"] .map-tool-btn {
      background: rgba(255, 255, 255, 0.95);
      border-color: #CBD5E1;
      color: #0F172A;
      box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
    }
    [data-theme="light"] .map-tool-btn:hover,
    [data-theme="light"] .map-tool-btn.active {
      background: #0284C7;
      color: #FFFFFF;
      border-color: #0284C7;
    }
    [data-theme="light"] .coordinate-display {
      background: rgba(255, 255, 255, 0.92);
      border-color: #CBD5E1;
      color: #475569;
    }
    [data-theme="light"] .sld-container {
      background: #F8FAFC;
      border-color: #E2E8F0;
    }
    [data-theme="light"] #sldSvg text {
      fill: #1E293B !important;
    }
    [data-theme="light"] .telemetry-pill,
    [data-theme="light"] .user-profile-badge {
      background: #FFFFFF;
      border-color: #CBD5E1;
      color: #0F172A;
    }
    [data-theme="light"] .global-search input {
      background: #FFFFFF;
      border-color: #CBD5E1;
      color: #0F172A;
    }
    [data-theme="light"] .nav-sidebar {
      background: #FFFFFF;
      border-color: #E2E8F0;
    }
    [data-theme="light"] .nav-item:hover {
      background: #F1F5F9;
      color: #0284C7;
    }
    [data-theme="light"] .modal-overlay {
      background: rgba(15, 23, 42, 0.5);
    }
    [data-theme="light"] .modal-box {
      background: #FFFFFF;
      border-color: #CBD5E1;
      color: #0F172A;
    }
    [data-theme="light"] .form-control {
      background: #FFFFFF;
      border-color: #CBD5E1;
      color: #0F172A;
    }
    [data-theme="light"] .layer-item {
      background: #FFFFFF;
      border-color: #E2E8F0;
    }
    [data-theme="light"] .kanban-card {
      background: #FFFFFF;
      border-color: #E2E8F0;
    }
    [data-theme="light"] #traceTreeDisplay {
      background: #F8FAFC;
      color: #0F172A;
      border-color: #E2E8F0;
    }'''

if light_css_target in text:
    text = text.replace(light_css_target, light_css_replacement, 1)
    print("Light CSS variables updated in portal/index.html")

# Update Theme Toggle JS logic to remember preference in localStorage
old_theme_js = '''    // Dark / Light Theme Toggle
    const themeToggleBtn = document.getElementById("themeToggleBtn");
    themeToggleBtn.onclick = () => {
      const current = document.body.getAttribute("data-theme");
      const next = current === "dark" ? "light" : "dark";
      document.body.setAttribute("data-theme", next);
    };'''

new_theme_js = '''    // White / Dark Theme Toggle with persistence
    const themeToggleBtn = document.getElementById("themeToggleBtn");
    function applyTheme(theme) {
      document.body.setAttribute("data-theme", theme);
      localStorage.setItem("powergis_theme", theme);
      themeToggleBtn.innerText = theme === "light" ? "🌙" : "☀️";
      themeToggleBtn.title = theme === "light" ? "Switch to Dark Mode" : "Switch to White Theme";
    }

    const savedTheme = localStorage.getItem("powergis_theme") || "light"; // Default to White Theme!
    applyTheme(savedTheme);

    themeToggleBtn.onclick = () => {
      const current = document.body.getAttribute("data-theme");
      applyTheme(current === "light" ? "dark" : "light");
    };'''

if old_theme_js in text:
    text = text.replace(old_theme_js, new_theme_js, 1)
    print("Theme toggle JS logic updated with White theme default.")

with open(portal_index, "w", encoding="utf-8") as f:
    f.write(text)

print("portal/index.html written successfully!")
