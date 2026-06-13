import pytest
import uuid
import client # Your compiled PyO3 module

BASE_URL = "http://localhost:9080" # Change to your server's port

# --- Configuration & Seed Data ---
ADMIN_USERNAME = "admin"
ADMIN_PASSWORD = "1pass2word3" # Replace with your seeded admin password

@pytest.fixture(scope="session")
def base_url():
    return BASE_URL

@pytest.fixture
def public_client(base_url):
    """An unauthenticated client."""
    return client.Connector(base_url)

@pytest.fixture
def admin_client(base_url):
    """A client logged in as the admin."""
    conn = client.Connector(base_url)
    status = conn.login(ADMIN_USERNAME, ADMIN_PASSWORD)
    assert status == 200, "Failed to login as admin. Check seeded credentials."
    return conn

@pytest.fixture
def unique_user(admin_client, base_url):
    """Creates a fresh, unique user for testing."""
    # 1. Admin generates a signup code
    status, code = admin_client.admin.signup_code()
    assert status in (200, 201)

    # 2. Generate unique valid credentials
    uid = uuid.uuid4().hex[:8]
    username = f"user_{uid}"
    password = f"Pass_{uid}_123!"

    # 3. Create the user
    conn = client.Connector(base_url)
    signup_status = conn.signup(code, username, password)
    assert signup_status == 201

    # 4. Login the new user
    login_status = conn.login(username, password)
    assert login_status == 200

    return {
        "client": conn,
        "username": username,
        "password": password
    }