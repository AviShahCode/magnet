# Magnet Python SDK

This is a high-performance, Rust-backed Python client for the Magnet backend. It is built using `PyO3` and `reqwest`, providing the raw speed of Rust with the ease of use of Python.

## 🛠 Installation & Setup

Because this client is written in Rust, it must be compiled into a Python-readable binary before use. We use `uv` for environment management and `maturin` for building.

**1. Set up the environment**
```bash
uv venv
source .venv/bin/activate  # Or .venv\Scripts\activate on Windows
uv pip install maturin
```

**2. Build and Install**
Whenever you modify the Rust code (`src/lib.rs`), you must recompile the client:

```bash
maturin develop
```

*This command compiles the Rust code and installs it directly into your active virtual environment as the `client` module.*

---

## 🚀 Quick Start

### 1. Connecting to the Server

Initialize the `Connector` with your backend's base URL. The client automatically handles session cookies under the hood.

```python
import client

base_url = "http://localhost:8000"
conn = client.Connector(base_url)

# Check if the server is alive
assert conn.ping() == 200
```

### 2. Authentication Flow

You need an admin-generated signup code to create a new user.

```python
# 1. Admin generates a signup code (Valid for 48 hours)
admin_conn = client.Connector(base_url)
admin_conn.login("admin", "admin_password")
status, signup_code = admin_conn.admin.signup_code()

# 2. User signs up with the code
# Usernames: 8-32 chars, alphanumeric & underscores only.
# Passwords: 6-64 chars.
conn.signup(signup_code, "new_user_123", "my_secure_password")

# 3. User logs in
conn.login("new_user_123", "my_secure_password")

# 4. User logs out (invalidates session cookie)
conn.logout()
```

---

## 📂 Drive Operations (Files & Folders)

The `drive` module allows you to upload, download, list, and delete files.

> **Important Note on Imports:** Because of how PyO3 compiles submodules, you must import the main `client` module and access classes via dot-notation (e.g., `client.drive.UploadItem`), rather than using `from client.drive import ...`.

### Uploading a File (to the Root directory)

To upload a file, create an `UploadItem` with the filename and the file's raw bytes. Pass `None` as the path to upload to the root directory.

```python
import client

# Read a local file
with open("my_photo.jpg", "rb") as f:
    file_bytes = f.read()

# Create the upload payload
item = client.drive.UploadItem("my_photo.jpg", file_bytes)

# Upload to root (path = None)
status, file_id = conn.drive.upload(None, item)
print(f"Uploaded successfully! File ID: {file_id}")
```

### Creating a Folder

To create a folder, you upload an item **without any content** (`None`).

```python
# Create an empty item to represent a folder
folder_item = client.drive.UploadItem("My Documents", None)

status, folder_id = conn.drive.upload(None, folder_item)
print(f"Folder created with ID: {folder_id}")
```

### Uploading a File into a Specific Folder

If you want to upload a file *inside* a folder, pass the parent folder's ID as the `path` argument.

```python
item = client.drive.UploadItem("secret_doc.txt", b"Top secret data")

# Pass the folder_id as the path
status, file_id = conn.drive.upload(folder_id, item)
```

### Reading Files and Listing Folders

The `get()` method behaves differently depending on whether you request a File ID or a Folder ID (or `None` for root).

**Listing a Folder:**

```python
# Get contents of the root folder
status, response = conn.drive.get(None)

if hasattr(response, 'items'):
    print("Folder Contents:")
    for item in response.items:
        print(f"- {item.name} (Type: {item.item_type}, ID: {item.id})")
```

**Downloading a File:**

```python
# Pass a specific File ID to download it
status, response = conn.drive.get("some-file-id-here")

if hasattr(response, 'content'):
    print(f"Downloaded {response.name}")
    # response.content contains the raw bytes
    with open("downloaded_file.txt", "wb") as f:
        f.write(response.content)
```

### Deleting Files and Folders

Deleting a folder will automatically cascade and delete all children (files and subfolders) inside it.

```python
# Delete a specific file or folder by its ID
status = conn.drive.delete("some-id-here")

if status == 200:
    print("Item deleted successfully.")
```

---

## 🔒 Admin Operations

The `/admin` endpoints are strictly protected by Role-Based Access Control (RBAC). Standard users will receive a `403 Forbidden` if they attempt to use these commands.

```python
admin_conn = client.Connector(base_url)
admin_conn.login("admin_user", "admin_pass")

# Verify admin access
assert admin_conn.admin.ping() == 200

# Generate a 2-day signup code for new users
status, code = admin_conn.admin.signup_code()
print(f"Give this code to the new user: {code}")
```