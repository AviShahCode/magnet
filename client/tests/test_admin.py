import pytest

def test_admin_ping_success(admin_client):
    """Admin should get 200 OK."""
    status = admin_client.admin.ping()
    assert status == 200

def test_admin_ping_forbidden_for_user(unique_user):
    """Standard user should get 403 Forbidden."""
    conn = unique_user["client"]
    status = conn.admin.ping()
    assert status == 403

def test_admin_signup_code_success(admin_client):
    """Admin can generate codes."""
    status, code = admin_client.admin.signup_code()
    assert status in (200, 201)
    assert isinstance(code, str)
    assert len(code) > 0

def test_admin_signup_code_forbidden_for_user(unique_user):
    """Standard user cannot generate signup codes."""
    conn = unique_user["client"]
    status, code = conn.admin.signup_code()
    assert status == 403