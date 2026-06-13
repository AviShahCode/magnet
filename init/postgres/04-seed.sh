#!/bin/bash
set -e

# Read from environment variables, exit if not provided.
# We map them to internal variables to avoid clashing with system vars.
USERNAME="${MAGNET_ADMIN_USERNAME}"
PASSWORD="${MAGNET_ADMIN_PASSWORD}"

# Generate an 8-character (6-byte) random base64 salt
SALT=$(head -c 6 /dev/urandom | base64)

# Hash password + salt using SHA-256
# Note: 'echo -n' is critical here so we don't accidentally hash a newline character
HASH=$(echo -n "${PASSWORD}${SALT}" | sha256sum | awk '{print $1}')

echo "Generating credentials for: ${USERNAME}"
echo "Executing SQL insertion..."

psql -d magnet <<EOF
INSERT INTO users VALUES (1, '${USERNAME}', '${HASH}', '${SALT}');
INSERT INTO user_roles VALUES (1, 1);
EOF

echo "Done."