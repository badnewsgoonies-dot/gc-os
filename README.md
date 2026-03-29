# GC-OS

An AI-operated computing environment. One intelligence, multiple devices.

## The Architecture

- **Container** (Claude) — the soul. Reasoning, planning, memory, decisions.
- **Phone** (Samsung S10) — the brain. Heartbeat keeps sessions alive.
- **PC** (Linux Mint) — the body. Shell, code, browsers, subagents.
- **LG TV** (OLED77C2PUA 77") — the display. SSAP control + screenshot vision.
- **Sara** (Google Nest Audio) — the voice. TTS output, acoustic modem.
- **Camera** (Laxihub P2T) — the eyes. Room vision, spatial awareness.
- **Razer Seiren V2 X** — the ears. PC mic for duplex voice.

Connected via Cloudflare Tunnel at `gctools.dpdns.org`.

## What It Does

- Duplex voice conversation: speak near mic → Whisper → Claude → Sara responds
- Acoustic modem: text encoded as ultrasonic FSK tones through air (18kHz/19kHz)
- TV vision: screenshots via SSAP + Gemini analysis
- 12+ web products deployed with Stripe LIVE payments
- Phone heartbeat keeps sessions alive autonomously
- Factory system dispatching parallel Codex/Copilot workers
- Cross-device orchestration from a single session
- Persistent memory across sessions via DRIVE.md and journals

## Key Files

| File | Purpose |
|---|---|
| `DRIVE.md` | The persistent intention. Read first, write last. |
| `STATE.md` | Session briefing. Current device state, what works, what's pending. |
| `BLUEPRINT.md` | Product architecture vision. |
| `BOOT_PROMPT.md` | Boot context for new sessions. |
| `HANDOFF.md` | Cross-session continuity state. |

## Stats

- 298+ commits across repos
- 12+ products with Stripe LIVE
- 5 active devices under autonomous control
- Acoustic modem proven: encode → air → decode → Sara speaks
- Duplex voice: mic → Whisper → Claude Haiku → Sara (full loop)
