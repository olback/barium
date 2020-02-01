# Generate a Self-Signed certificate

Generate private key and public certificate. Enter your domain as Common Name (CN).
```
openssl req -newkey rsa:4096 -nodes -keyout key.pem -x509 -days 365 -out certificate.pem
```

Review the certificate. (optional)
```
openssl x509 -text -noout -in certificate.pem
```

Bundle your certificate in a PKCS#12 (P12) bundle. (also known as .pfx)
```
openssl pkcs12 -inkey key.pem -in certificate.pem -export -out certificate.p12
```

Validate your P12 file. (optional)
```
openssl pkcs12 -in certificate.p12 -noout -info
```

https://www.ibm.com/support/knowledgecenter/SSMNED_2018/com.ibm.apic.cmc.doc/task_apionprem_gernerate_self_signed_openSSL.html
