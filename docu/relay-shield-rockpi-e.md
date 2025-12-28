# KS0212 Relay Shield auf Rock Pi E

## Hardware
- **Board:** Rock Pi E (RK3328)
- **Shield:** Keyestudio KS0212 4-Channel Relay Shield
- **Relays:** SONGLE, 10A 250VAC / 30VDC
- **Ansteuerung:** TTL Level (3.3V)

## GPIO Mapping

### Raspberry Pi → Rock Pi E Pinout

| Relay | RPi BCM | RPi Pin | Rock Pi E Pin | Rock Pi E GPIO | GPIO Number |
|-------|---------|---------|---------------|----------------|-------------|
| J2    | 4       | 7       | 7             | GPIO1_D4       | 60          |
| J3    | 22      | 15      | 15            | GPIO0_D3       | 27          |
| J4    | 6       | 31      | 31            | GPIO2_C5       | 85          |
| J5    | 26      | 37      | 37            | GPIO2_C6       | 86          |

### GPIO Berechnung (RK3328)
```
GPIO_Nummer = Bank × 32 + Gruppe × 8 + Pin

Beispiel GPIO2_C5:
= 2 × 32 + 2 × 8 + 5
= 64 + 16 + 5
= 85
```

## Manuelle Steuerung via sysfs

### Einzelnes Relay schalten (Beispiel J2)

```bash
# GPIO exportieren
sudo bash -c 'echo 60 > /sys/class/gpio/export'

# Als Output konfigurieren
sudo bash -c 'echo out > /sys/class/gpio/gpio60/direction'

# Relay EIN
sudo bash -c 'echo 1 > /sys/class/gpio/gpio60/value'

# Relay AUS
sudo bash -c 'echo 0 > /sys/class/gpio/gpio60/value'

# GPIO freigeben
sudo bash -c 'echo 60 > /sys/class/gpio/unexport'
```

### Alle Relays nacheinander testen

```bash
# Alle GPIOs exportieren und als Output setzen
for gpio in 60 27 85 86; do
  sudo bash -c "echo $gpio > /sys/class/gpio/export"
  sudo bash -c "echo out > /sys/class/gpio/gpio$gpio/direction"
done

# Sequenziell schalten (J2→J3→J4→J5)
for gpio in 60 27 85 86; do
  echo "Aktiviere GPIO $gpio"
  sudo bash -c "echo 1 > /sys/class/gpio/gpio$gpio/value"
  sleep 1
  sudo bash -c "echo 0 > /sys/class/gpio/gpio$gpio/value"
  sleep 0.5
done

# Aufräumen
for gpio in 60 27 85 86; do
  sudo bash -c "echo $gpio > /sys/class/gpio/unexport"
done
```

## Python Steuerung

### Installation
```bash
sudo apt update
sudo apt install -y python3-gpiod
```

### Beispiel-Script

```python
#!/usr/bin/env python3
import gpiod
import time

# GPIO Mapping
RELAYS = {
    'J2': 60,  # GPIO1_D4
    'J3': 27,  # GPIO0_D3
    'J4': 85,  # GPIO2_C5
    'J5': 86   # GPIO2_C6
}

# GPIO Chip öffnen
chip = gpiod.Chip('gpiochip2')  # RK3328 GPIO Bank 2

# Relay schalten
def relay_on(gpio_num):
    line = chip.get_line(gpio_num)
    line.request(consumer="relay", type=gpiod.LINE_REQ_DIR_OUT)
    line.set_value(1)
    
def relay_off(gpio_num):
    line = chip.get_line(gpio_num)
    line.request(consumer="relay", type=gpiod.LINE_REQ_DIR_OUT)
    line.set_value(0)

# Test alle Relays
for name, gpio in RELAYS.items():
    print(f"Aktiviere {name} (GPIO {gpio})")
    relay_on(gpio)
    time.sleep(1)
    relay_off(gpio)
    time.sleep(0.5)
```

## Wichtige Hinweise

- **Spannung:** Rock Pi E liefert 3.3V auf GPIO (Toleranz bis 3.6V)
- **Shield Design:** Original für Raspberry Pi (identischer 40-Pin Header)
- **Physische Pins:** Pin-Nummerierung identisch, aber andere GPIO-Nummern
- **I2C Pins:** Pin 27/28 nicht für GPIO nutzbar (I2C1)

## Getestet
- **Datum:** 2024-12-27
- **System:** Rock Pi E, Armbian
- **Kernel:** 6.12.58-current-rockchip64
- **Status:** Alle 4 Relays funktional ✓

## Quellen
- Keyestudio KS0212 Datasheet
- Radxa Rock Pi E V1.2 Pinout
- RK3328 GPIO Dokumentation
