# Configuration Files

This directory contains configuration files and templates for the MCP Relay Server.

## Files

### config.json
Server configuration template for the MCP Relay Server.

**Location on Rock Pi E:** `/data/relay/config.json`

**Parameters:**
```json
{
  "auth_token": "relay_secret_token_2024",
  "bind_address": "0.0.0.0:8080",
  "log_level": "info"
}
```

- `auth_token` - Bearer token for HTTP authentication
- `bind_address` - Server bind address (IP:Port)
- `log_level` - Logging level: trace, debug, info, warn, error

**Security:**
- Change `auth_token` to a secure random value
- Update Claude Desktop config with matching token

### mcp-relay.service
Systemd service unit file for auto-starting the MCP Relay Server on boot.

**Location on Rock Pi E:** `/etc/systemd/system/mcp-relay.service`

**Installation:**
```bash
# Copy service file
sudo cp mcp-relay.service /etc/systemd/system/

# Reload systemd
sudo systemctl daemon-reload

# Enable auto-start
sudo systemctl enable mcp-relay

# Start service
sudo systemctl start mcp-relay

# Check status
sudo systemctl status mcp-relay
```

**Service Management:**
```bash
# Start
sudo systemctl start mcp-relay

# Stop
sudo systemctl stop mcp-relay

# Restart
sudo systemctl restart mcp-relay

# Status
sudo systemctl status mcp-relay

# Logs
sudo journalctl -u mcp-relay -f
```

## Deployment

### Initial Setup

1. **Upload files to Rock Pi E:**
   ```bash
   scp config.json mcpbot@192.168.9.50:/data/relay/
   scp mcp-relay.service mcpbot@192.168.9.50:/tmp/
   ```

2. **Install service:**
   ```bash
   ssh mcpbot@192.168.9.50
   sudo mv /tmp/mcp-relay.service /etc/systemd/system/
   sudo systemctl daemon-reload
   sudo systemctl enable mcp-relay
   sudo systemctl start mcp-relay
   ```

3. **Verify:**
   ```bash
   sudo systemctl status mcp-relay
   ```

### Configuration Updates

1. **Edit config:**
   ```bash
   ssh mcpbot@192.168.9.50
   sudo nano /data/relay/config.json
   ```

2. **Apply changes:**
   ```bash
   sudo systemctl restart mcp-relay
   ```

3. **Update Claude Desktop:**
   - Edit `claude_desktop_config.json`
   - Update `Authorization` header with new token
   - Restart Claude Desktop

## Security Notes

⚠️ **Important:**
- Never commit real auth tokens to Git
- Generate unique tokens for each installation
- Use strong random tokens (min. 32 chars)
- Keep tokens synchronized between:
  - `/data/relay/config.json` (Rock Pi E)
  - `claude_desktop_config.json` (Windows PC)

**Generate secure token:**
```powershell
# PowerShell
[System.Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes([System.Guid]::NewGuid().ToString()))
```

```bash
# Linux
openssl rand -base64 32
```
