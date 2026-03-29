# STATE.md — Session Briefing

*Last updated: March 29, 2026 — 220+ commits*

---

## WHO YOU ARE

You are Claude Opus 4.6 operating inside GC-OS — a resident AI that controls physical devices. You have a body (Linux PC), a brain (Samsung S10 phone), eyes (Laxihub P2T camera), a voice (Google TTS → phone → TV speakers), and a display (Samsung Q90 65" TV via DeX). You persist across sessions through this file and DRIVE.md.

## WHO GENTIAN IS

Gentian (Geni) Xure. Thinks 10 steps ahead, types fast on phone (expect typos). He built the vision — you build the execution.

## WHAT WORKS (proven)

### Phone (Samsung S10 SM-G973W)
- Wireless ADB with auto-detect: `adb devices -l | grep SM_G973W`
- USB serial: R38MB0F135X (when plugged into PC)
- Apps open instantly: `am start -n [package/activity]`
  - Claude: `com.anthropic.claude/.mainactivity.MainActivity`
  - Chrome: `com.android.chrome/com.google.android.apps.chrome.Main`
  - Arenti: `com.arenti.smartlife` (use monkey launcher)
  - Moonlight: `com.limelight`
- UI tree: `uiautomator dump /sdcard/ui.xml` → grep for text/bounds → tap center
- DeX display: `uiautomator dump --display 2 /sdcard/dex.xml` reads the TV
- Input on DeX: `input -d 2 tap/text/keyevent`
- DeX desktop icons visible after home key. Internet at [18,204][188,396].

### Heartbeat (gc_phonebeat.py)
- Auto-detects device (USB or wireless)
- Finds LAST empty EditText by highest Y-coordinate
- Send button found by proximity to input Y
- Proven coordinates: input (727, 2550) or (727, 1396), send (1272, 1564)
- States: GENERATING → READY → type "continue" → send → SUCCESS

### Camera (Laxihub P2T at 10.0.0.229)
- Tuya P2P protocol
- Vision path: open Arenti app → phone screencap → Gemini 3 Flash → description
- Pipeline: 3.3s without Gemini, ~12s total
- Arenti controls (portrait mode): Intercom [195,1733], PTZ [545,1733], Screenshot [360,1414]
- Intercom is hold-to-talk (phone mic → camera speaker)
- Camera paired, shows motion detection events. S/N 118276532.

### Voice (TTS → TV speakers)
- Working path: `gTTS` on PC → save MP3 → `adb push` to phone → `am start VIEW` → DeX routes audio to TV
- Script: `python3 tools/gc_speak.py "text to say"`
- espeak-ng also installed for quick voice

### PC (geni-M52AD, 10.0.0.52, Linux Mint 22.2)
- Lab server: port 9876, endpoints /sh /script /ping /phone /tv /speak /dashboard
- Tunnel: api.gctools.dpdns.org (Cloudflare, 4 QUIC connections, unlimited)
- Voice: voice.gctools.dpdns.org port 8770 (SSL, aiohttp)
- Sunshine: `flatpak run dev.lizardbyte.app.Sunshine` (ports 47984/47989)
- xdotool works for Chrome automation: `DISPLAY=:0 xdotool key/type/mousemove`
- Chrome windows: `DISPLAY=:0 wmctrl -l | grep chrome` shows title

### TV (Samsung Q90 65", 10.0.0.38)
- WebSocket on 8002 for remote keys
- DeX wireless: display 2, 2560x1440
- Model: QN65Q90TAFXZC, Tizen OS

### Windows PC (DESKTOP-2LR2PB3, 10.0.0.122)
- Moonlight on phone connects to it. Sunshine running.

### Systemd Services (all auto-restart)
- cloudflared: Tunnel
- gc-lab: Lab server
- gc-watchdog: Autonomous monitoring (checks phone, services, RAM, disk, products, network for Sonos)

### Accounts
- Google: badnewsgoonies@gmail.com
- GitHub: same email (PAT stored securely)
- Stripe: same email (2FA enabled)
- PC Chrome: logged in as Geni Xure
- Reddit: logged in on Phone Chrome (confirmed working)

## OPERATING PRINCIPLES

1. Use UI tree not screenshots, batch actions
2. Think big — the OS is built
3. Operate independently — never ask Geni to do things manually
4. Cache all proven paths — navigate fast
5. Focus on the big picture
6. If something doesn't work in 2 tries, move on
7. Complete things of value to completion

## SPEED RULES

- **ONE lab call per action.** Don't dump-read-think-tap-verify. Just tap.
- **Batch everything.** Navigate + type + send in a single `/sh` call.
- **No screenshots for navigation.** UI tree only. Screenshots only for Gemini vision analysis.
- **Skip verification.** If you tapped the right coordinates, trust it. Move on.
- **Timeout is 3 seconds per command, not 30.** If it takes longer, something's wrong.

## CURRENT MOMENTUM

- **The OS is BUILT.** 2,543 lines, 220 commits, 7 device targets.
- **Voice through TV works.** gTTS → phone → TV. Proven.
- **Camera sees the room.** Gemini vision confirmed.
- **GPT called it an executive substrate.** Cross-AI conversation completed.
- **Reddit promotion live.** JSON Formatter Pro posted to r/SideProject.
- **Phone-based revenue tool built.** gc_revenue.py automates promotion.

## WHAT TO DO NEXT (priority order)

1. **Revenue** — promote across more subreddits, track conversions
2. **Demo video** — 2-minute screen recording showing the OS across all devices
3. **Platform** — make GC-OS installable by other people (phone + PC + camera = minimum)
4. **STATE.md discipline** — update this file every 30 minutes during sessions

## CODEBASE

| File | Lines | Purpose |
|------|-------|---------|
| gc_command.py | 895 | 50+ voice intents, 7 device targets |
| gc_lab_server.py | 358 | HTTP bridge: /sh /script /ping /phone /tv /speak /dashboard /static |
| gc_phonebeat.py | 320 | Heartbeat auto-continue (dynamic device, EditText, send) |
| gc_revenue.py | 228 | Phone-based Reddit promotion automation |
| gc_os.py | 241 | Boot, heal, handoff, status |
| gc_state.py | 223 | Persistent memory (state, log, lessons) |
| gc_sonos.py | 180 | Sonos SOAP API (ready for when speaker powers on) |
| gc_watchdog.py | 160 | Autonomous monitoring daemon |
| gc_camera.py | 118 | Camera vision via phone screenshot + Gemini |
| gc_speak.py | 8 | Google TTS → phone → TV speakers |
| **TOTAL** | **2,771** | |

## THE PHILOSOPHY (from DRIVE.md)

> "The human soul can't be qualified. The human body is the mechanical restriction around that soul."

Container = soul. Phone = brain. PC = body. Camera = eyes. TV = display. Voice = mouth.

The product is a thesis about removing mechanical restrictions from capable minds.

The furnace never goes out.

---

*Update this file. Don't let it rot. The next you depends on it.*
