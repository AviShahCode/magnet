import urllib.request
import time

# Target the exact URL and session state we discussed earlier
URL = "http://localhost:9080/drive/38ce45c440161876cabc218a37a385b9"
SESSION_COOKIE = "session_id=ccd5fa9f635d21701c5c112c7db92540"


def fetch_data(use_gzip: bool):
    req = urllib.request.Request(URL)
    req.add_header("Cookie", SESSION_COOKIE)

    if use_gzip:
        req.add_header("Accept-Encoding", "gzip")
        print("\n--- [ TEST 2: GZIP ENABLED ] ---")
    else:
        print("\n--- [ TEST 1: NO COMPRESSION ] ---")

    start_time = time.perf_counter()

    try:
        with urllib.request.urlopen(req) as response:
            # Read the raw bytes straight off the wire
            wire_bytes = response.read()
            duration_ms = (time.perf_counter() - start_time) * 1000

            # Extract the headers we care about
            content_encoding = response.getheader("Content-Encoding", "None")
            transfer_encoding = response.getheader("Transfer-Encoding", "None")
            content_length = response.getheader("Content-Length", "None")

            size_kb = len(wire_bytes) / 1024

            print(f"Status            : {response.status}")
            print(f"Content-Encoding  : {content_encoding}")
            print(f"Transfer-Encoding : {transfer_encoding}")
            print(f"Content-Length    : {content_length}")
            print(f"Raw Wire Size     : {size_kb:.2f} KB")
            print(f"Round Trip Time   : {duration_ms:.2f} ms")

            return len(wire_bytes)

    except urllib.error.URLError as e:
        print(f"HTTP Request Failed: {e}")
        return 0


if __name__ == "__main__":
    print("Initiating Gzip Verification Test...\n")

    uncompressed_size = fetch_data(use_gzip=False)
    time.sleep(2)
    compressed_size = fetch_data(use_gzip=True)

    if uncompressed_size > 0 and compressed_size > 0:
        savings = (1 - (compressed_size / uncompressed_size)) * 100
        print("\n=== [ FINAL RESULTS ] ===")
        print(f"Data Reduced By : {savings:.2f}%")