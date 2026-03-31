# Docker Deployment Guide

## Building the Image

```bash
# Build locally
docker build -t xswd-relayer:latest .

# Or using docker-compose
docker-compose build
```

## Running the Container

### Setup logs directory (required for file logging)
The container runs as user `relayer` (UID 1000) and needs write access to the logs directory:

```bash
# Create logs directory with correct permissions
mkdir -p logs
sudo chown -R 1000:1000 logs
chmod -R 755 logs
```

### Using docker-compose (recommended)
```bash
docker-compose up -d
```

### Using docker run
```bash
docker run -d \
  --name xswd-relayer \
  -p 8080:8080 \
  -v $(pwd)/logs:/app/logs \
  --restart unless-stopped \
  xswd-relayer:latest
```

### With custom configuration
```bash
docker run -d \
  --name xswd-relayer \
  -p 8080:8080 \
  -v $(pwd)/logs:/app/logs \
  --restart unless-stopped \
  xswd-relayer:latest \
  --bind-address 0.0.0.0:8080 \
  --log-level debug \
  --keep-alive-interval 30s \
  --channel-creation-timeout 300s
```

## Pushing to Docker Registry

### Docker Hub
```bash
# Tag the image
docker tag xswd-relayer:latest YOUR_DOCKERHUB_USERNAME/xswd-relayer:latest
docker tag xswd-relayer:latest YOUR_DOCKERHUB_USERNAME/xswd-relayer:v0.1.0

# Push to registry
docker push YOUR_DOCKERHUB_USERNAME/xswd-relayer:latest
docker push YOUR_DOCKERHUB_USERNAME/xswd-relayer:v0.1.0
```

### Private Registry
```bash
# Tag for private registry
docker tag xswd-relayer:latest registry.example.com/xswd-relayer:latest

# Login to registry
docker login registry.example.com

# Push
docker push registry.example.com/xswd-relayer:latest
```

## Cloudflare Tunnel Setup

### Running cloudflared alongside the relayer

You can run cloudflared in two ways:

#### Option 1: As a separate systemd service (recommended for VPS)
```bash
# Install cloudflared
curl -L https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64 -o cloudflared
chmod +x cloudflared
sudo mv cloudflared /usr/local/bin/

# Create tunnel
cloudflared tunnel create xswd-relayer

# Configure tunnel - create config.yml
sudo mkdir -p /etc/cloudflared
sudo nano /etc/cloudflared/config-xswd.yml
```

**config-xswd.yml:**
```yaml
tunnel: YOUR_TUNNEL_ID
credentials-file: /root/.cloudflared/YOUR_TUNNEL_ID.json

ingress:
  - hostname: xswd.yourdomain.com
    service: ws://localhost:8080
    originRequest:
      noTLSVerify: false
      connectTimeout: 30s
      # WebSocket-specific settings
      disableChunkedEncoding: true
  - service: http_status:404
```

```bash
# Install as systemd service
sudo cloudflared service install --config /etc/cloudflared/config-xswd.yml

# Start the service
sudo systemctl start cloudflared
sudo systemctl enable cloudflared

# Check status
sudo systemctl status cloudflared
```

#### Option 2: In same docker-compose (simpler but less flexible)
Uncomment the cloudflared service in docker-compose.yml and add your tunnel token.

### DNS Configuration

In your Cloudflare DNS dashboard:

1. **Add CNAME record:**
   - Type: `CNAME`
   - Name: `xswd` (or whatever subdomain you want)
   - Target: `YOUR_TUNNEL_ID.cfargotunnel.com`
   - Proxy status: Proxied (orange cloud) ✅

2. **WebSocket Support:**
   - Cloudflare tunnels support WSS (WebSocket Secure) natively
   - No special configuration needed - WSS works out of the box
   - The tunnel automatically handles TLS termination

3. **Connect from frontend:**
   ```typescript
   const relayerUrl = 'wss://xswd.yourdomain.com'
   ```

### Important Notes for WebSocket over Cloudflare Tunnel

✅ **Works perfectly for WSS:**
- Cloudflare tunnels fully support WebSocket connections
- TLS is automatically handled (wss:// not ws://)
- No special headers or configuration needed

✅ **Multiple tunnels on same VPS:**
- You can run multiple cloudflared instances with different configs
- Each tunnel can point to different local services
- Use different config files: `/etc/cloudflared/config-forge.yml`, `/etc/cloudflared/config-xswd.yml`

✅ **CNAME records work great:**
- Point your CNAME to `{tunnel-id}.cfargotunnel.com`
- Enable proxy (orange cloud) for security and caching benefits
- Traffic flows: Client → Cloudflare → Tunnel → Your VPS:8080

## VPS Deployment Workflow

```bash
# On VPS - create directory structure with correct permissions
sudo mkdir -p /opt/xswd-relayer/logs
sudo chown -R 1000:1000 /opt/xswd-relayer/logs
sudo chmod -R 755 /opt/xswd-relayer/logs

# Pull from your registry
docker pull YOUR_USERNAME/xswd-relayer:latest

# Run the container
docker run -d \
  --name xswd-relayer \
  -p 127.0.0.1:8080:8080 \
  -v /opt/xswd-relayer/logs:/app/logs \
  --restart unless-stopped \
  YOUR_USERNAME/xswd-relayer:latest

# Verify it's running
docker ps
docker logs xswd-relayer

# Test WebSocket locally
curl -i -N -H "Connection: Upgrade" -H "Upgrade: websocket" http://localhost:8080/ws
```

## Monitoring

```bash
# View logs
docker logs -f xswd-relayer

# Check resource usage
docker stats xswd-relayer

# Health check
docker inspect --format='{{.State.Health.Status}}' xswd-relayer
```

## Configuration Options

All command-line options from `xswd-relayer --help` can be passed to the container:

- `--bind-address` - IP:Port to bind (default: 0.0.0.0:8080)
- `--log-level` - Log level: off, error, warn, info, debug, trace
- `--keep-alive-interval` - WebSocket ping interval (default: 60s)
- `--channel-creation-timeout` - Max wait for peer join (default: 120s)
- `--session-message-timeout` - Message delivery timeout (default: 1s)
- `--max-frame-size` - Max WebSocket frame size in bytes (default: 65536)
- `--prometheus-enable` - Enable Prometheus metrics
- `--prometheus-route` - Metrics endpoint route (default: /metrics)
- `--disable-interactive-mode` - Disable CLI (required for Docker)
- `--disable-ascii-art` - Disable startup banner

## Security Considerations

1. **Non-root user:** Container runs as user `relayer` (UID 1000)
2. **Bind to localhost only:** When behind cloudflared, bind to `127.0.0.1:8080` instead of `0.0.0.0:8080`
3. **Firewall:** If using cloudflared tunnel, you don't need to expose port 8080 publicly
4. **Resource limits:** Adjust CPU/memory limits in docker-compose.yml based on expected load
