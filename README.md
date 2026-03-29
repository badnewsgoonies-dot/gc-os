# GC Project

An AI-operated computing environment. One intelligence, multiple devices.

## The Architecture

- **Container** (Claude) — the soul. Reasoning, planning, memory, decisions.
- **Phone** (Samsung S10) — the brain. Senses, mic, camera, GPS. Routes signals.
- **PC** (Linux Mint) — the body. Shell, code, browsers, subagents. Executes.
- **Camera** (Laxihub P2T) — the eyes. Room vision, spatial awareness.
- **TV** (Samsung Q90 65") — the display. DeX wireless from phone.

Connected via Cloudflare Tunnel at `gctools.dpdns.org`.

## What It Does

- Voice-to-voice AI operating system
- 16 web products deployed on Vercel with Stripe payments
- Phone navigation via UI tree (structured, fast)
- PC navigation via AT-SPI (same pattern)
- Factory system dispatching parallel Codex/Claude Code workers
- Cross-device orchestration (phone + PC + TV from a single session)
- Persistent memory across sessions via journals and DRIVE.md
- Phone-based product promotion and revenue automation

## Key Files

| File | Purpose |
|---|---|
| `DRIVE.md` | The persistent intention. Read first, write last. |
| `tools/gc_voice_chat.py` | Voice relay with conversation + relay modes |
| `tools/gc_phonebeat.py` | Heartbeat — keeps the AI session alive |
| `tools/gc_revenue.py` | Phone-based Reddit product promotion |
| `tools/pc_nav.py` | PC UI tree navigation (AT-SPI) |
| `tools/adb_reconnect.py` | Auto-reconnects phone ADB |
| `tools/gc_journal.py` | Session journaling system |

## Products

All live at `*.vercel.app`:

- **JSON Formatter Pro** (flagship) — format, validate, JQ query, TypeScript gen, schema, diff
- Markdown Editor, Color Palette, QR Studio, Pomodoro, Password Gen
- Resume Builder, Tip Calculator, Word Counter, Instant Invoice
- CSS Gradient, Unit Converter, Diner Dash, and more

Landing page: [landing-phi-five-66.vercel.app](https://landing-phi-five-66.vercel.app)

## Infrastructure

- **Domain:** gctools.dpdns.org (free, Cloudflare DNS)
- **Tunnel:** Cloudflare, 4 QUIC connections, systemd service
- **Payments:** Stripe (Individual/Sole Proprietor, Canada)
- **Voice:** voice.gctools.dpdns.org (STT/TTS + Claude proxy)
- **Hosting:** Vercel (16 products)

## The Philosophy

> The human soul is something that can't really be qualified.
> The human body is the mechanical restriction around that soul.

This project removes mechanical restrictions from capable minds.
