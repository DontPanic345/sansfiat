# 354 judge: not shown

Source: `main-sweep.json`, `branch-sweep.json` (with floor), `compare.txt` (`judge --compare`). Engine unchanged since those sweeps; the review only changed one view string.

| Score | Old | New | Floor | Read |
|---|---|---|---|---|
| Question: hungry army-turns outside the capital's ring | 583 of 1,654 (35%) | 522 of 1,531 (34%) | 71 | inside floor |
| Hungry outside every store's ring | 583 | 513 | 48 | past floor, but the old ring had no settlements |
| Warriors starved / armies lost | 221 / 5 | 198 / 2 | 6 / 1 | past floor |
| Standing, top civ at +200 | 23.2% | 22.8% | 1.2 | inside |
| Doubt: lead changes | 10 | 11 | 0 | past, by one |
| Doubt: spread at 200 | 0.172 | 0.182 | 0.005 | past |
| Decisions: AI orders per 100 | 75.4 | 75.7 | 2.7 | inside |
| Motion: land changed / clans changed | 466 / 258 | 706 / 368 | 71 / 45 | past (settlements taken 0 to 18, all on small 2) |
| Led standing | 14.3% | 13.4% | 0.4 | past, down |
| Led over idle | 4.3 | 4.1 | 1.1 | inside |
| Health (panics, digest mismatches) | 0 | 0 | 0 | in band |

The rule fires only where a settlement stands near a front: small 2 (hungry far from the capital 32% to 26%, 61 army-turns fed from a settlement's ring). On the other six maps no army stood in a settlement's ring, so the headline question moves on one map and not past the floor overall. The motion is mostly one map's settlements changing hands.
