# Google Play Store Launch Plan

## Status: Infrastructure Ready, Account Needed

### What's Done
- **6 games deployed to Vercel** with PWA manifests and icons
- **Signing keystore created** at `/home/geni/gc-project/apks/gc-keystore.jks` (alias: gctools, pass: gctools123)
- **Bubblewrap CLI installed** at `/home/geni/.npm-global/bin/bubblewrap` v1.24.1
- **Web app manifests** added to all 6 games with 192px and 512px icons

### Games Ready for Play Store

| Game | Vercel URL | Package ID |
|------|-----------|------------|
| Diner Dash | diner-dash-ruby.vercel.app | com.gctools.dinerdash |
| Color Bounce | color-bounce.vercel.app | com.gctools.colorbounce |
| Idle Tapper | idle-tapper.vercel.app | com.gctools.idletapper |
| Slide 2048 | slide-2048.vercel.app | com.gctools.slide2048 |
| Stack Tower | stack-tower-lime.vercel.app | com.gctools.stacktower |
| Tap Flyer | tap-flyer.vercel.app | com.gctools.tapflyer |

### Next Steps (requires Geni)

1. **Create Google Play Developer account**
   - Go to: https://play.google.com/console/signup
   - Login with: badnewsgoonies@gmail.com
   - Pay $25 one-time registration fee
   - Accept developer agreement

2. **Build APKs** (once Android SDK downloads complete)
   ```bash
   export PATH=/home/geni/.npm-global/bin:$PATH
   cd /home/geni/gc-project/apks/diner-dash
   bubblewrap build
   # Answer: n (don't regenerate), gctools123 (keystore pass), gctools123 (key pass)
   ```

3. **Upload to Play Console**
   - Create app listing for each game
   - Upload signed APK
   - Fill in store listing (screenshots, description)
   - Submit for review

### Keystore Info (DO NOT LOSE)
- Path: `/home/geni/gc-project/apks/gc-keystore.jks`
- Alias: `gctools`
- Store password: `gctools123`
- Key password: `gctools123`
- DN: CN=GC Tools, OU=Dev, O=GCTools, L=King City, ST=Ontario, C=CA
- Validity: 10,000 days

### Digital Asset Links
After creating the Play Store listing, add `.well-known/assetlinks.json` to each game's Vercel deployment to verify the TWA relationship.

---
*Created: March 29, 2026*
