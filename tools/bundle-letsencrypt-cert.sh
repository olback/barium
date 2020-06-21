#!/bin/sh

# https://gist.github.com/novemberborn/4eb91b0d166c27c2fcd4#gistcomment-2593153

set -e

$OUTFILE=cert.p12

openssl pkcs12 -export -out $OUTFILE -inkey privkey.pem -in cert.pem -certfile chain.pem

echo "Saved PKCS 12 packed certificate to $OUTFILE"
