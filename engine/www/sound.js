// The sound bus: each feed event kind names a file in assets/sounds.json, and the bus plays it when the
// feed hears that kind. A kind's file is tried once, when it is first heard unmuted; a miss makes that
// kind silent from then on, with no error. No sound files ship. Sound waits for the player's first
// click, as browsers require. `heard` counts every event heard by kind, muted or not, so a script can
// see the bus ran.

export class Sound {
  /** @param {Record<string, string>} files each feed kind to its file name (sounds.json) */
  constructor(files, base = './assets/sounds/') {
    this.files = files;
    this.base = base;
    this.heard = {};
    this.muted = true;
    this.clips = new Map(); // kind -> an Audio, or null for a miss
    document.addEventListener('pointerdown', () => (this.muted = false), { once: true });
  }

  /** A new batch of feed events (feed.js's form). */
  hear(events) {
    for (const e of events) {
      this.heard[e.kind] = (this.heard[e.kind] ?? 0) + 1;
      if (!this.muted) this.play(e.kind);
    }
  }

  play(kind) {
    if (!this.clips.has(kind)) this.load(kind);
    const clip = this.clips.get(kind);
    if (clip) clip.play().catch(() => {});
  }

  /** Tries the kind's file once: a clip if it loads, null if not. */
  load(kind) {
    const file = this.files[kind];
    this.clips.set(kind, null);
    if (!file) return;
    const url = `${this.base}${file}`;
    fetch(url, { method: 'HEAD' })
      .then((r) => {
        if (r.ok) this.clips.set(kind, new Audio(url));
      })
      .catch(() => {});
  }
}
