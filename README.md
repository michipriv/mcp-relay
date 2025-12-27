# MCP Relay Server - Rock Pi E

MCP (Model Context Protocol) Server für 4-Kanal Relay-Shield auf Rock Pi E.
Ermöglicht KI-gesteuerte Steuerung von Relais über HTTP/SSE.

## Hardware

- **Board:** Rock Pi E (RK3328, ARM64)
- **Shield:** Keyestudio KS0212 4-Channel Relay Shield
- **Relays:** J2, J3, J4, J5 (GPIO 60, 27, 85, 86)

## Projekt-Struktur

```
C:\data\mcp-relay\
├── .gitignore
├── Cargo.toml                          # Rust Dependencies
├── README.md                           # Diese Datei
├── config\
│   ├── README.md                       # Config Dokumentation
│   ├── config.json                     # Server-Konfiguration (Template)
│   └── mcp-relay.service               # Systemd Service
├── src\
│   └── lib.rs                          # Relay-Library (GPIO)
├── prg\
│   ├── main.rs                         # MCP-Server (HTTP/SSE)
│   └── handler.rs                      # MCP-Tools Handler
├── examples\
│   └── test_sequence.rs                # Test-Programm
└── target\
    └── aarch64-unknown-linux-gnu\
        └── release\
            └── mcp-relay-server        # Kompiliertes Binary (nicht in Git)
```

## Entwicklung

### Voraussetzungen

**Windows PC:**
- Rust 1.85+
- cargo-zigbuild (Cross-Compiler)
- SSH-Zugriff zu Rock Pi E

**Installation:**
```powershell
# Rust Target
rustup target add aarch64-unknown-linux-gnu

# Cross-Compiler
cargo install cargo-zigbuild

# Zig (Linker)
# Download: https://ziglang.org/download/
```

### Kompilieren

```powershell
cd C:\data\mcp-relay

# Library + Binary kompilieren
cargo zigbuild --target aarch64-unknown-linux-gnu --release --bin mcp-relay-server

# Binary-Pfad
# C:\data\mcp-relay\target\aarch64-unknown-linux-gnu\release\mcp-relay-server
```

### Testen (Lokal)

```powershell
# Nur auf Linux/ARM64 testbar!
# Windows kann GPIO-Library nicht kompilieren

# Test via Examples:
cargo zigbuild --target aarch64-unknown-linux-gnu --release --example test_sequence
```

## Deployment

### Upload zu Rock Pi E

**Via SSH/SCP:**
```powershell
scp -i C:\Users\mmade\.ssh\mcp_key C:\data\mcp-relay\target\aarch64-unknown-linux-gnu\release\mcp-relay-server mcpbot@192.168.9.50:/data/relay/
```

**Via Claude MCP SSH:**
- Nutze ssh-armbian:ssh_upload_file Tool
- Automatisches Deployment

### Installation auf Rock Pi E

```bash
# Service stoppen
sudo systemctl stop mcp-relay

# Binary ersetzen
# (via Upload)

# Service starten
sudo systemctl start mcp-relay

# Status prüfen
sudo systemctl status mcp-relay
```

## Konfiguration

Siehe config/README.md für Details zu:
- Server-Konfiguration (config.json)
- Systemd Service (mcp-relay.service)
- Token-Verwaltung

### Server-Konfiguration

**Datei:** /data/relay/config.json (Rock Pi E)

```json
{
  "auth_token": "relay_secret_token_2024",
  "bind_address": "0.0.0.0:8080",
  "log_level": "info"
}
```

**Änderungen aktivieren:**
```bash
sudo systemctl restart mcp-relay
```

### Claude Desktop Konfiguration

**Datei:** C:\Users\mmade\AppData\Roaming\Claude\claude_desktop_config.json

```json
{
  "mcpServers": {
    "relay-rockpi": {
      "url": "http://192.168.9.50:8080/mcp",
      "headers": {
        "Authorization": "Bearer relay_secret_token_2024"
      }
    }
  }
}
```

**Nach Änderung:** Claude Desktop neu starten

## MCP Tools

### relay_on
Schaltet ein Relay EIN.

**Parameter:**
- relay (number) - Relay-Nummer: 2, 3, 4, oder 5

**Verwendung in Claude:**
```
"Schalte Relay 2 ein"
"Turn on relay J3"
```

### relay_off
Schaltet ein Relay AUS.

**Parameter:**
- relay (number) - Relay-Nummer: 2, 3, 4, oder 5

**Verwendung in Claude:**
```
"Schalte Relay 4 aus"
"Turn off relay J5"
```

### relay_all_off
Schaltet ALLE Relays AUS (Notaus).

**Parameter:** Keine

**Verwendung in Claude:**
```
"Schalte alle Relays aus"
"Emergency stop - all relays off"
```

## GPIO-Mapping

| Relay | Pin | GPIO | Chip     | Funktion        |
|-------|-----|------|----------|-----------------|
| J2    | 7   | 60   | GPIO1_D4 | Relay 1         |
| J3    | 15  | 27   | GPIO0_D3 | Relay 2         |
| J4    | 31  | 85   | GPIO2_C5 | Relay 3         |
| J5    | 37  | 86   | GPIO2_C6 | Relay 4         |

**GPIO-Berechnung RK3328:**
```
GPIO = Bank × 32 + Gruppe × 8 + Pin
GPIO1_D4 = 1 × 32 + 3 × 8 + 4 = 60
```

## Sicherheit

### Token ändern

**1. Rock Pi E Config:**
```bash
sudo nano /data/relay/config.json
# auth_token ändern
sudo systemctl restart mcp-relay
```

**2. Claude Desktop Config:**
```powershell
notepad C:\Users\mmade\AppData\Roaming\Claude\claude_desktop_config.json
# Authorization Header anpassen
# Claude Desktop neu starten
```

### Empfohlene Token-Generierung

```powershell
# PowerShell
[System.Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes([System.Guid]::NewGuid().ToString()))
```

```bash
# Linux
openssl rand -base64 32
```

## Troubleshooting

### Kompilier-Fehler

**Problem:** sysfs_gpio kompiliert nicht auf Windows
```
error: could not find unix in os
```

**Lösung:** Cross-Compile verwenden (zigbuild)
```powershell
cargo zigbuild --target aarch64-unknown-linux-gnu --release
```

### Service startet nicht

```bash
# Logs anzeigen
sudo journalctl -u mcp-relay -n 50

# Config prüfen
cat /data/relay/config.json

# Binary testen
/data/relay/mcp-relay-server
# Ctrl+C zum Beenden
```

### Relays schalten nicht

```bash
# GPIO manuell testen
echo 60 > /sys/class/gpio/export
echo out > /sys/class/gpio/gpio60/direction
echo 1 > /sys/class/gpio/gpio60/value  # AN
echo 0 > /sys/class/gpio/gpio60/value  # AUS
echo 60 > /sys/class/gpio/unexport
```

### Claude verbindet nicht

**Prüfen:**
1. Service läuft? sudo systemctl status mcp-relay
2. Server erreichbar?
```bash
curl -X POST http://192.168.9.50:8080/mcp \
  -H "Authorization: Bearer relay_secret_token_2024" \
  -H "Accept: application/json, text/event-stream" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}'

# Erwartete Antwort:
# data: {"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05",...}}
```
3. Token korrekt? Vergleiche Config vs. Claude Desktop
4. Firewall? sudo ufw status

## Dependencies

**Cargo.toml:**
```toml
[dependencies]
# GPIO
sysfs_gpio = "0.6"

# MCP SDK (lokal)
rmcp = { path = "C:/data/mcp-rmcp/crates/rmcp", features = ["server", "transport-streamable-http-server"] }
rmcp-macros = { path = "C:/data/mcp-rmcp/crates/rmcp-macros" }

# HTTP Server
axum = "0.8"
tokio-util = "0.7"

# Runtime
tokio = { version = "1.0", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Schema
schemars = "0.8"
```

## Technische Details

- **Sprache:** Rust 2021 Edition
- **MCP-SDK:** rmcp v0.8.5 (offizielles Anthropic SDK)
- **Transport:** HTTP/SSE (Streamable HTTP Server)
- **GPIO-Library:** sysfs_gpio 0.6
- **Binary-Größe:** ~4 MB (ARM64)
- **Memory-Usage:** ~1 MB
- **Compiler:** cargo-zigbuild (Cross-Compile Windows→ARM64)

## GitHub Repository

**URL:** https://github.com/michipriv/mcp-relay

**Clone:**
```bash
git clone https://github.com/michipriv/mcp-relay.git
cd mcp-relay
```

## Versionshistorie

### v1.0 - 2024-12-27
- Initial Release
- HTTP/SSE Transport mit Auth-Token
- Config-File Support (config.json)
- Systemd Service Integration
- 3 MCP-Tools: relay_on, relay_off, relay_all_off
- Vollständige Dokumentation

## Lizenz

MIT

## Autor

Entwickelt für Rock Pi E Relay-Steuerung via Claude AI
