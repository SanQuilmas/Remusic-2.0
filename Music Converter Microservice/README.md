# Remusic API

All done inside the remusic-conversion folder

```bash
podman build -t conversion -f Dockerfile
```

```bash
podman run -d --name conversion --network mynetwork -p 8060:8060 -v /home/leonel/Virtual\ Machines/remusic/temp:/app/temp conversion
```
