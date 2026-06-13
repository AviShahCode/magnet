import pytest
import uuid
import client


def test_drive_empty_root(unique_user):
    conn = unique_user["client"]
    status, response = conn.drive.get(None)

    assert status == 200
    # response should be a GetResponse::Folder variant
    assert hasattr(response, 'items')
    assert len(response.items) == 0


def test_upload_and_get_file(unique_user):
    conn = unique_user["client"]
    filename = f"test_{uuid.uuid4().hex}.txt"
    file_content = b"Hello, World!"

    # 1. Upload to root
    item = client.drive.UploadItem(filename, file_content)
    status, file_id = conn.drive.upload(None, item)
    assert status == 201
    assert file_id is not None

    # 2. Get root and verify it shows up as a file
    status, folder = conn.drive.get(None)
    assert status == 200
    assert len(folder.items) == 1
    assert folder.items[0].name == filename

    # 3. Get the actual file content by path/ID
    status, file_data = conn.drive.get(file_id)
    assert status == 200
    assert hasattr(file_data, 'content')
    assert file_data.content == file_content


def test_delete_tree(unique_user):
    conn = unique_user["client"]

    # 1. Create a "folder" (Upload item with no content usually implies folder in many drive APIs)
    folder_name = "my_folder"
    folder_item = client.drive.UploadItem(folder_name, None)
    status, folder_id = conn.drive.upload(None, folder_item)
    assert status == 201

    # 2. Upload file INSIDE the folder
    file_name = "child.txt"
    file_item = client.drive.UploadItem(file_name, b"child data")
    status, child_id = conn.drive.upload(folder_id, file_item)
    assert status == 201

    # 3. Delete the parent folder
    status = conn.drive.delete(folder_id)
    assert status == 204

    # 4. Verify parent is gone
    status, _ = conn.drive.get(folder_id)
    assert status == 404

    # 5. Verify child is also gone (cascade delete)
    status, _ = conn.drive.get(child_id)
    assert status == 404


def test_drive_user_isolation(admin_client, base_url):
    """Ensure User A cannot access User B's files."""

    # Generate unique usernames to prevent 409 Conflicts on re-runs
    username_a = f"userA_{uuid.uuid4().hex[:8]}"
    username_b = f"userB_{uuid.uuid4().hex[:8]}"

    # Setup User A
    _, code_a = admin_client.admin.signup_code()
    conn_a = client.Connector(base_url)
    assert conn_a.signup(code_a, username_a, "password123") == 201, "User A signup failed"
    assert conn_a.login(username_a, "password123") == 200, "User A login failed"

    # Setup User B
    _, code_b = admin_client.admin.signup_code()
    conn_b = client.Connector(base_url)
    assert conn_b.signup(code_b, username_b, "password123") == 201, "User B signup failed"
    assert conn_b.login(username_b, "password123") == 200, "User B login failed"

    # User A uploads a file
    item = client.drive.UploadItem("secret.txt", b"top secret a")
    status, file_id = conn_a.drive.upload(None, item)
    assert status == 201

    # User B tries to read User A's file
    status, _ = conn_b.drive.get(file_id)
    assert status in (403, 404), f"User B read User A's file! Expected 403/404, got {status}"

    # User B tries to delete User A's file
    status = conn_b.drive.delete(file_id)
    assert status in (403, 404), f"User B deleted User A's file! Expected 403/404, got {status}"
