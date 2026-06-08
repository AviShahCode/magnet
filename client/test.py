import client, time
from multiprocessing import Pool
# print(dir(client.drive.GetResponse))

def benchmark(_):
    conn = client.Connector("http://localhost:9080")
    conn.login("admin", "1234")

    for x in range(100):
        # conn.login("avishah", "1234")
        # conn.logout()
        status, items = conn.drive.get("38ce45c440161876cabc218a37a385b9")
        # print(status, items.content)

    conn.logout()

start = time.perf_counter()
with Pool(20) as pool:
    pool.map(benchmark, [None] * 100)
print(time.perf_counter() - start)

exit()

def print_drive_tree(conn, folder_id=None, indent_level=0):
    # Fetch the contents of the current folder (None = root)
    status, res = conn.drive.get(folder_id)

    # Create the indentation string (4 spaces per level)
    indent = "    " * indent_level

    if status != 200:
        print(f"{indent}⚠️ Error {status} reading folder {folder_id}")
        return

    for item in res.items:
        print(f"{indent}{item}")
        if item.item_type == client.drive.ItemType.Folder:
            print_drive_tree(conn, item.id, indent_level + 1)


# --- How to use it ---

conn = client.Connector("http://localhost:9080")
print("login:", conn.login("admin", "1234"))

print("\nmy drive:")
print_drive_tree(conn)

s, res = conn.drive.get("63e99a4a4afb4e953d23e718e48a28e2")
print("\nget file")
print(s)
print(isinstance(res, client.drive.GetResponse.File), res.name, "=", res.content)

print("\nuploading...")
res1 = conn.drive.upload(None, client.drive.UploadItem("upload folder", None))
res2 = conn.drive.upload(res1[1], client.drive.UploadItem("upload.txt", b"\x01hello"))
print("uploaded", res1[1], res2[1])
root_files = conn.drive.get(None)
print("listing:", root_files[0], root_files[1].items)
nest_files = conn.drive.get(res1[1])
print("listing:", nest_files[0], nest_files[1].items)

time.sleep(5)

print("\ndeleting...")
# conn.drive.delete(res2[1])
conn.drive.delete(res1[1])
print("now")
root_files = conn.drive.get(None)
print("listing:", root_files[0], root_files[1].items)
nest_files = conn.drive.get(res1[1])
print("listing:", nest_files[0], nest_files[1].items)

# delete file of other user
print("\ndelete other user file", conn.drive.delete("7ded1b19ed96c533efec9affdcccff18"))

conn.logout()

