#!/bin/sh

# Generate cert and keys
openssl req -newkey rsa:4096 -nodes -keyout key.pem -x509 -days 365 -out cert.pem -subj '/CN=localhost'

# Pack pfx/p12
openssl pkcs12 -inkey key.pem -in cert.pem -export -out cert.p12
