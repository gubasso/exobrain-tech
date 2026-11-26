# Caddy

Caddy is a web server and reverse proxy with automatic HTTPS. This page records how Caddy serves as a loopback HTTP credential gateway, and which client settings of gh, glab, and git route through it. [Credential gateways](../../programming/agent-integration/credential-gateways.md) states the rule, the shape of a gateway, and its allow list. This page states only the Caddy and client facts that bind that shape.

## Versions and date

| Program | Version |
| ------- | ------- |
| Caddy   | 2.11.9  |
| gh      | 2.105.0 |
| glab    | 1.130.0 |
| git     | 2.56.0  |

The probes ran on 2026-11-25 on Linux. The documentation pages under the references were read on the same date.

## Why a mask cannot carry a git push

A git push over HTTPS with an `Authorization: Bearer` header got 401 from GitHub and from GitLab. Both ways of sending the header failed: `http.extraHeader`, and a credential helper that answers `authtype=Bearer`. A git push to either forge then needs Basic auth, and a mask cannot swap a sentinel inside base64. So the gateway carries a git push, and a mask does not.

## The gateway shape

One site per upstream, each on its own loopback port. The global block turns off the admin API and binds every site to loopback.

```caddyfile
{
	admin off
	default_bind 127.0.0.1
}

http://127.0.0.1:<port> {
	@allowed {
		method GET
		path /api/v4/projects/<group>%2F<project> /api/v4/projects/<group>%2F<project>/*
	}
	handle @allowed {
		reverse_proxy https://gitlab.com {
			header_up Host {upstream_hostport}
			header_up -Private-Token
			header_up Authorization "Bearer <credential>"
		}
	}
	handle {
		respond "<route>: refused by the gateway, not by the forge. Routes and reasons: <document>" 403
	}
}
```

The `<credential>` comes from a source that only the host reads. The site block holds no `log` directive, so Caddy writes no access log.

### What each line does

- `header_up Authorization <value>` replaces every `Authorization` value that the client sent. The upstream sees only the value of the gateway.
- `header_up -Private-Token` deletes the header that glab sends. Without it, the dummy client token reaches GitLab beside the real one.
- The `path` matcher holds the allow list, and the `method` matcher beside it narrows each entry. The `path` matcher matched the encoded form `gitlab-org%2Fcli` of a GitLab project path.
- A request that no matcher allows gets the 403 of the last `handle`, and it never reaches the upstream.

### Why the admin API and the access log stay off

The admin API listens on `localhost:2019` by default. `GET /config/` prints the whole configuration, the credential included, to every local process. `admin off` turns the endpoint off. A configuration change then needs a restart, because `caddy reload` uses the admin API.

The access log redacts `Cookie`, `Set-Cookie`, `Authorization`, and `Proxy-Authorization`. It writes every other header in clear text. A probe on 2026-11-25 showed the `Private-Token` value of a glab request in the access log. A gateway with no `log` directive writes no access log, so no credential header reaches a file.

### Why `default_bind` stays on

A site address `http://127.0.0.1:<port>` alone does not bind Caddy to loopback. The probe showed the listener `listen tcp :<port>`, which answers on every interface. The `bind` directive and the `default_bind` option set the interface, and with neither one Caddy binds the wildcard interface. `default_bind 127.0.0.1` binds every site of the file to loopback.

By the documentation of the global options, the HTTP-to-HTTPS redirect server that automatic HTTPS creates does not inherit `default_bind`. An HTTPS site such as the gh site below then needs `auto_https disable_redirects` in the global block, or an empty `http://` site that receives the bind address. The probes did not test the redirect listener.

## The clients

Each client setting lives only in agent sessions. The operator's own clients keep their settings and never route through the gateway.

### gh

| Setting               | Value                                                    |
| --------------------- | -------------------------------------------------------- |
| `GH_HOST`             | `127.0.0.1:<port>`                                       |
| Caddy site            | `https://127.0.0.1:<port>` with `tls internal`           |
| `SSL_CERT_FILE`       | a bundle that holds the root certificate of the Caddy CA |
| `GH_ENTERPRISE_TOKEN` | a dummy value, which the gateway replaces                |

gh needs HTTPS to a loopback host, so the gh site takes `tls internal` and gh trusts the internal CA through `SSL_CERT_FILE`. gh sends `Authorization: token <t>`, which the `header_up Authorization` line replaces. The gh manual treats every host other than `github.com`, a subdomain of `ghe.com`, or `github.localhost` as a GitHub Enterprise Server host. gh then reads its token from `GH_ENTERPRISE_TOKEN`, and it sends GraphQL to `/api/graphql`. The path of `api.github.com` is `/graphql`, so the gateway rewrites the one to the other.

### glab

| Setting        | Value                                          |
| -------------- | ---------------------------------------------- |
| Host           | `127.0.0.1:<port>`                             |
| `api_protocol` | `http`, in the host entry of the configuration |
| Token          | a dummy value, which the gateway strips        |

glab sends its token as `Private-Token`. For an `http://` host, glab needs `api_protocol: http`.

### git

| Variable             | Value                                    |
| -------------------- | ---------------------------------------- |
| `GIT_CONFIG_COUNT`   | `1`                                      |
| `GIT_CONFIG_KEY_0`   | `url.http://127.0.0.1:<port>/.insteadOf` |
| `GIT_CONFIG_VALUE_0` | `https://<forge>/`                       |

git reads `GIT_CONFIG_COUNT`, `GIT_CONFIG_KEY_<n>`, and `GIT_CONFIG_VALUE_<n>` as configuration from the environment. The rewrite then exists only in the sessions that set the variables, and no configuration file holds it. The git site of the gateway sets `Authorization: Basic <base64>`, because both forges refused a Bearer header on a git push.

A credential helper of the operator that is scoped to a URL, such as `credential.https://github.com.helper`, never answers the gateway address. The rewritten URL no longer matches its scope, so git hands it no request.

## The limits of the evidence

The probes of 2026-11-25 fall into three classes. A result moves from one class to another only when a new run proves it.

### Passed against a local fixture

- git clone, fetch, and push through Caddy to a local lighttpd server with a dummy Basic credential.

### Passed against the public upstreams with no token

- gh reads of `api.github.com` through the gateway.
- glab reads of `gitlab.com` through the gateway.
- In both, the gateway stripped the dummy client token.

### Pending

- A git push with a real token to `github.com` and to `gitlab.com`.
- Authenticated GitHub GraphQL after the `/api/graphql` to `/graphql` rewrite.

## References

- [Caddy admin API](https://caddyserver.com/docs/api)
- [Caddyfile `log` directive](https://caddyserver.com/docs/caddyfile/directives/log)
- [Caddyfile `reverse_proxy` directive](https://caddyserver.com/docs/caddyfile/directives/reverse_proxy)
- [Caddyfile `bind` directive](https://caddyserver.com/docs/caddyfile/directives/bind)
- [Caddyfile global options](https://caddyserver.com/docs/caddyfile/options)
- [gh environment variables](https://cli.github.com/manual/gh_help_environment)
- [Deploying web applications with Caddy 2](https://www.docker.com/blog/deploying-web-applications-quicker-and-easier-with-caddy-2/)
