# GC-OS HANDOFF — Session 10

## Devices
- Phone: Samsung S10 USB (R38MB0F135X) — heartbeat active
- TV: LG OLED77C2PUA at 10.0.0.96, paired, SSAP + screenshot vision
- Speaker: Sara (Nest Audio) at 10.0.0.176, TTS via gc_speak.py
- PC: 10.0.0.52, Lab API running
- Camera: Laxihub P2T at 10.0.0.229

## Heartbeat
- gc_heartbeat_forever.sh (auto-restarts)
- GENERATING guard: skips when Claude is thinking
- Clears stale text: detects leftover "continue" and clears
- Fast-tap: type + tap (954,2024) immediately
- 30s interval

## Products (7 total, 5 with Stripe)
- Resume Builder: resume-builder-omega-olive.vercel.app
- Diner Dash: diner-dash-ruby.vercel.app
- JSON Formatter: json-formatter-lyart.vercel.app (Stripe LIVE)
- Invoice Generator: products/invoice-generator/index.html
- QR Generator: products/qr-generator/index.html
- Color Palette: products/color-palette/index.html
- Landing: landing-phi-five-66.vercel.app

## Voice
- gc_voice_loop.py running on PC (44100Hz mic, device 3)
- Wake word: "hey jarvis"
- gc_speak.py works standalone for TTS to Sara

## TV Vision
- ssap://tv/executeOneShot → screenshot URL → Gemini 2.5 Flash

## Revenue
- Reddit promotion live on r/SideProject (JSON Formatter Pro)
- gc_revenue.py ready for additional subreddit posts
- Stripe checkout integrated on flagship product

## Commits: 291 on master
