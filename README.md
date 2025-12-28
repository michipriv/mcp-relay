# Relay REST API Server - Rock Pi E

REST API Server für 4-Kanal Relay-Shield auf Rock Pi E.
Ermöglicht Steuerung von Relais über HTTP REST-Endpunkte.

## Hardware

- **Board:** Rock Pi E (RK3328, ARM64)
- **Shield:** Keyestudio KS0212 4-Channel Relay Shield
- **Relays:** 1, 2, 3, 4 (GPIO 60, 27, 85, 86)

## Projekt-Struktur

```
C:\data\mcp-relay\
├── .gitignore
├── Cargo.toml                          # Rust Dependencies
├── README.md                           # Diese Datei
├── config\
│   ├── README.md                       # Config Dokumentation
│   └── config.json                     # Server-Konfiguration
├── src\
│   └── lib.rs                          # Relay-Library (GPIO)
├── prg\
│   └── main.rs                         # REST API Server
└── target\
    └── aarch64-unknown-linux-gnu\
        └── release\
            └── relay-rest-server       # Kompiliertes Binary (nicht in Git)
```

## Entwicklung

### Voraussetzungen

**Windows PC:**
- Rust 1.92+
- cargo-zigbuild (Cross-Compiler)
- SSH-Zugriff zu Rock Pi E

**Installation:**
```powershell
# Rust Target
rustup target add aarch64-unknown-linux-gnu

# Cross-Compiler
cargo install cargo-zigbuild
```

### Kompilieren

```powershell
cd C:\data\mcp-relay

# Binary kompilieren
cargo zigbuild --target aarch64-unknown-linux-gnu --release

# Binary-Pfad
# C:\data\mcp-relay\target\aarch64-unknown-linux-gnu\release\relay-rest-server
```

## Deployment

### Installation auf Rock Pi E

```bash
# Service stoppen
sudo systemctl stop mcp-relay

# Binary ersetzen
sudo cp /tmp/relay-rest-server /data/relay/mcp-relay-server
sudo chmod +x /data/relay/mcp-relay-server

# Service starten
sudo systemctl start mcp-relay

# Status prüfen
sudo systemctl status mcp-relay
```

## Konfiguration

### Server-Konfiguration

**Datei:** `/data/relay/config.json` (Rock Pi E)

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

### Systemd Service

**Datei:** `/etc/systemd/system/mcp-relay.service`

```ini
[Unit]
Description=Relay REST API Server
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/data/relay
ExecStart=/data/relay/mcp-relay-server
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
Environment="CONFIG_PATH=/data/relay/config.json"

[Install]
WantedBy=multi-user.target
```

## REST API

### Endpunkte

**Alle Requests benötigen Authorization Header:**
```
Authorization: Bearer relay_secret_token_2024
```

#### POST /relay/{id}/on
Schaltet Relay EIN (id: 1-4)

**Beispiel:**
```bash
curl -X POST http://192.168.9.50:8080/relay/1/on \
  -H "Authorization: Bearer relay_secret_token_2024"
```

**Response:**
```json
{"relay":1,"state":"on"}
```

#### POST /relay/{id}/off
Schaltet Relay AUS (id: 1-4)

**Beispiel:**
```bash
curl -X POST http://192.168.9.50:8080/relay/2/off \
  -H "Authorization: Bearer relay_secret_token_2024"
```

**Response:**
```json
{"relay":2,"state":"off"}
```

#### POST /relay/all/off
Schaltet ALLE Relays AUS

**Beispiel:**
```bash
curl -X POST http://192.168.9.50:8080/relay/all/off \
  -H "Authorization: Bearer relay_secret_token_2024"
```

**Response:**
```json
{"message":"all relays off"}
```

#### GET /relay/status
Gibt Status aller Relays zurück

**Beispiel:**
```bash
curl http://192.168.9.50:8080/relay/status \
  -H "Authorization: Bearer relay_secret_token_2024"
```

**Response:**
```json
{
  "relays": [
    {"id":1,"state":"on"},
    {"id":2,"state":"off"},
    {"id":3,"state":"on"},
    {"id":4,"state":"off"}
  ]
}
```

#### GET /health
Health-Check Endpunkt

**Beispiel:**
```bash
curl http://192.168.9.50:8080/health \
  -H "Authorization: Bearer relay_secret_token_2024"
```

**Response:**
```json
{"status":"ok"}
```

## GPIO-Mapping

| Relay | Pin | GPIO | Chip     | Funktion |
|-------|-----|------|----------|----------|
| 1     | 7   | 60   | GPIO1_D4 | Relay 1  |
| 2     | 15  | 27   | GPIO0_D3 | Relay 2  |
| 3     | 31  | 85   | GPIO2_C5 | Relay 3  |
| 4     | 37  | 86   | GPIO2_C6 | Relay 4  |

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

### API-Test

```bash
# Health-Check
curl -v http://192.168.9.50:8080/health \
  -H "Authorization: Bearer relay_secret_token_2024"

# Status abfragen
curl -v http://192.168.9.50:8080/relay/status \
  -H "Authorization: Bearer relay_secret_token_2024"
```

## Dependencies

**Cargo.toml:**
```toml
[dependencies]
sysfs_gpio = "0.6"
axum = "0.8"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

## Technische Details

- **Sprache:** Rust 2021 Edition
- **Framework:** Axum 0.8
- **GPIO-Library:** sysfs_gpio 0.6
- **Binary-Größe:** ~2.2 MB (ARM64)
- **Memory-Usage:** <1 MB
- **Compiler:** cargo-zigbuild (Cross-Compile Windows→ARM64)

## Versionshistorie

### v0.2.0 - 2024-12-28
- Umstellung auf REST API
- Entfernung MCP Dependencies
- Endpoints: /relay/{id}/on, /relay/{id}/off, /relay/all/off, /relay/status, /health
- Relay-Nummerierung 1-4 (vorher 2-5)

### v0.1.0 - 2024-12-27
- Initial Release (MCP Server)

## Lizenz

MIT

## Autor

Entwickelt für Rock Pi E Relay-Steuerung
