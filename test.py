import requests

url = "http://localhost:7742/admin"

headers = {
    "session_token": "a5e3ec9aa60acf55ee91ad3c28a75f28"
}

data = {
    "filename": "notes.txt",
    "content": "This is my uploaded file."
}

response = requests.post(url, headers=headers, verify=True)  # verify=False if using mkcert/self-signed

print("Status:", response.status_code)
print("Response:", response.text)
