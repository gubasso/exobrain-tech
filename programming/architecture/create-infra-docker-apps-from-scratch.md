# Create Infra with Docker Apps From Scratch

## Prerequisites

- A VPS reachable over SSH, to act as the entry node.
- Docker and Docker Compose installed on it.
- A domain whose records point at that VPS.

## Steps

1. Stand up the entry VPS.
   - Netmaker, for the overlay network between nodes.
   - [Traefik](../../infra/networking/traefik.md), as the reverse proxy in front of everything else.
2. Add [Portainer](../../infra/containers/docker-portainer.md) as the container manager.
3. Put CrowdSec behind Traefik [^sec1].
4. Put Authelia in front of the apps for single sign-on and 2FA [^auth1].
5. Verify each service answers on its own hostname, through Traefik and past Authelia.

## After

# Basic Apps: Structure and Security

- Reverse Proxy: [Traefik](../../infra/networking/traefik.md)
- Containers Manager: [Portainer](../../infra/containers/docker-portainer.md)
- Security:
  - CrowdSec
  - Authelia

# References

[^sec1]: [Open Source & Collaborative Security with CrowdSec and Traefik - CrowdSec & Traefik Tutorial](https://www.youtube.com/watch?v=-GxUP6bNxF0)
    :techno_tim:

[^auth1]: [2 Factor Auth and Single Sign On with Authelia](https://www.youtube.com/watch?v=u6H-Qwf4nZA)
