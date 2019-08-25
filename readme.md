# Substr8

A security-focused microservice to avoid caching external secrets.

## Why

It's common to inject secrets into an application's containers before deploying it.  This is convenient and well supported,
but unless you take care to remove those secrets, they continue to exist in memory, probably in environment variables. 
Should a container become comprimised, the keys to the kingdom are easily obtainable, allowing a single container to become a
pivot point.  This means that every container in a cluster becomes a high value asset.

Substr8 intends to be the sole point of access for AWS SecretManager and other assets.  This allows consistent and secure handling
of application secrets.

## TODO

Substr8 is currently very young and in an initial hardening phase.  Most work is centered around laying security groundwork down to
add additional cloud resource functionality.

- JWT for per-deployment auth
- In-memory secret encryption
- Migration to Alpine
- Azure support

