# Remusic Frontend

All done inside the remusic-front folder

```bash
podman build -t frontend -f Dockerfile
```

```bash
podman run -d --name frontend --network mynetwork -p 5173:80 frontend
```
