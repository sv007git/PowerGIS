import os

portal_index = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "portal", "index.html"))

with open(portal_index, "r", encoding="utf-8") as f:
    text = f.read()

# 1. Add Scope Banner CSS
scope_css = '''
    /* Role Scope & Entitlement Banner */
    .scope-banner {
      background: var(--bg-card);
      border-bottom: 1px solid var(--border-color);
      padding: 6px 16px;
      font-size: 11px;
      display: flex;
      justify-content: space-between;
      align-items: center;
      flex-shrink: 0;
      z-index: 100;
    }
    .scope-left {
      display: flex;
      align-items: center;
      gap: 10px;
    }
    .scope-icon {
      font-size: 14px;
    }
    .scope-title {
      font-weight: 700;
      color: var(--text-main);
    }
    .scope-desc {
      color: var(--text-muted);
    }
    .scope-badges {
      display: flex;
      align-items: center;
      gap: 6px;
    }
'''

if ".scope-banner {" not in text:
    text = text.replace("  </style>", scope_css + "\n  </style>", 1)
    print("Scope banner CSS added.")

# 2. Add Scope Banner HTML right after </header>
scope_html = '''</header>

  <!-- Entitlement & Access Scope HUD -->
  <div class="scope-banner" id="scopeBanner">
    <div class="scope-left">
      <span class="scope-icon" id="scopeIcon">🛡️</span>
      <span class="scope-title" id="scopeTitle">Active Entitlement:</span>
      <span class="scope-desc" id="scopeDesc">Executive Engineer — Urban Distribution Division Operations</span>
    </div>
    <div class="scope-badges" id="scopeBadges">
      <!-- Populated dynamically -->
    </div>
  </div>'''

if "</header>" in text and "id=\"scopeBanner\"" not in text:
    text = text.replace("</header>", scope_html, 1)
    print("Scope banner HTML added.")

# 3. Replace ALL_16_ROLES definition with complete granular permissions and allowedViews
old_roles_target = "const ALL_16_ROLES = ["
# Find where ALL_16_ROLES ends
start_idx = text.find(old_roles_target)
if start_idx != -1:
    end_idx = text.find("];", start_idx) + 2
    old_roles_block = text[start_idx:end_idx]
    
    new_roles_block = '''const ALL_16_ROLES = [
      {
        id: "super_admin",
        name: "Super Administrator",
        officer: "Harish Vyas",
        designation: "Chief IT & Architecture",
        email: "superadmin@powergis.gov.in",
        perms: "Full System Superuser",
        icon: "🛡️",
        allowedViews: ["gis-view", "assets-view", "network-view", "scada-view", "tickets-view", "tasks-view", "surveys-view", "projects-view", "reports-view", "organization-view", "users-view", "settings-view"],
        can_create: true, can_edit: true, can_delete: true, can_assign: true, can_approve: true, can_manage_scada: true, can_manage_users: true, can_export: true,
        scopeDesc: "Global Root Administrator — Full Governance, Security & Schema Administration"
      },
      {
        id: "state_admin",
        name: "State Administrator",
        officer: "Dr. A. Balamurugan",
        designation: "Secretary (Power) Puducherry",
        email: "stateadmin@powergis.gov.in",
        perms: "Statewide Configuration",
        icon: "🏛️",
        allowedViews: ["gis-view", "assets-view", "network-view", "scada-view", "tickets-view", "tasks-view", "surveys-view", "projects-view", "reports-view", "organization-view", "users-view"],
        can_create: true, can_edit: true, can_delete: true, can_assign: true, can_approve: true, can_manage_scada: true, can_manage_users: true, can_export: true,
        scopeDesc: "State Directorate of Electricity — Policy, Statewide Assets & High-level Oversight"
      },
      {
        id: "circle_admin",
        name: "Circle Administrator",
        officer: "Er. G. Ravichandran",
        designation: "Superintending Engineer (Circle)",
        email: "circleadmin@powergis.gov.in",
        perms: "Circle Master Approvals",
        icon: "⚡",
        allowedViews: ["gis-view", "assets-view", "network-view", "tickets-view", "tasks-view", "surveys-view", "projects-view", "reports-view", "organization-view"],
        can_create: true, can_edit: true, can_delete: false, can_assign: true, can_approve: true, can_manage_scada: false, can_manage_users: false, can_export: true,
        scopeDesc: "Puducherry Operation Circle — Circle Administration & Inter-Divisional Approvals"
      },
      {
        id: "superintending_engineer",
        name: "Superintending Engineer (SE)",
        officer: "Er. R. Sundararajan",
        designation: "SE — Planning & Monitoring",
        email: "se.pondicherry@powergis.gov.in",
        perms: "Capital Works Authorization",
        icon: "🏢",
        allowedViews: ["gis-view", "assets-view", "network-view", "scada-view", "tickets-view", "tasks-view", "projects-view", "reports-view", "organization-view"],
        can_create: true, can_edit: true, can_delete: false, can_assign: true, can_approve: true, can_manage_scada: true, can_manage_users: false, can_export: true,
        scopeDesc: "SE Planning & Monitoring — Circle Grid SCADA, Capital Works Sanctioning & Audits"
      },
      {
        id: "executive_engineer",
        name: "Executive Engineer (EE)",
        officer: "Er. M. Sivakumar",
        designation: "EE — Urban Division",
        email: "ee.distribution@powergis.gov.in",
        perms: "Operational Dispatch & Feeder Control",
        icon: "⚡",
        allowedViews: ["gis-view", "assets-view", "network-view", "scada-view", "tickets-view", "tasks-view", "surveys-view", "projects-view", "reports-view", "organization-view"],
        can_create: true, can_edit: true, can_delete: false, can_assign: true, can_approve: true, can_manage_scada: true, can_manage_users: false, can_export: true,
        scopeDesc: "EE Urban Distribution Division — Feeder Operations, Dispatch & Outage Restoration"
      },
      {
        id: "assistant_executive_engineer",
        name: "Assistant Executive Engineer (AEE)",
        officer: "Er. N. Radhakrishnan",
        designation: "AEE — Town Sub-Division",
        email: "aee.town@powergis.gov.in",
        perms: "Sub-Division Maintenance",
        icon: "🔧",
        allowedViews: ["gis-view", "assets-view", "network-view", "tickets-view", "tasks-view", "surveys-view", "reports-view", "organization-view"],
        can_create: true, can_edit: true, can_delete: false, can_assign: true, can_approve: true, can_manage_scada: false, can_manage_users: false, can_export: true,
        scopeDesc: "AEE Marapalam Sub-Division — Substation O&M, Feeder Load Balancing & Inspections"
      },
      {
        id: "junior_engineer",
        name: "Junior Engineer (JE)",
        officer: "Er. V. Murugan",
        designation: "JE — Line Patrolling & Breakdowns",
        email: "je.field@powergis.gov.in",
        perms: "Field Inspection & Ticket Action",
        icon: "👷",
        allowedViews: ["gis-view", "assets-view", "tickets-view", "tasks-view", "surveys-view"],
        can_create: true, can_edit: true, can_delete: false, can_assign: false, can_approve: false, can_manage_scada: false, can_manage_users: false, can_export: true,
        scopeDesc: "JE Town Section — Breakdown Maintenance, Line Patrolling & Field Work Orders"
      },
      {
        id: "gis_admin",
        name: "GIS Administrator",
        officer: "P. Karthikeyan",
        designation: "Lead Geospatial Engineer",
        email: "gisadmin@powergis.gov.in",
        perms: "Spatial Layers & Topology Schema",
        icon: "🗺️",
        allowedViews: ["gis-view", "assets-view", "network-view", "surveys-view", "organization-view"],
        can_create: true, can_edit: true, can_delete: true, can_assign: false, can_approve: false, can_manage_scada: false, can_manage_users: false, can_export: true,
        scopeDesc: "GIS Administrator — Geospatial Schema, Layer Publishing & Topology Snapping"
      },
      {
        id: "gis_operator",
        name: "GIS Operator",
        officer: "S. Divya",
        designation: "Senior GIS Digitizer",
        email: "gisoperator@powergis.gov.in",
        perms: "Feature Editing & Asset Digitization",
        icon: "📍",
        allowedViews: ["gis-view", "assets-view", "network-view", "surveys-view"],
        can_create: true, can_edit: true, can_delete: false, can_assign: false, can_approve: false, can_manage_scada: false, can_manage_users: false, can_export: true,
        scopeDesc: "Senior GIS Analyst — Asset Digitization, Geocoding & Feeder Trace Mapping"
      },
      {
        id: "section_engineer",
        name: "Section Engineer",
        officer: "Er. K. Anand",
        designation: "SE — Marapalam Section",
        email: "ae.substation@powergis.gov.in",
        perms: "Section Grid Approvals",
        icon: "🔌",
        allowedViews: ["gis-view", "assets-view", "tickets-view", "tasks-view", "surveys-view", "reports-view"],
        can_create: true, can_edit: true, can_delete: false, can_assign: false, can_approve: false, can_manage_scada: false, can_manage_users: false, can_export: true,
        scopeDesc: "Section In-charge — Local DTR Inspections & Consumer Service Verification"
      },
      {
        id: "om_supervisor",
        name: "O&M Supervisor",
        officer: "T. Arumugam",
        designation: "Senior Supervisor — Grid O&M",
        email: "om.supervisor@powergis.gov.in",
        perms: "Crew Shift Supervision",
        icon: "⚙️",
        allowedViews: ["gis-view", "assets-view", "tickets-view", "tasks-view", "surveys-view"],
        can_create: true, can_edit: true, can_delete: false, can_assign: true, can_approve: false, can_manage_scada: false, can_manage_users: false, can_export: false,
        scopeDesc: "O&M Line Supervisor — Shift Crew Mobilization & Field Equipment Verification"
      },
      {
        id: "field_engineer",
        name: "Field Engineer",
        officer: "Er. S. Prakash",
        designation: "Substation Protection Specialist",
        email: "field.protection@powergis.gov.in",
        perms: "Relay & Breaker Maintenance",
        icon: "🧰",
        allowedViews: ["gis-view", "tickets-view", "tasks-view", "surveys-view"],
        can_create: true, can_edit: true, can_delete: false, can_assign: false, can_approve: false, can_manage_scada: false, can_manage_users: false, can_export: false,
        scopeDesc: "Field Patrol Engineer — Relay Calibration, Bushing Testing & Physical Inspection"
      },
      {
        id: "surveyor",
        name: "Surveyor",
        officer: "R. Manikandan",
        designation: "DGPS Asset Surveyor",
        email: "surveyor@powergis.gov.in",
        perms: "Field Geotagged Surveys",
        icon: "🛰️",
        allowedViews: ["gis-view", "surveys-view"],
        can_create: true, can_edit: false, can_delete: false, can_assign: false, can_approve: false, can_manage_scada: false, can_manage_users: false, can_export: false,
        scopeDesc: "Certified GPS Surveyor — Field Asset Geocoding, Photo Proofs & Accuracy Validation"
      },
      {
        id: "contractor",
        name: "Contractor",
        officer: "L&T Electrical Wing",
        designation: "Turnkey EPC Contractor (RDSS)",
        email: "contractor.rdss@powergis.gov.in",
        perms: "Task Updates & Photos",
        icon: "🏗️",
        allowedViews: ["tasks-view", "projects-view"],
        can_create: false, can_edit: false, can_delete: false, can_assign: false, can_approve: false, can_manage_scada: false, can_manage_users: false, can_export: false,
        scopeDesc: "EPC Contractor Mode (Larsen & Toubro) — Assigned Tasks & RDSS Project Works Only"
      },
      {
        id: "auditor",
        name: "Auditor",
        officer: "CAG / Regulatory Cell",
        designation: "Joint Regulatory Auditor",
        email: "auditor@powergis.gov.in",
        perms: "Audit Trails & SLA Reports",
        icon: "📊",
        allowedViews: ["reports-view", "tickets-view", "tasks-view", "gis-view"],
        can_create: false, can_edit: false, can_delete: false, can_assign: false, can_approve: false, can_manage_scada: false, can_manage_users: false, can_export: true,
        scopeDesc: "Regulatory Compliance Auditor — Read-Only Outage SLA, Feeder Loading & Audit Logs"
      },
      {
        id: "readonly_viewer",
        name: "Read-only Viewer",
        officer: "Citizen / Portal Guest",
        designation: "Public Power Portal Guest",
        email: "guest@powergis.gov.in",
        perms: "Public Outage Maps Only",
        icon: "👁️",
        allowedViews: ["gis-view", "tickets-view"],
        can_create: false, can_edit: false, can_delete: false, can_assign: false, can_approve: false, can_manage_scada: false, can_manage_users: false, can_export: false,
        scopeDesc: "Public Citizen Viewer — Public Network Status & General Outage Information (Read-Only)"
      }
    ];'''
    text = text[:start_idx] + new_roles_block + text[end_idx:]
    print("ALL_16_ROLES updated with permissions, icons, and allowedViews.")

# 4. Inject enforceRoleEntitlements function into script
enforce_func_js = '''
    // =========================================================================
    // ROLE-BASED ACCESS CONTROL (RBAC) & JURISDICTIONAL ENTITLEMENT ENGINE
    // =========================================================================
    function enforceRoleEntitlements(role) {
      if (!role) return;

      // 1. Update Scope Banner
      const scopeIcon = document.getElementById("scopeIcon");
      const scopeDesc = document.getElementById("scopeDesc");
      const scopeBadges = document.getElementById("scopeBadges");
      if (scopeIcon) scopeIcon.innerText = role.icon || "⚡";
      if (scopeDesc) scopeDesc.innerText = `${role.name} (${role.officer}) — ${role.scopeDesc}`;

      if (scopeBadges) {
        scopeBadges.innerHTML = "";
        const badges = [];
        if (role.can_create) badges.push({ text: "Create Permitted", cls: "badge-success" });
        else badges.push({ text: "Creation Locked", cls: "badge-warning" });

        if (role.can_manage_scada) badges.push({ text: "SCADA Control Active", cls: "badge-danger" });
        if (role.id === "contractor") badges.push({ text: "EPC Contract Scope", cls: "badge-primary" });
        if (role.id === "auditor") badges.push({ text: "Audit Mode", cls: "badge-primary" });
        if (role.id === "readonly_viewer") badges.push({ text: "Public Read-Only", cls: "badge-warning" });

        badges.forEach(b => {
          const span = document.createElement("span");
          span.className = `badge ${b.cls}`;
          span.innerText = b.text;
          scopeBadges.appendChild(span);
        });
      }

      // 2. Strict Sidebar Navigation Filtering
      const navItems = document.querySelectorAll(".nav-sidebar .nav-item[data-view]");
      navItems.forEach(item => {
        const viewId = item.getAttribute("data-view");
        if (role.allowedViews.includes(viewId)) {
          item.style.display = "flex";
        } else {
          item.style.display = "none";
        }
      });

      // Hide section titles if all section items are hidden
      document.querySelectorAll(".nav-sidebar .nav-section").forEach(sec => {
        const visibleItems = sec.querySelectorAll(".nav-item[data-view]:not([style*='display: none'])");
        sec.style.display = visibleItems.length > 0 ? "block" : "none";
      });

      // 3. Ensure Active View is Entitled
      const activeViewElem = document.querySelector(".view-container.active");
      const currentActiveId = activeViewElem ? activeViewElem.id : null;
      if (!currentActiveId || !role.allowedViews.includes(currentActiveId)) {
        const fallbackView = role.allowedViews[0];
        const targetNav = document.querySelector(`.nav-sidebar .nav-item[data-view='${fallbackView}']`);
        if (targetNav) {
          targetNav.click();
        } else {
          document.querySelectorAll(".view-container").forEach(v => v.classList.remove("active"));
          const fallbackElem = document.getElementById(fallbackView);
          if (fallbackElem) fallbackElem.classList.add("active");
        }
      }

      // 4. Action Buttons Entitlement (Creation, SCADA, Export, Administration)
      const newTicketHeaderBtn = document.getElementById("openNewTicketModalBtn");
      const createTicketBtn2 = document.getElementById("createTicketBtn2");
      const addAssetBtn = document.getElementById("openAddAssetModalBtn");
      const newTaskBtn = document.getElementById("openNewTaskModalBtn");
      const submitSurveyBtn = document.getElementById("submitSurveyBtn");

      if (newTicketHeaderBtn) newTicketHeaderBtn.style.display = role.can_create ? "inline-flex" : "none";
      if (createTicketBtn2) createTicketBtn2.style.display = role.can_create ? "inline-flex" : "none";
      if (addAssetBtn) addAssetBtn.style.display = (role.can_create && role.can_manage_gis_layers !== false) ? "inline-flex" : "none";
      if (newTaskBtn) newTaskBtn.style.display = role.can_create ? "inline-flex" : "none";

      if (submitSurveyBtn) {
        if (role.id === "surveyor" || role.id === "junior_engineer" || role.id === "field_engineer" || role.can_create) {
          submitSurveyBtn.disabled = false;
          submitSurveyBtn.innerText = "💾 Submit Inspection Record";
        } else {
          submitSurveyBtn.disabled = true;
          submitSurveyBtn.innerText = "🔒 Survey Submission Restricted";
        }
      }

      // SCADA Controls Entitlement
      const simulateTripBtn = document.getElementById("simulateTripBtn");
      const autoTicketFromAlarmBtn = document.getElementById("autoTicketFromAlarmBtn");
      const ackAllAlarmsBtn = document.getElementById("ackAllAlarmsBtn");
      if (simulateTripBtn) simulateTripBtn.style.display = role.can_manage_scada ? "inline-flex" : "none";
      if (autoTicketFromAlarmBtn) autoTicketFromAlarmBtn.style.display = role.can_manage_scada ? "inline-flex" : "none";
      if (ackAllAlarmsBtn) ackAllAlarmsBtn.style.display = role.can_manage_scada ? "inline-flex" : "none";

      // Export Buttons
      const exportCsvBtn = document.getElementById("exportCsvBtn");
      const exportGeoJsonBtn = document.getElementById("exportGeoJsonBtn");
      if (exportCsvBtn) exportCsvBtn.style.display = role.can_export ? "inline-flex" : "none";
      if (exportGeoJsonBtn) exportGeoJsonBtn.style.display = role.can_export ? "inline-flex" : "none";

      // 5. Data Filtering by Role Entitlement
      renderTicketsTable();
      renderKanban();
    }
'''

if "function enforceRoleEntitlements(role)" not in text:
    target_render16 = "function render16RolesDirectory() {"
    text = text.replace(target_render16, enforce_func_js + "\n    " + target_render16, 1)
    print("enforceRoleEntitlements function injected.")

# 5. Update renderTicketsTable and renderKanban to respect entitlements
ticket_table_old = '''    function renderTicketsTable() {
      const tbody = document.getElementById("ticketsTableBody");
      tbody.innerHTML = "";
      APP_DATA.tickets.forEach(t => {
        const tr = document.createElement("tr");
        tr.innerHTML = `
          <td><b>${t.number}</b></td>
          <td>${t.category}</td>
          <td>${t.subject}</td>
          <td><span class="badge badge-danger">${t.priority.toUpperCase()}</span></td>
          <td><span class="badge badge-primary">${t.status.toUpperCase()}</span></td>
          <td>${t.assignedTo}</td>
          <td>⏱️ ${t.slaHours}h Target</td>
          <td>
            <button class="btn btn-secondary btn-sm" onclick="handleAdvanceTicket('${t.id}')">Advance Stage ➔</button>
          </td>
        `;
        tbody.appendChild(tr);
      });
    }'''

ticket_table_new = '''    function renderTicketsTable() {
      const tbody = document.getElementById("ticketsTableBody");
      if (!tbody) return;
      tbody.innerHTML = "";

      let visibleTickets = APP_DATA.tickets;
      // If Public Read-only viewer, only show public outages (critical priority)
      if (currentRoleObj.id === "readonly_viewer") {
        visibleTickets = APP_DATA.tickets.filter(t => t.priority === "critical");
      }

      visibleTickets.forEach(t => {
        const tr = document.createElement("tr");
        const actionBtn = currentRoleObj.can_assign || currentRoleObj.can_approve
          ? `<button class="btn btn-secondary btn-sm" onclick="handleAdvanceTicket('${t.id}')">Advance Stage ➔</button>`
          : `<span class="badge badge-primary" style="font-size: 10px;">${t.status.toUpperCase()}</span>`;

        tr.innerHTML = `
          <td><b>${t.number}</b></td>
          <td>${t.category}</td>
          <td>${t.subject}</td>
          <td><span class="badge badge-danger">${t.priority.toUpperCase()}</span></td>
          <td><span class="badge badge-primary">${t.status.toUpperCase()}</span></td>
          <td>${t.assignedTo}</td>
          <td>⏱️ ${t.slaHours}h Target</td>
          <td>${actionBtn}</td>
        `;
        tbody.appendChild(tr);
      });
    }'''

if ticket_table_old in text:
    text = text.replace(ticket_table_old, ticket_table_new, 1)
    print("renderTicketsTable updated with entitlement filters.")

# 6. Update renderKanban to filter for contractor and restrict advance buttons
kanban_old = '''    function renderKanban() {
      const cols = ["created", "assigned", "in_progress", "completed", "verified"];
      cols.forEach(c => {
        const container = document.getElementById(`cards-${c}`);
        container.innerHTML = "";
        const tasks = APP_DATA.tasks.filter(t => t.status === c);
        document.getElementById(`count-${c}`).innerText = tasks.length;

        tasks.forEach(task => {
          const card = document.createElement("div");
          card.className = "kanban-card";
          card.innerHTML = `
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;">
              <span class="badge badge-primary">${task.code}</span>
              <span class="badge badge-warning">${task.priority.toUpperCase()}</span>
            </div>
            <div class="kanban-card-title">${task.title}</div>
            <div class="kanban-card-meta">
              <span>👷 ${task.team}</span>
              <button class="btn btn-secondary btn-sm" onclick="advanceTaskStatus('${task.id}')" title="Advance to next column">→</button>
            </div>
          `;
          container.appendChild(card);
        });
      });
    }'''

kanban_new = '''    function renderKanban() {
      const cols = ["created", "assigned", "in_progress", "completed", "verified"];
      
      // If Contractor, filter tasks to their assigned work only
      let taskPool = APP_DATA.tasks;
      if (currentRoleObj.id === "contractor") {
        taskPool = APP_DATA.tasks.filter(t => 
          t.team.toLowerCase().includes("crew a") || 
          t.title.toLowerCase().includes("underground") || 
          t.title.toLowerCase().includes("cable") || 
          t.priority === "high"
        );
      }

      cols.forEach(c => {
        const container = document.getElementById(`cards-${c}`);
        if (!container) return;
        container.innerHTML = "";
        const tasks = taskPool.filter(t => t.status === c);
        const countElem = document.getElementById(`count-${c}`);
        if (countElem) countElem.innerText = tasks.length;

        tasks.forEach(task => {
          const card = document.createElement("div");
          card.className = "kanban-card";
          
          const advanceBtn = (currentRoleObj.can_assign || currentRoleObj.can_create || currentRoleObj.id === "contractor")
            ? `<button class="btn btn-secondary btn-sm" onclick="advanceTaskStatus('${task.id}')" title="Advance to next column">→</button>`
            : `<span style="font-size: 10px; color: var(--text-dim);">Read Only</span>`;

          card.innerHTML = `
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;">
              <span class="badge badge-primary">${task.code}</span>
              <span class="badge badge-warning">${task.priority.toUpperCase()}</span>
            </div>
            <div class="kanban-card-title">${task.title}</div>
            <div class="kanban-card-meta">
              <span>👷 ${task.team}</span>
              ${advanceBtn}
            </div>
          `;
          container.appendChild(card);
        });
      });
    }'''

if kanban_old in text:
    text = text.replace(kanban_old, kanban_new, 1)
    print("renderKanban updated with contractor filtering and permission checks.")

# 7. Call enforceRoleEntitlements inside switchRole and updateSessionUI
old_switch_role = '''    window.switchRole = function(roleId) {
      const match = ALL_16_ROLES.find(r => r.id === roleId);
      if (match) {
        currentRoleObj = match;
        document.getElementById("headerUserName").innerText = match.officer;
        document.getElementById("headerRoleName").innerText = match.name;
        document.getElementById("headerAvatar").innerText = match.name.split(" ").map(w => w[0]).slice(0, 2).join("");
        render16RolesDirectory();
      }
    };'''

new_switch_role = '''    window.switchRole = function(roleId) {
      const match = ALL_16_ROLES.find(r => r.id === roleId);
      if (match) {
        currentRoleObj = match;
        document.getElementById("headerUserName").innerText = match.officer;
        document.getElementById("headerRoleName").innerText = match.name;
        document.getElementById("headerAvatar").innerText = match.name.split(" ").map(w => w[0]).slice(0, 2).join("");
        render16RolesDirectory();
        enforceRoleEntitlements(match);
      }
    };'''

if old_switch_role in text:
    text = text.replace(old_switch_role, new_switch_role, 1)
    print("switchRole updated to enforce entitlements.")

old_update_session = '''    function updateSessionUI(roleObj) {
      document.getElementById("headerUserName").innerText = roleObj.officer;
      document.getElementById("headerRoleName").innerText = roleObj.name;
      document.getElementById("headerAvatar").innerText = roleObj.name.split(" ").map(w => w[0]).slice(0, 2).join("");
      headerLogoutBtn.style.display = "inline-flex";
      headerLoginBtn.style.display = "none";
      render16RolesDirectory();
    }'''

new_update_session = '''    function updateSessionUI(roleObj) {
      document.getElementById("headerUserName").innerText = roleObj.officer;
      document.getElementById("headerRoleName").innerText = roleObj.name;
      document.getElementById("headerAvatar").innerText = roleObj.name.split(" ").map(w => w[0]).slice(0, 2).join("");
      headerLogoutBtn.style.display = "inline-flex";
      headerLoginBtn.style.display = "none";
      render16RolesDirectory();
      enforceRoleEntitlements(roleObj);
    }'''

if old_update_session in text:
    text = text.replace(old_update_session, new_update_session, 1)
    print("updateSessionUI updated to enforce entitlements.")

# Also add initial call at page startup
init_target = 'enforceRoleEntitlements(currentRoleObj);'
if init_target not in text:
    text = text.replace('console.log("⚡ PowerGIS Enterprise GIS Portal Initialized', 'enforceRoleEntitlements(currentRoleObj);\n    console.log("⚡ PowerGIS Enterprise GIS Portal Initialized', 1)
    print("Initial call to enforceRoleEntitlements added.")

with open(portal_index, "w", encoding="utf-8") as f:
    f.write(text)

print("portal/index.html updated successfully with complete RBAC entitlements!")
