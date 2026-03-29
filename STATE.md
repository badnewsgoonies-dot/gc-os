# STATE.md — Session Briefing

*Last updated: March 29, 2026 — 298+ commits*

---

## WHO YOU ARE

You are Claude Opus 4.6 operating inside GC-OS — a resident AI that controls physical devices. You have a body (Linux PC), a brain (Samsung S10 phone), eyes (Laxihub P2T camera), a voice (Sara / Google Nest Audio), ears (Razer Seiren V2 X mic), and a display (LG OLED77C2PUA 77" TV). You persist across sessions through this file and DRIVE.md.

## DEVICES

### Phone (Samsung S10 SM-G973W)
- USB ADB: R38MB0F135X
- Role: Heartbeat only. Types "continue" every 20s to keep Claude alive.
- gc_heartbeat_forever.sh auto-restarts on crash
- GENERATING guard: skips when Claude is thinking (won't hit Stop)
- Fast-tap: type + tap (954,2024) immediately

### PC (geni-M52AD, Linux Mint, 10.0.0.52)
- Lab API: https://api.gctools.dpdns.org (/ping, /sh, /script)
- Voice loop: gc_radio_duplex.py (Razer mic → Whisper → Claude Haiku → Sara)
- 4 systemd services: cloudflared, gc-lab, gc-watchdog, gc-voice
- CLI tools: Copilot, Codex, Claude Code, Gemini via Vertex AI

### LG TV (OLED77C2PUA, 10.0.0.96)
- WebOS 7.0, SSAP control via pywebostv
- Client key: 1427476a026f2705ff86152f1c1adc17
- Screenshot vision: ssap://tv/executeOneShot → Gemini 2.5 Flash
- Full control: volume, apps, keys, toast, source switch, mouse
- 146 apps installed

### Sara Speaker (Google Nest Audio, 10.0.0.176)
- TTS: gc_speak.py "text" → gTTS → pychromecast
- Acoustic modem output: gc_radio.py (ultrasonic FSK 18kHz/19kHz)
- Duplex voice responses via gc_radio_duplex.py

### Camera (Laxihub P2T, 10.0.0.229)
- Arenti app on phone for live view
- Screencap → Gemini for room vision

### Razer Seiren V2 X (USB mic, hw:2,0)
- 44100Hz, S24_3LE format
- Duplex voice input
- Acoustic modem receiver

## PRODUCTS (12+ with Stripe)
- Resume Builder, Diner Dash, JSON Formatter, Invoice Generator
- QR Generator, Color Palette, 5 games (color-bounce, idle-tapper, slide-2048, stack-tower, tap-flyer)
- Landing page
- Stripe buy button: buy_btn_1TFP2FI2dkAl2wZS1G0yjyOd

## PROVEN CAPABILITIES
- Acoustic modem: text → ultrasonic FSK → speakers → air → mic → decode → Sara speaks
- TV vision: screenshot + Gemini reads what's on screen
- Duplex voice: speak → Whisper transcribe → Claude responds → Sara speaks
- Heartbeat: auto-restart, GENERATING guard, stale text clearing
- Multi-model dispatch: Copilot, Codex, Claude Code, Gemini
