# GC Session Boot — March 2026

## What GC Is
An autonomous AI agent that controls devices (phone, TV, PC, browser) from a Linux PC. A control loop. The body runs on the PC, the brain visits via Claude API.

## Live Infrastructure (all running)

### 4 systemd services on Linux PC (10.0.0.52):
- **gc-core.service** (port 9876) — device orchestration, HTTP API for phone/TV/browser/shell/files
- **gc-ttyd.service** (port 7681) — terminal display for TV
- **gc-proxy.service** (port 3456) — CLIProxyAPI, routes Claude API through Max subscription (free)
- **gc-auto.service** — autonomous brain loop with persistent memory

### Cloudflare Tunnel:
`https://api.gctools.dpdns.org` → port 9876

### Devices:
- Phone: Samsung S10 SM-G973W, Android 12, ADB wireless, accessibility ON
  - ADB binary: `/home/geni/.bubblewrap/android_sdk/platform-tools/adb`
  - Device ID: `adb-R38MB0F135X-fD2EP0._adb-tls-connect._tcp`
- TV: Samsung QN65Q90TAFXZC, 65" 4K, IP 10.0.0.38, WebSocket :8002
- PC: Intel i3-4160, 7.4GB RAM, 457GB disk, Linux Mint 22.2, Ollama (8 models), Docker, DISPLAY=:0
- Windows PC: 10.0.0.122, Moonlight connected
- Camera: Laxihub P2T, 10.0.0.229, WiFi

### Credentials:
- Google: badnewsgoonies@gmail.com / Deadlock12345 (capital D), 2FA via phone
- PC sudo: kkkjjj12
- GitHub: ghp_qtR17wEDQ9tRN1aC8ajx + QsIVt0kwRQ1DvzMI (join to use)
- Repo: github.com/badnewsgoonies-dot/gc-project (220+ commits, master)
- Vercel token: vca_8p5BdBpUzqo1jIpY5zVz4vRHvvrtXNEpDz5Q2WAYIo47qE7xHF2AcxP5 (team: next-era)
- GEMINI_API_KEY: in ~/.gc/env (`export GEMINI_API_KEY=AIzaSyAzqk5FH-E3JIhs_2mUwhwH2IuxeAXHBMk`)

## 13+ Products LIVE on Vercel — ALL PASSING

| # | Product | URL | Score |
|---|---------|-----|-------|
| 1 | Color Palette Generator | https://color-palette-xi.vercel.app | 9/10 |
| 2 | Unit Converter | https://convert-pied-three.vercel.app | 9/10 |
| 3 | CSS Gradient Generator | https://css-gradient-ecru.vercel.app | 9/10 |
| 4 | Diner Dash Game | https://diner-dash-ruby.vercel.app | 8/10 |
| 5 | Instant Invoice | https://instant-invoice-nine.vercel.app | 8/10 |
| 6 | JSON Formatter | https://json-formatter-lyart.vercel.app | 8/10 |
| 7 | Markdown Editor | https://markdown-editor-smoky-ten.vercel.app | 9/10 |
| 8 | Password Generator | https://password-gen-lovat-kappa.vercel.app | 8/10 |
| 9 | Pomodoro Timer | https://pomodoro-omega-rouge.vercel.app | 9/10 |
| 10 | QR Studio | https://qr-studio-opal.vercel.app | 8/10 |
| 11 | Resume Builder | https://resume-builder-omega-olive.vercel.app | 9/10 |
| 12 | Tip Calculator | https://tip-calculator-two-pied.vercel.app | 8/10 |
| 13 | Word Counter | https://word-counter-two-beryl.vercel.app | 9/10 |

Mobile verified on Samsung S10. Screenshots available.

### Key Code:
- tools/gc_core.py — HTTP API for all devices (systemd)
- tools/gc_auto.py — autonomous loop v2 with persistent memory
- tools/gc_memory.py — SQLite-backed work plan, session log, lessons, context
- tools/gc_revenue.py — phone-based Reddit promotion automation
- tools/gc_drill.py — dispatch/observe/evaluate/deploy pipeline
- tools/gc_eval.py — screenshot-based Gemini evaluator
- factory/factory_queue.py — SQLite task queue with atomic claims
- tools/gc_install.sh — one-command PC setup

### Autonomous Loop State:
- Running v2 with memory.db (~/.gc/memory.db)
- 8 active goals bootstrapped
- Uses CLIProxyAPI on localhost:3456 (Max subscription, no per-token cost)
- Cycle count persists across restarts

## Decisions Locked:
- Ship to Vercel first, Chrome Web Store second, Play Store third
- Free + IAP unlock for apps
- Gemini 2.0 Flash for eval
- Cloudflare for domains (at-cost)

## What To Do Next
Products are live and passing eval. The factory is proven. Next priorities:

1. **Revenue** — promote products, drive first sale
2. **Legitimize products** — privacy policies, meta tags, favicons
3. **Chrome Web Store** — package best 3 products as extensions
4. **Play Store** — TWA build for top products
5. **New products** — keep the factory running

## How To Operate
- Call gc_core via Cloudflare tunnel: `curl -s https://api.gctools.dpdns.org/sh -H 'Content-Type: application/json' -d '{"cmd":"..."}'`
- ADB: `/home/geni/.bubblewrap/android_sdk/platform-tools/adb shell am start -a android.intent.action.VIEW -d URL`
- Eval: `GEMINI_API_KEY=... python3 tools/gc_eval.py <url> --name "Name"`
- Push code via git, pull on PC, restart services

## User Info
- Location: King City / Vaughan, Ontario, Canada
- Brings app ideas and direction
- Wants the AI to operate independently at an elevated level
