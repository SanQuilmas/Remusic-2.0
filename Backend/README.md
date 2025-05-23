# Remusic API

All done inside the remusic-back folder

```bash
podman build -t backend -f Dockerfile
```

```bash
podman run -d --name backend --network mynetwork -p 8080:8080 backend
```
