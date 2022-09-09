# merflow

Build notes:
```
glibc build - passing
musl libc build - failing (via deps compiling of postgres v0.19.4 - failed to select a version for the requirement `phf = "^0.11"`
  candidate versions found which didn't match: 0.10.1, 0.10.0, 0.9.0, ...)


Held back dep to postgres v0.19.3 tunil v0.19.4 has musl libc deps building.
```


A template microservice template for redis hydration from postgres.

The postgres connection string with the credentials is set in an environment variable named pcred.

The format for the pgred environment variable is `postgresql://USER:PASS@SERVER:PORT`, where postgresql is the protocol spec that remains the same, and the all caps values are set as desired.

The included Dockerfile uses the `FROM ekidd/rust-musl-builder` AS build to compile with cargo and then we copy the dependencies into a `FROM scratch` empty container. The resulting OCI image has no shell, nothing but the dependencies for the web server, which is helpful for further protecting secrets in the environment variables.

The design is to have merflow running with redis on the loopback (same Pod or VM) and pull from postgres, inserting data from postgres into redis periodically. Adjust the usage from the template in the `main.rs` file to align to the scheme and needs of the data.

The default structure is to SELECT from a users table, inserting key value pairs based on the data.

id: email
username: password

We assume that the password is a hash or encoded ciphertext :)

