import os

portal_path = os.path.join(os.path.dirname(__file__), "..", "..", "portal", "index.html")
portal_path = os.path.abspath(portal_path)

with open(portal_path, "r", encoding="utf-8") as f:
    content = f.read()

# 1. Update Header to add Sign In / Sign Out buttons
header_target = '<div class="user-profile-badge" id="openUserModalBtn"'
header_replacement = '''<a href="./login.html" class="btn btn-secondary btn-sm" id="headerLoginBtn" style="display: none;" title="Go to Sign In Page">🔑 Sign In</a>
      <button class="btn btn-danger btn-sm" id="headerLogoutBtn" title="Sign out of engineering session">🚪 Logout</button>
      <div class="user-profile-badge" id="openUserModalBtn"'''

if header_target in content:
    content = content.replace(header_target, header_replacement, 1)
    print("Header successfully updated with Login/Logout buttons.")
else:
    print("WARNING: Header target not found.")

# 2. Add Login Page link in Sidebar
sidebar_target = '<div class="nav-item" data-view="settings-view">'
sidebar_replacement = '''<a href="./login.html" class="nav-item" style="text-decoration: none; color: var(--text-muted);">
          <div class="nav-left">
            <span class="nav-icon">🔑</span>
            <span class="nav-text">Login Page</span>
          </div>
          <span class="badge-count" style="color: var(--primary);">Auth</span>
        </a>
        <div class="nav-item" data-view="settings-view">'''

if sidebar_target in content:
    content = content.replace(sidebar_target, sidebar_replacement, 1)
    print("Sidebar successfully updated with Login Page link.")
else:
    print("WARNING: Sidebar target not found.")

# 3. Add Login Screen Gate Overlay right before Leaflet library
modal_target = '<!-- Leaflet Library -->'
login_gate_html = '''<!-- Integrated Enterprise Login Screen Gate Overlay -->
  <div class="modal-overlay" id="loginGateOverlay" style="z-index: 5000; background: rgba(7, 11, 18, 0.95); backdrop-filter: blur(12px);">
    <div class="modal-box" style="max-width: 900px; width: 95vw; background: var(--bg-surface); border: 1px solid var(--border-accent); box-shadow: 0 25px 60px rgba(0,0,0,0.8); display: grid; grid-template-columns: 400px 1fr; padding: 0; max-height: 90vh;">
      
      <!-- Left: Standard Sign In Form -->
      <div style="padding: 28px; border-right: 1px solid var(--border-color); display: flex; flex-direction: column; justify-content: center;">
        <div style="display: flex; align-items: center; gap: 10px; margin-bottom: 12px;">
          <div style="width: 32px; height: 32px; border-radius: 8px; background: linear-gradient(135deg, #0284C7, #38BDF8); display: flex; align-items: center; justify-content: center; font-size: 16px;">⚡</div>
          <div>
            <div style="font-weight: 700; font-size: 16px;">PowerGIS Portal</div>
            <div style="font-size: 10px; color: var(--primary); text-transform: uppercase; font-weight: 600;">Govt of Puducherry</div>
          </div>
        </div>
        <div style="font-size: 18px; font-weight: 700; margin-bottom: 4px;">Sign In to PowerGIS</div>
        <div style="font-size: 12px; color: var(--text-muted); margin-bottom: 18px;">Puducherry Electricity Department Operations</div>

        <div id="gateAlert" style="display: none; padding: 8px 12px; border-radius: 6px; font-size: 12px; margin-bottom: 14px; background: rgba(239, 68, 68, 0.15); color: #FCA5A5; border: 1px solid rgba(239, 68, 68, 0.3);"></div>

        <form id="gateLoginForm" onsubmit="handleGateLogin(event)">
          <div class="form-group" style="margin-bottom: 12px;">
            <label class="form-label" style="margin-bottom: 4px;">Official Email or Username</label>
            <input type="text" class="form-control" id="gateUserInput" value="ee.distribution@powergis.gov.in" placeholder="name@powergis.gov.in" required>
          </div>
          <div class="form-group" style="margin-bottom: 14px;">
            <label class="form-label" style="margin-bottom: 4px;">Password</label>
            <input type="password" class="form-control" id="gatePassInput" value="PowerGIS@2026" required>
          </div>
          <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; font-size: 12px;">
            <label style="display: flex; align-items: center; gap: 6px; cursor: pointer; color: var(--text-muted);">
              <input type="checkbox" id="gateRememberMe" checked>
              <span>Remember me</span>
            </label>
            <a href="./login.html" style="color: var(--primary); text-decoration: none;">Forgot password?</a>
          </div>
          <button type="submit" class="btn btn-primary" style="width: 100%; padding: 10px; font-weight: 700;">Sign In ➔</button>
        </form>

        <div style="margin-top: 14px; text-align: center;">
          <button type="button" class="btn btn-secondary btn-sm" id="gateGuestBtn" style="width: 100%;">Continue as Guest (Read-Only Viewer)</button>
        </div>
      </div>

      <!-- Right: Quick 1-Click Role Logins -->
      <div style="padding: 24px; background: var(--bg-card); display: flex; flex-direction: column; overflow-y: auto; max-height: 520px;">
        <div style="font-weight: 700; font-size: 13px; display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;">
          <span>⚡ Instant 1-Click Role Logins</span>
          <span class="badge badge-primary">16 Roles</span>
        </div>
        <div style="font-size: 11px; color: var(--text-muted); margin-bottom: 12px;">Select any engineering designation below to log in immediately:</div>
        
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 6px;" id="gateRoleGrid">
          <!-- Populated by JS -->
        </div>

        <div style="margin-top: 14px; padding: 10px; border-radius: 6px; background: rgba(7, 11, 18, 0.6); border: 1px solid var(--border-color); font-size: 11px; color: var(--text-muted);">
          <b>Universal Demo Password:</b> <code style="color: var(--primary);">PowerGIS@2026</code><br>
          PostgreSQL 16 + PostGIS 3.4 &bull; Full RBAC Enabled
        </div>
      </div>

    </div>
  </div>

  '''

if modal_target in content:
    content = content.replace(modal_target, login_gate_html + modal_target, 1)
    print("Login Gate Overlay HTML successfully injected.")
else:
    print("WARNING: Modal target not found.")

# 4. Add Authentication and Session Management JavaScript
js_target = 'console.log("⚡ PowerGIS Enterprise GIS Portal Initialized (16 Roles, SCADA, PostGIS 16 ready).");'
auth_js = '''
    // =========================================================================
    // 15. AUTHENTICATION, LOGIN GATE & SESSION MANAGEMENT
    // =========================================================================
    const loginGateOverlay = document.getElementById("loginGateOverlay");
    const gateRoleGrid = document.getElementById("gateRoleGrid");
    const headerLoginBtn = document.getElementById("headerLoginBtn");
    const headerLogoutBtn = document.getElementById("headerLogoutBtn");

    // Populate the 16 1-Click login buttons on the login gate
    ALL_16_ROLES.forEach(r => {
      const btn = document.createElement("button");
      btn.type = "button";
      btn.style.background = "var(--bg-surface)";
      btn.style.border = "1px solid var(--border-color)";
      btn.style.borderRadius = "5px";
      btn.style.padding = "6px 8px";
      btn.style.textAlign = "left";
      btn.style.cursor = "pointer";
      btn.style.display = "flex";
      btn.style.flexDirection = "column";
      btn.style.gap = "2px";
      btn.innerHTML = `
        <div style="font-size: 11px; font-weight: 600; color: var(--text-main); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">${r.name}</div>
        <div style="font-size: 10px; color: var(--primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">${r.officer}</div>
      `;
      btn.onclick = () => {
        completeSessionLogin(r);
      };
      gateRoleGrid.appendChild(btn);
    });

    function completeSessionLogin(roleObj) {
      currentRoleObj = roleObj;
      const session = {
        token: "jwt_powergis_" + btoa(roleObj.id + ":" + Date.now()),
        user: roleObj,
        loginTime: new Date().toISOString()
      };
      localStorage.setItem("powergis_session", JSON.stringify(session));
      updateSessionUI(roleObj);
      loginGateOverlay.classList.remove("active");
    }

    function updateSessionUI(roleObj) {
      document.getElementById("headerUserName").innerText = roleObj.officer;
      document.getElementById("headerRoleName").innerText = roleObj.name;
      document.getElementById("headerAvatar").innerText = roleObj.name.split(" ").map(w => w[0]).slice(0, 2).join("");
      headerLogoutBtn.style.display = "inline-flex";
      headerLoginBtn.style.display = "none";
      render16RolesDirectory();
    }

    window.handleGateLogin = function(e) {
      e.preventDefault();
      const userVal = document.getElementById("gateUserInput").value.trim().toLowerCase();
      const passVal = document.getElementById("gatePassInput").value;
      const alertBox = document.getElementById("gateAlert");

      const match = ALL_16_ROLES.find(r => 
        r.id.toLowerCase() === userVal || 
        r.email.toLowerCase() === userVal ||
        r.officer.toLowerCase().includes(userVal)
      );

      if (!match) {
        alertBox.innerText = "Invalid user account. Use 1-click login or check credentials.";
        alertBox.style.display = "block";
        return;
      }

      if (passVal !== "PowerGIS@2026") {
        alertBox.innerText = "Incorrect password. Default demo password is 'PowerGIS@2026'.";
        alertBox.style.display = "block";
        return;
      }

      alertBox.style.display = "none";
      completeSessionLogin(match);
    };

    document.getElementById("gateGuestBtn").onclick = () => {
      const guestRole = ALL_16_ROLES.find(r => r.id === "readonly_viewer") || ALL_16_ROLES[ALL_16_ROLES.length - 1];
      currentRoleObj = guestRole;
      updateSessionUI(guestRole);
      headerLogoutBtn.style.display = "none";
      headerLoginBtn.style.display = "inline-flex";
      loginGateOverlay.classList.remove("active");
    };

    headerLogoutBtn.onclick = () => {
      localStorage.removeItem("powergis_session");
      loginGateOverlay.classList.add("active");
      headerLogoutBtn.style.display = "none";
      headerLoginBtn.style.display = "inline-flex";
    };

    // Check existing session on load
    const savedSession = localStorage.getItem("powergis_session");
    if (savedSession) {
      try {
        const parsed = JSON.parse(savedSession);
        if (parsed && parsed.user) {
          const match = ALL_16_ROLES.find(r => r.id === parsed.user.id) || parsed.user;
          currentRoleObj = match;
          updateSessionUI(match);
          loginGateOverlay.classList.remove("active");
        }
      } catch (err) {
        loginGateOverlay.classList.add("active");
      }
    } else {
      // First visit: Show Login Gate Screen!
      loginGateOverlay.classList.add("active");
    }
'''

if js_target in content:
    content = content.replace(js_target, auth_js + "\n    " + js_target, 1)
    print("Authentication JavaScript successfully injected.")
else:
    print("WARNING: JS target not found.")

with open(portal_path, "w", encoding="utf-8") as f:
    f.write(content)

print("portal/index.html updated successfully!")
