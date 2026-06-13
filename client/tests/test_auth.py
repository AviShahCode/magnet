import pytest
import uuid
import client


def test_public_ping(public_client):
    status = public_client.ping()
    assert status == 200


def test_login_fail(public_client):
    status = public_client.login("nonexistent_user", "badpassword")
    assert status in (401, 404)


def test_logout(unique_user):
    conn = unique_user["client"]
    status, _ = conn.drive.get(None)
    assert status == 200

    logout_status = conn.logout()
    assert logout_status == 204

    status_after, _ = conn.drive.get(None)
    assert status_after == 401


# --- Signup Constraint Tests ---

def test_signup_success_and_409_conflict(admin_client, base_url):
    """Test successful signup, and verify duplicate username returns 409."""
    _, code = admin_client.admin.signup_code()
    conn = client.Connector(base_url)

    # Generate unique 16-character username (well within the 8..=32 limit)
    username = f"user_{uuid.uuid4().hex[:10]}"
    password = "valid_password_123"

    # 1. First signup should be 201 Created
    status = conn.signup(code, username, password)
    assert status == 201

    # 2. Second signup with the EXACT SAME username should be 409 Conflict
    _, code2 = admin_client.admin.signup_code()  # Fresh code just in case
    conflict_status = conn.signup(code2, username, "some_other_password")
    assert conflict_status == 409


def test_signup_username_too_short(admin_client, base_url):
    """Username must be >= 8 chars."""
    _, code = admin_client.admin.signup_code()
    # 7 characters
    assert client.Connector(base_url).signup(code, "short_7", "valid_pass") == 400


def test_signup_username_too_long(admin_client, base_url):
    """Username must be <= 32 chars."""
    _, code = admin_client.admin.signup_code()
    # 33 characters
    assert client.Connector(base_url).signup(code, "u" * 33, "valid_pass") == 400


def test_signup_username_invalid_regex(admin_client, base_url):
    """Username must match ^[a-zA-Z0-9_]+$"""
    _, code = admin_client.admin.signup_code()
    # Hyphens and special chars are invalid
    assert client.Connector(base_url).signup(code, "invalid-char!", "valid_pass") == 400
    assert client.Connector(base_url).signup(code, "no spaces", "valid_pass") == 400


def test_signup_password_too_short(admin_client, base_url):
    """Password must be >= 6 chars."""
    _, code = admin_client.admin.signup_code()
    username = f"user_{uuid.uuid4().hex[:8]}"
    # 5 characters
    assert client.Connector(base_url).signup(code, username, "short") == 400


def test_signup_password_too_long(admin_client, base_url):
    """Password must be <= 64 chars."""
    _, code = admin_client.admin.signup_code()
    username = f"user_{uuid.uuid4().hex[:8]}"
    # 65 characters
    assert client.Connector(base_url).signup(code, username, "p" * 65) == 400