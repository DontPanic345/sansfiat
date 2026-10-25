---
name: proposal-deck
description: Narrated slide deck that proposes how an idea for a unit, mechanic or piece gets built, for the owner to watch and approve. Use when asked for a proposal, or to present a plan as slides.
---

# Proposal deck

The owner brings an idea. You turn it into a proposal they can watch: a short deck that a British voice talks through, with each point appearing on screen as it's said. It ends on the decisions they need to make and the issue body Relay will read, so approving the deck is approving the piece.

## 1. Work out the proposal

Read the idea's notes, the `docs/design/` files it touches, `docs/proposals/` for where the plan stands, and `relay/README.md` for what a piece's issue body holds. Then settle how it gets built on the engine: what changes, the numbers, where it sits in the plan, and what's out of scope. Each open question becomes a call you make, with its reason. Note every figure you picked that the owner didn't give; the deck and your reply both say so.

## 2. Write the deck

Copy `template.html` to `index.html` in a fresh folder in your scratchpad. It's a working player with example slides; replace the title, the heading, the slides and the narration.

A run of about ten slides plays in six minutes or so: the idea, where it sits, a slide per part of the mechanic, what the player sees and clicks, the decisions, the issue body, and what happens on a go. A diagram is worth drawing wherever the mechanic is spatial.

Each slide is a `<section class="slide" data-id="…">`. Its elements carry `data-step`, and the narration JSON at the bottom holds a list of lines per slide id: line N is spoken as step N appears, so a slide has one line per step. Slide text stays short and the voice carries the explanation. Write the lines to be heard: short sentences, and numbers as they'd be said ("piece twenty-nine").

## 3. Record and publish

```powershell
.claude/skills/proposal-deck/make-audio.ps1 <folder>
```

It records every line with `edge-tts` (through `uvx`, so the text goes to Microsoft's speech service), warns where a slide's steps and lines don't match, and prints the `files` map.

Watch it before the owner does. `check.mjs` plays the deck in local Chrome: copy it into a scratch folder with `playwright-core` installed and run `node check.mjs <folder>`. It screenshots every slide to `<folder>/shots` and lists script errors, audio that didn't load, and slides that flash their builds on entry. Look at the shots for clipped or crowded slides.

Publish `index.html` as an artifact with that map as `files` and the folder as `root`. The page plays its own audio; the Slides artifact type can't. Publishing the same path again keeps the link.

Give the owner the link, the calls you made and the figures you picked. Once they say go, the design file and the issue body follow from the deck.
