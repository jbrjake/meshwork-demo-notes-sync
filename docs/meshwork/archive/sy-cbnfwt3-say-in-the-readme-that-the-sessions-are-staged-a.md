---
id: sy-cbnfwt3
title: Say in the README that the sessions are staged and how to replay them
status: done
category: docs
verify: "all(exists README.md, contains README.md /story\\/replay\\.sh/)"
created: 2026-10-02T17:21Z
---

This repo is part of a demo, and the README says so up front. The agent sessions in its history after day 0 are staged: a script performs them with real meshwork commands, real code changes and real outputs. The README names the script, `story/replay.sh` in meshwork-demo-notes-portfolio, and how to run it.

This lands at day 0 because re-recording the story resets `main` to the `story/0-day0` tag, which would drop a README commit made after it.

## log
- 2026-10-02T17:21Z created
- 2026-10-02T17:21Z open→doing — claimed by claude (602c381b-d7db-491e-8df6-85682e6152ed)
- 2026-10-02T17:21Z doing→done — verify exit 0 @ 2ca6fdd+1
