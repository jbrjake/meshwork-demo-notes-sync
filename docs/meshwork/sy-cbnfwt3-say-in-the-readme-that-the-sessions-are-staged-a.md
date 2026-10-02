---
id: sy-cbnfwt3
title: Say in the README that the sessions are staged and how to replay them
status: open
category: docs
verify: "all(exists README.md, contains README.md /story\\/replay\\.sh/)"
created: 2026-10-02T17:21Z
---

This repo is part of a demo, and the README says so up front. The agent sessions in its history after day 0 are staged: a script performs them with real meshwork commands, real code changes and real outputs. The README names the script, `story/replay.sh` in meshwork-demo-notes-portfolio, and how to run it.

This lands at day 0 because re-recording the story resets `main` to the `story/0-day0` tag, which would drop a README commit made after it.

## log
- 2026-10-02T17:21Z created
