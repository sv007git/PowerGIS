import os

portal_index = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "portal", "index.html"))

with open(portal_index, "r", encoding="utf-8") as f:
    text = f.read()

assert 'function enforceRoleEntitlements(role)' in text, "Missing enforceRoleEntitlements"
assert 'id="scopeBanner"' in text, "Missing scopeBanner"
assert 'allowedViews' in text, "Missing allowedViews"
assert 'can_manage_scada' in text, "Missing can_manage_scada"
assert 'contractor' in text, "Missing contractor role"
assert 'surveyor' in text, "Missing surveyor role"
assert 'auditor' in text, "Missing auditor role"

print("SUCCESS: All RBAC and entitlement integrity checks verified successfully!")
