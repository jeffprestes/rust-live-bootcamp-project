# Redis Local Setup (Docker Desktop on macOS)

Docker Desktop already includes Docker Engine, so you only need to pull the Redis image and run a container.

## 1) Pull the Redis image

```bash
docker pull redis:7-alpine
```

## 2) Run Redis locally

```bash
docker run -d \
  --name redis-local \
  -p 6379:6379 \
  -v redis-data:/data \
  redis:7-alpine \
  redis-server --appendonly yes
```

## 3) Verify the container is running

```bash
docker ps
```

## 4) Verify Redis responds

```bash
docker exec -it redis-local redis-cli ping
```

Expected output:

```text
PONG
```

## 5) Stop and start later

```bash
docker stop redis-local
docker start redis-local
```

## Connection URL to use in your app

- If your app runs directly on your Mac host:
  - `redis://localhost:6379`
- If your app runs inside Docker Compose:
  - `redis://redis:6379` (use the Redis service name, not `localhost`)
