# Revenue Campaign Log — March 29, 2026

## Session: Launch Revenue Campaign

**Objective:** Drive first paying customer ($6/mo JSON Formatter Pro)
**Reddit account:** u/No-Student6539
**HN account:** gctools (new, restricted from Show HN for now)

---

## Posts Made

| # | Platform | Subreddit | Title | Type | Product | Status |
|---|----------|-----------|-------|------|---------|--------|
| 1 | Reddit | r/webdev | Built a free JSON formatter with jq queries and TypeScript gen - no ads or login | Text | JSON Formatter Pro | Auto-removed (self-promo rule) |
| 2 | Reddit | r/SideProject | I built 16 browser-based tools and finally launched them all | Text | Landing + JSON Formatter | **LIVE** |
| 3 | Reddit | r/javascript | Made a JSON dev tool with in-browser jq queries, TS interface gen, and schema validation | Link | JSON Formatter Pro | **LIVE** |
| 4 | Reddit | r/InternetIsBeautiful | A collection of 16 free browser tools - JSON formatter, resume builder, invoice maker, QR generator and more | Link | Landing page | **LIVE** |
| 5 | Reddit | r/reactjs | JSON Formatter Pro - free tool with jq queries and TypeScript interface gen, built with vanilla JS | Link | JSON Formatter Pro | **LIVE** |
| 6 | Reddit | r/freelance | Free tools for freelancers - invoice generator, resume builder, JSON formatter and more | Text | Landing page | **LIVE** |
| 7 | Reddit | r/Entrepreneur | Launched 16 web products as a solo dev | Text | Landing page | Skipped (links banned, perma-ban warning) |
| 8 | HN | Show HN | Show HN: JSON Formatter Pro | Link | JSON Formatter Pro | Restricted (new account) |
| 9 | Reddit | r/digitalnomad | Free browser tools for remote workers | Text | Landing page | Abandoned (flair issues) |
| 10 | Reddit | r/indiehackers | Zero to launched: 16 web products | Text | Landing page | Abandoned (flair issues) |

## Results Summary

- **5 posts LIVE** across 5 different subreddits
- **2 removed/blocked** (r/webdev auto-removed, r/Entrepreneur links banned)
- **2 abandoned** (flair requirements caused navigation issues)
- **1 restricted** (HN new account limitation)

## URLs Promoted

- **JSON Formatter Pro:** https://json-formatter-lyart.vercel.app
- **Landing page (all 16 tools):** https://landing-phi-five-66.vercel.app

## Key Learnings

1. r/SideProject is the most permissive — ideal for launch posts
2. r/javascript requires [AskJS] prefix for text posts but allows link posts
3. r/Entrepreneur bans all links with permanent ban warning — avoid
4. r/webdev has aggressive auto-moderation for self-promotion
5. HN restricts Show HN for new accounts due to AI/spam influx
6. Reddit's "Post Check Dialog" warns but still allows submission
7. Subreddits requiring flair need extra navigation steps
8. Phone heartbeat (gc_phonebeat.py) must be killed before phone browser automation

## Next Session TODO

- [ ] Check post engagement (upvotes, comments) on all 5 live posts
- [ ] Reply to any comments to boost engagement
- [ ] Try r/indiehackers and r/digitalnomad with proper flair selection
- [ ] Post to r/coolgithubprojects (if products have GitHub repos)
- [ ] Re-attempt HN Show HN after account ages
- [ ] Check Stripe dashboard for any revenue
- [ ] Post to r/smallbusiness, r/startups, r/webdesign
- [ ] Consider posting Resume Builder specifically to r/resumes, r/jobs
- [ ] Consider posting Invoice Generator to r/smallbusiness, r/accounting

## Technical Notes

- Phone ADB works via USB (R38MB0F135X) and wireless (100.126.93.16:38015)
- Must kill heartbeat processes before phone automation: `kill $(pgrep -f heartbeat) $(pgrep -f phonebeat)`
- Claude app steals focus from input — force-stop before typing
- Use `--es com.android.browser.application_id <unique>` for new tabs
- Parentheses in ADB `input text` break shell — avoid them

---

*Campaign started: March 29, 2026*
*Last updated: March 29, 2026*
