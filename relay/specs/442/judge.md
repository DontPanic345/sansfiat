# #442 review: the judge

No sweep was run: the piece changes no rule. `git diff origin/main...HEAD --stat -- engine/core engine/cli engine/web`
is empty, so every sweep, digest and save is main's. `SAVE_VERSION` stays 27.

| Score | Main | Branch | Floor | Note |
|---|---|---|---|---|
| Standing | main's | the same | - | no engine change |
| Doubt | main's | the same | - | no engine change |
| Decisions | main's | the same | - | no engine change |
| Motion | main's | the same | - | no engine change |
| Health | main's | the same | - | digests match on all 48 views (build-3's `--diff`); medium 9 +8 turns `e5fc275e001a30e2` on both (`proto/fields.txt` on main, the review's run on the branch) |

Verdict: **same**. The piece's question is the art, judged on the views in `views.md`.

## The review's own checks

- The field clearing on the real game, medium 9 after 8 turns led (`/tmp` script, the same loop as `shots.mjs`): 32 plot
  tiles, 16 on forest or marsh; `view.woodsKey` equals those 16; the woods go from 1236 pieces (no fields) to 1209, and
  none stands in the clearing of a field tile. Per field tile, 2 to 5 pieces before and 0 to 3 after.
- Read the screenshots in `shots/` beside main's: the medium 9 wood reads as wood (dense crowns inside, scattered at the
  margins); the small 9 river runs clear through the trees; the marsh (crop of `small2-lone-close`) is straw reeds and
  blue pools with no tan bank on the sea. The pools are blocky up close and the lone hex is a small clump (build-3's notes).
- The furrows don't show in the fields captures, on main or the branch: the field tiles there sit under the home icon,
  the floodplain and the peaks. The clearing is proven by the check above and by `check-map.mjs`, not by the picture.
