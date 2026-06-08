# Client connector wrapper for python

Make a connector:

```python
import client

conn = client.Connector("http://localhost:7742")
```

### Auth commands

1. To login/logout:

```python
conn.login("username", "password")
conn.logout()
```

### Drive commands

1. To get a drive connector:

```python
drive_conn = conn.drive
```

2. To get file/folder

```python
status, res = drive_conn.get(path)
```

If `path` is `None`, it defaults to user's home directory. If path points to a folder, it returns `client.drive.GetResponse.Folder`, and its items can be accessed with `res.items`. If path points to a file, it returns `client.drive.GetResponse.File`, and its name and byte content can be accessed with `res.name`, `res.content`.

3. To upload file/folder

```python
status, res = drive_conn.upload(path, client.drive.UploadItem(name, content))
```

If content is `None`, it creates a folder at the path, otherwise creates a file with specified byte content. The resulting path of the new file is returned as `res`.

4. To delete file/folder

```python
status = drive_conn.delete(path)
```

Note: path cannot be `None` (cannot delete home directory). This is recursive.