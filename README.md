# merflow

Build notes:
```
glibc build - passing
musl libc build - failing (via deps compiling of postgres v0.19.4 - failed to select a version for the requirement `phf = "^0.11"`
  candidate versions found which didn't match: 0.10.1, 0.10.0, 0.9.0, ...)


Cargo.toml has held back dep of postgres v0.19.3 unil v0.19.4 has musl libc deps building.
```


A template microservice template for redis hydration from postgres.

The postgres connection string with the credentials is set in an environment variable named pcred.

The format for the pgred environment variable is `postgresql://USER:PASS@SERVER:PORT`, where postgresql is the protocol spec that remains the same, and the all caps values are set as desired.

The included Dockerfile uses the `FROM ekidd/rust-musl-builder` AS build to compile with cargo and then we copy the dependencies into a `FROM scratch` empty container. The resulting OCI image has no shell, nothing but the dependencies for the web server, which is helpful for further protecting secrets in the environment variables.

The design is to have merflow running with redis on the loopback (same Pod or VM) and pull from postgres, inserting data from postgres into redis periodically. Adjust the usage from the template in the `main.rs` file to align to the scheme and needs of the data.

The default structure is to SELECT from a users table, inserting key value pairs based on the data.

```
id: email
username: password
```

We assume that the password is a hash or encoded ciphertext :)

The docker container will exit with a failure if run via Docker because it will fail to connect to redis on the loopback.
The container is designed for Kubernetes to be included in the same Pod as redis, like a sidecar, so that merflow can call localhost:6379
and reach redis. The environment variable pcred is also required to be set.


#### Adding TLS

Note that the postgres connection does not use TLS by default. While this might be okay when a cluster has automatic network encryption and the postgres is inside the cluster, in many production cases, TLS would be desired on the postgres connection, especially if the connection between redis and postgres requires traversal over insecure or untrusted networks, like across the internet or plaintext/insecure LAN or WAN.

See `postgres-native-tls = "0.5.0"` and https://docs.rs/postgres-native-tls/latest/postgres_native_tls/ for adding TLS to merflow.
And for the postgres side, see https://www.postgresql.org/docs/9.1/ssl-tcp.html, https://docs.bitnami.com/kubernetes/infrastructure/postgresql-ha/administration/enable-tls/ and https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/PostgreSQL.Concepts.General.SSL.html, depending on which postgres deployment type is used.

The redis connection, while only on the loopback, also has no TLS and no password by default. If using TLS, after redis is set to use TLS, then change the main.rs to use rediss instead of redis. There may be certificates that need to be loaded by merflow in this case.
