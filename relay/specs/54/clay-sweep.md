# Clay sweep: #54 build-1

`cargo build --release -p sansfiat && ./target/release/sansfiat sweep --size all` (seeds 1–10
on each size, `--every 50`) after build-1, and `sansfiat bench --size small|medium|large`
(seeds 1–3, 200 turns). The numbers are starting numbers that run, not tuned (build-5 tunes
them).

## How it runs

- **Every seed ends, all at the turn limit** (turn 499): 27 of 30 by the limit and the top-up,
  3 large seeds on the limit with three centres already but the cover short. None ends on its
  own (main: 8/10 small, 9/10 medium, 10/10 large). Medium seed 10 ends with two centres: the
  top-up found no room for a third in its region.
- **Clans are fewer**: 9–45 on small (main 87–149), 7–112 on medium, 15–126 on large. Each
  holds a small territory (2–3 tiles on average), so cover stays low (0.07–0.80, mostly
  under the 0.8 the end needs). Markets still form (3–12 a map), on homes with four clans in
  reach as before; few reach `centre_clans` traders with a hearth before the limit.
- **People level off** on 24 of 30 seeds (change under 10% over the last 100 turns): 9/10
  small, 7/10 medium, 8/10 large; the rest are still growing (+10% to +26%).
- **Speed**: 0.1–0.7 ms a turn on small, 0.2–1.5 on medium, 0.4–2.0 on large in the sweep
  (main: 3–5, 5–9, 8–14). Per clan, a turn costs about as much or less than before: the
  add/drop rule re-divides labour for each candidate over a territory of a few tiles (a scan
  Dijkstra over the territory and one pass over its tiles), and the go step runs one bounded
  `cost_map` per clan only when a festival is within `reach_walk / 2` hex steps.

## Bench

```
small:
seed 1: world 7.5 ms, 200 turns 18.9 ms (0.095 ms a turn)
seed 2: world 7.2 ms, 200 turns 12.4 ms (0.062 ms a turn)
seed 3: world 11.2 ms, 200 turns 30.0 ms (0.150 ms a turn)
medium:
seed 1: world 12.5 ms, 200 turns 50.7 ms (0.254 ms a turn)
seed 2: world 17.8 ms, 200 turns 28.2 ms (0.141 ms a turn)
seed 3: world 35.7 ms, 200 turns 7.6 ms (0.038 ms a turn)
large:
seed 1: world 57.8 ms, 200 turns 69.5 ms (0.348 ms a turn)
seed 2: world 81.8 ms, 200 turns 30.3 ms (0.151 ms a turn)
seed 3: world 75.0 ms, 200 turns 112.9 ms (0.565 ms a turn)
```

## Sweep

```
small  seed   1: ended 499 by limit+topup centres 3 topped up 3 (1/1/1)  cover 0.44  clans  21  people  177.8  markets  4  hearths  16  closed 1/1/2  0.3 ms a turn
small           turn    0  clans   3  people   18.0
small           turn   50  clans   5  people   41.2
small           turn  100  clans  10  people   85.5
small           turn  150  clans  14  people  130.8
small           turn  200  clans  20  people  153.7
small           turn  250  clans  20  people  163.2
small           turn  300  clans  20  people  169.6
small           turn  350  clans  20  people  173.0
small           turn  400  clans  20  people  175.8
small           turn  450  clans  21  people  179.1
small           turn  500  clans  21  people  177.8
small  seed   2: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.61  clans  38  people  291.8  markets  5  hearths  28  closed 2/1/2  0.4 ms a turn
small           turn    0  clans   3  people   18.0
small           turn   50  clans   7  people   47.9
small           turn  100  clans   7  people   62.1
small           turn  150  clans   9  people   69.1
small           turn  200  clans  11  people   77.5
small           turn  250  clans  18  people  152.5
small           turn  300  clans  24  people  192.3
small           turn  350  clans  30  people  240.2
small           turn  400  clans  33  people  268.3
small           turn  450  clans  36  people  281.7
small           turn  500  clans  38  people  291.8
small  seed   3: ended 499 by limit+topup centres 3 topped up 2 (1/1/0)  cover 0.55  clans  27  people  211.9  markets  4  hearths  24  closed 1/1/1  0.4 ms a turn
small           turn    0  clans   3  people   18.0
small           turn   50  clans   8  people   74.6
small           turn  100  clans  15  people  116.9
small           turn  150  clans  18  people  145.6
small           turn  200  clans  22  people  173.3
small           turn  250  clans  24  people  199.4
small           turn  300  clans  27  people  207.1
small           turn  350  clans  27  people  213.6
small           turn  400  clans  27  people  214.0
small           turn  450  clans  27  people  214.4
small           turn  500  clans  27  people  211.9
small  seed   4: ended 499 by limit+topup centres 3 topped up 2 (0/1/1)  cover 0.55  clans  22  people  185.9  markets  4  hearths  23  closed 2/1/1  0.3 ms a turn
small           turn    0  clans   3  people   18.0
small           turn   50  clans   6  people   47.0
small           turn  100  clans   7  people   56.6
small           turn  150  clans  11  people   86.0
small           turn  200  clans  12  people  108.2
small           turn  250  clans  15  people  123.3
small           turn  300  clans  20  people  158.0
small           turn  350  clans  21  people  166.0
small           turn  400  clans  21  people  173.2
small           turn  450  clans  21  people  179.9
small           turn  500  clans  22  people  185.9
small  seed   5: ended 499 by limit+topup centres 3 topped up 3 (0/1/2)  cover 0.28  clans   9  people   79.0  markets  5  hearths  10  closed 2/1/1  0.2 ms a turn
small           turn    0  clans   3  people   18.0
small           turn   50  clans   5  people   49.6
small           turn  100  clans   7  people   69.3
small           turn  150  clans   9  people   80.7
small           turn  200  clans   9  people   82.0
small           turn  250  clans   9  people   82.0
small           turn  300  clans   9  people   82.0
small           turn  350  clans   9  people   82.0
small           turn  400  clans   9  people   82.0
small           turn  450  clans   9  people   82.0
small           turn  500  clans   9  people   79.0
small  seed   6: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.72  clans  44  people  363.8  markets  4  hearths  86  closed 1/1/2  0.7 ms a turn
small           turn    0  clans   3  people   18.0
small           turn   50  clans  13  people  110.5
small           turn  100  clans  20  people  171.1
small           turn  150  clans  28  people  237.7
small           turn  200  clans  38  people  313.5
small           turn  250  clans  39  people  335.6
small           turn  300  clans  44  people  359.1
small           turn  350  clans  44  people  364.4
small           turn  400  clans  44  people  366.1
small           turn  450  clans  44  people  366.8
small           turn  500  clans  44  people  363.8
small  seed   7: ended 499 by limit+topup centres 3 topped up 3 (1/0/2)  cover 0.25  clans  21  people  146.8  markets  4  hearths  22  closed 2/1/2  0.2 ms a turn
small           turn    0  clans   3  people   18.0
small           turn   50  clans   5  people   40.4
small           turn  100  clans   7  people   58.4
small           turn  150  clans   8  people   63.4
small           turn  200  clans  12  people   97.7
small           turn  250  clans  14  people  112.8
small           turn  300  clans  18  people  132.8
small           turn  350  clans  19  people  143.0
small           turn  400  clans  21  people  148.5
small           turn  450  clans  21  people  149.8
small           turn  500  clans  21  people  146.8
small  seed   8: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.61  clans  45  people  335.6  markets  4  hearths  50  closed 2/1/2  0.5 ms a turn
small           turn    0  clans   3  people   18.0
small           turn   50  clans   6  people   43.3
small           turn  100  clans  14  people  101.6
small           turn  150  clans  19  people  145.4
small           turn  200  clans  25  people  183.2
small           turn  250  clans  28  people  203.7
small           turn  300  clans  31  people  248.3
small           turn  350  clans  37  people  291.0
small           turn  400  clans  43  people  317.4
small           turn  450  clans  44  people  332.1
small           turn  500  clans  45  people  335.6
small  seed   9: ended 499 by limit+topup centres 3 topped up 3 (1/2/0)  cover 0.45  clans  15  people  123.7  markets  5  hearths  29  closed 2/1/1  0.2 ms a turn
small           turn    0  clans   3  people   18.0
small           turn   50  clans   5  people   47.1
small           turn  100  clans   7  people   62.4
small           turn  150  clans  10  people   82.0
small           turn  200  clans  12  people   95.5
small           turn  250  clans  12  people  107.6
small           turn  300  clans  14  people  116.3
small           turn  350  clans  14  people  121.4
small           turn  400  clans  15  people  124.6
small           turn  450  clans  15  people  126.5
small           turn  500  clans  15  people  123.7
small  seed  10: ended 499 by limit+topup centres 3 topped up 3 (1/1/1)  cover 0.43  clans  16  people  115.0  markets  4  hearths  18  closed 1/1/2  0.1 ms a turn
small           turn    0  clans   3  people   18.0
small           turn   50  clans   7  people   49.2
small           turn  100  clans   8  people   68.8
small           turn  150  clans  10  people   79.8
small           turn  200  clans  11  people   83.4
small           turn  250  clans  12  people   86.7
small           turn  300  clans  12  people   89.6
small           turn  350  clans  13  people   93.4
small           turn  400  clans  13  people  103.1
small           turn  450  clans  16  people  112.2
small           turn  500  clans  16  people  115.0
small: the top-up took 10 existing markets, 7 tended tiles, 7 other grain tiles
small: 0 of 10 ended on their own, 10 needed the top-up, 0 ended on the limit with three centres already (cover short), median end turn 499
small: people over the last 100 turns before the end changed by +1% +9% -1% +7% -4% -1% -1% +6% -1% +12%: 9 of 10 levelled off (under 10%)
medium seed   1: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.33  clans  29  people  239.9  markets  4  hearths  37  closed 4/2/4  0.7 ms a turn
medium          turn    0  clans   3  people   18.0
medium          turn   50  clans  10  people   89.9
medium          turn  100  clans  20  people  157.1
medium          turn  150  clans  24  people  195.3
medium          turn  200  clans  24  people  209.8
medium          turn  250  clans  25  people  215.0
medium          turn  300  clans  25  people  220.1
medium          turn  350  clans  26  people  221.1
medium          turn  400  clans  26  people  223.9
medium          turn  450  clans  26  people  226.3
medium          turn  500  clans  29  people  239.9
medium seed   2: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.52  clans  47  people  371.6  markets  7  hearths  54  closed 4/2/4  0.7 ms a turn
medium          turn    0  clans   3  people   18.0
medium          turn   50  clans   7  people   63.8
medium          turn  100  clans  12  people  102.4
medium          turn  150  clans  17  people  138.0
medium          turn  200  clans  20  people  164.3
medium          turn  250  clans  24  people  194.8
medium          turn  300  clans  31  people  249.3
medium          turn  350  clans  34  people  266.8
medium          turn  400  clans  36  people  296.0
medium          turn  450  clans  43  people  339.9
medium          turn  500  clans  47  people  371.6
medium seed   3: ended 499 by limit+topup centres 3 topped up 3 (0/1/2)  cover 0.08  clans   7  people   49.2  markets  3  hearths   5  closed 4/2/3  0.5 ms a turn
medium          turn    0  clans   3  people   18.0
medium          turn   50  clans   4  people   32.9
medium          turn  100  clans   5  people   40.3
medium          turn  150  clans   7  people   50.6
medium          turn  200  clans   7  people   51.3
medium          turn  250  clans   7  people   51.9
medium          turn  300  clans   7  people   52.2
medium          turn  350  clans   7  people   52.2
medium          turn  400  clans   7  people   52.2
medium          turn  450  clans   7  people   52.2
medium          turn  500  clans   7  people   49.2
medium seed   4: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.55  clans  74  people  581.5  markets  7  hearths  97  closed 4/2/4  1.3 ms a turn
medium          turn    0  clans   3  people   18.0
medium          turn   50  clans   9  people   71.4
medium          turn  100  clans  18  people  139.4
medium          turn  150  clans  26  people  218.3
medium          turn  200  clans  37  people  288.8
medium          turn  250  clans  47  people  372.7
medium          turn  300  clans  58  people  448.2
medium          turn  350  clans  63  people  497.4
medium          turn  400  clans  70  people  526.3
medium          turn  450  clans  72  people  555.2
medium          turn  500  clans  74  people  581.5
medium seed   5: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.74  clans 112  people  902.5  markets 10  hearths 157  closed 4/2/3  1.5 ms a turn
medium          turn    0  clans   3  people   18.0
medium          turn   50  clans   6  people   50.3
medium          turn  100  clans  12  people  105.3
medium          turn  150  clans  23  people  180.6
medium          turn  200  clans  35  people  274.9
medium          turn  250  clans  53  people  429.4
medium          turn  300  clans  71  people  570.3
medium          turn  350  clans  96  people  761.8
medium          turn  400  clans 105  people  837.9
medium          turn  450  clans 110  people  884.4
medium          turn  500  clans 112  people  902.5
medium seed   6: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.51  clans  57  people  464.4  markets  7  hearths  74  closed 4/2/3  0.7 ms a turn
medium          turn    0  clans   3  people   18.0
medium          turn   50  clans  10  people   87.0
medium          turn  100  clans  16  people  124.4
medium          turn  150  clans  19  people  154.7
medium          turn  200  clans  23  people  185.0
medium          turn  250  clans  34  people  270.1
medium          turn  300  clans  37  people  311.2
medium          turn  350  clans  43  people  347.1
medium          turn  400  clans  44  people  377.6
medium          turn  450  clans  49  people  413.2
medium          turn  500  clans  57  people  464.4
medium seed   7: ended 499 by limit+topup centres 3 topped up 3 (3/0/0)  cover 0.35  clans  27  people  240.0  markets  5  hearths  33  closed 4/2/4  0.5 ms a turn
medium          turn    0  clans   3  people   18.0
medium          turn   50  clans  11  people   91.3
medium          turn  100  clans  22  people  171.8
medium          turn  150  clans  25  people  199.8
medium          turn  200  clans  25  people  215.6
medium          turn  250  clans  26  people  222.9
medium          turn  300  clans  26  people  231.0
medium          turn  350  clans  27  people  233.4
medium          turn  400  clans  27  people  239.5
medium          turn  450  clans  27  people  241.3
medium          turn  500  clans  27  people  240.0
medium seed   8: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.51  clans  58  people  447.0  markets  7  hearths  50  closed 4/2/3  0.6 ms a turn
medium          turn    0  clans   3  people   18.0
medium          turn   50  clans  12  people  104.5
medium          turn  100  clans  22  people  181.6
medium          turn  150  clans  30  people  227.0
medium          turn  200  clans  43  people  322.8
medium          turn  250  clans  48  people  358.2
medium          turn  300  clans  51  people  383.4
medium          turn  350  clans  55  people  414.4
medium          turn  400  clans  57  people  429.9
medium          turn  450  clans  58  people  446.3
medium          turn  500  clans  58  people  447.0
medium seed   9: ended 499 by limit+topup centres 3 topped up 3 (1/2/0)  cover 0.20  clans  11  people   97.6  markets  3  hearths   3  closed 4/2/3  0.2 ms a turn
medium          turn    0  clans   3  people   18.0
medium          turn   50  clans   6  people   50.3
medium          turn  100  clans   7  people   63.5
medium          turn  150  clans   8  people   71.1
medium          turn  200  clans  11  people   94.2
medium          turn  250  clans  11  people  100.4
medium          turn  300  clans  11  people  100.5
medium          turn  350  clans  11  people  100.6
medium          turn  400  clans  11  people  100.6
medium          turn  450  clans  11  people  100.6
medium          turn  500  clans  11  people   97.6
medium seed  10: ended 499 by limit+topup centres 2 topped up 1 (0/1/0)  cover 0.80  clans  27  people  220.1  markets  6  hearths  22  closed 4/2/4  0.3 ms a turn
medium          turn    0  clans   3  people   18.0
medium          turn   50  clans  10  people   74.0
medium          turn  100  clans  14  people  120.7
medium          turn  150  clans  16  people  137.3
medium          turn  200  clans  17  people  147.1
medium          turn  250  clans  19  people  159.4
medium          turn  300  clans  20  people  172.3
medium          turn  350  clans  23  people  197.8
medium          turn  400  clans  25  people  206.3
medium          turn  450  clans  27  people  213.8
medium          turn  500  clans  27  people  220.1
medium: the top-up took 12 existing markets, 4 tended tiles, 2 other grain tiles
medium: 0 of 10 ended on their own, 10 needed the top-up, 0 ended on the limit with three centres already (cover short), 1 with fewer than three centres, median end turn 499
medium: people over the last 100 turns before the end changed by +7% +26% -6% +10% +8% +23% +0% +4% -3% +7%: 7 of 10 levelled off (under 10%)
large  seed   1: ended 499 by limit+topup centres 3 topped up 2 (1/1/0)  cover 0.22  clans  47  people  387.4  markets  6  hearths  81  closed 8/4/8  1.0 ms a turn
large           turn    0  clans   3  people   18.0
large           turn   50  clans  11  people   96.9
large           turn  100  clans  22  people  189.9
large           turn  150  clans  31  people  263.2
large           turn  200  clans  33  people  286.0
large           turn  250  clans  37  people  308.0
large           turn  300  clans  42  people  348.6
large           turn  350  clans  43  people  362.2
large           turn  400  clans  45  people  367.8
large           turn  450  clans  47  people  387.1
large           turn  500  clans  47  people  387.4
large  seed   2: ended 499 by limit+topup centres 3 topped up 2 (1/1/0)  cover 0.17  clans  38  people  281.8  markets  5  hearths  44  closed 8/4/8  0.9 ms a turn
large           turn    0  clans   3  people   18.0
large           turn   50  clans   7  people   57.7
large           turn  100  clans  11  people   91.4
large           turn  150  clans  18  people  138.5
large           turn  200  clans  30  people  227.0
large           turn  250  clans  32  people  247.6
large           turn  300  clans  33  people  257.6
large           turn  350  clans  36  people  265.7
large           turn  400  clans  35  people  271.6
large           turn  450  clans  38  people  283.8
large           turn  500  clans  38  people  281.8
large  seed   3: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.19  clans  58  people  458.3  markets  6  hearths  73  closed 8/4/8  1.3 ms a turn
large           turn    0  clans   3  people   18.0
large           turn   50  clans  17  people  133.8
large           turn  100  clans  30  people  244.3
large           turn  150  clans  38  people  289.4
large           turn  200  clans  43  people  340.3
large           turn  250  clans  49  people  371.5
large           turn  300  clans  53  people  412.9
large           turn  350  clans  55  people  433.7
large           turn  400  clans  56  people  443.7
large           turn  450  clans  57  people  456.1
large           turn  500  clans  58  people  458.3
large  seed   4: ended 499 by limit+topup centres 3 topped up 1 (0/1/0)  cover 0.20  clans  65  people  537.0  markets  7  hearths 113  closed 8/4/8  1.5 ms a turn
large           turn    0  clans   3  people   18.0
large           turn   50  clans   7  people   61.6
large           turn  100  clans  22  people  180.5
large           turn  150  clans  37  people  304.3
large           turn  200  clans  47  people  358.9
large           turn  250  clans  50  people  408.2
large           turn  300  clans  58  people  459.4
large           turn  350  clans  62  people  497.7
large           turn  400  clans  64  people  513.1
large           turn  450  clans  64  people  529.2
large           turn  500  clans  65  people  537.0
large  seed   5: ended 499 by limit+topup centres 3 topped up 3 (3/0/0)  cover 0.21  clans  41  people  326.4  markets  6  hearths  38  closed 8/4/8  0.8 ms a turn
large           turn    0  clans   3  people   18.0
large           turn   50  clans  12  people   94.7
large           turn  100  clans  16  people  143.1
large           turn  150  clans  22  people  171.4
large           turn  200  clans  27  people  193.6
large           turn  250  clans  29  people  225.9
large           turn  300  clans  34  people  259.3
large           turn  350  clans  36  people  282.6
large           turn  400  clans  39  people  305.5
large           turn  450  clans  41  people  321.8
large           turn  500  clans  41  people  326.4
large  seed   6: ended 499 by limit+topup centres 3 topped up 3 (1/1/1)  cover 0.07  clans  15  people  120.6  markets  4  hearths  31  closed 8/4/8  0.4 ms a turn
large           turn    0  clans   3  people   18.0
large           turn   50  clans   8  people   71.6
large           turn  100  clans  10  people   81.6
large           turn  150  clans  12  people   89.8
large           turn  200  clans  13  people  104.4
large           turn  250  clans  15  people  118.9
large           turn  300  clans  15  people  122.4
large           turn  350  clans  15  people  123.6
large           turn  400  clans  15  people  123.6
large           turn  450  clans  15  people  123.6
large           turn  500  clans  15  people  120.6
large  seed   7: ended 499 by limit       centres 3 topped up 0 (0/0/0)  cover 0.33  clans  94  people  744.1  markets 10  hearths 113  closed 8/4/8  1.5 ms a turn
large           turn    0  clans   3  people   18.0
large           turn   50  clans  17  people  126.7
large           turn  100  clans  27  people  218.8
large           turn  150  clans  39  people  313.9
large           turn  200  clans  52  people  408.1
large           turn  250  clans  56  people  450.6
large           turn  300  clans  70  people  532.8
large           turn  350  clans  78  people  586.0
large           turn  400  clans  84  people  640.6
large           turn  450  clans  91  people  696.1
large           turn  500  clans  94  people  744.1
large  seed   8: ended 499 by limit       centres 3 topped up 0 (0/0/0)  cover 0.37  clans 126  people  988.2  markets 12  hearths 160  closed 8/4/8  2.0 ms a turn
large           turn    0  clans   3  people   18.0
large           turn   50  clans  20  people  155.1
large           turn  100  clans  42  people  327.3
large           turn  150  clans  66  people  510.9
large           turn  200  clans  77  people  605.5
large           turn  250  clans  86  people  683.8
large           turn  300  clans  98  people  777.4
large           turn  350  clans 112  people  839.4
large           turn  400  clans 122  people  928.3
large           turn  450  clans 124  people  963.9
large           turn  500  clans 126  people  988.2
large  seed   9: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.27  clans  56  people  444.4  markets  9  hearths  73  closed 8/4/8  1.2 ms a turn
large           turn    0  clans   3  people   18.0
large           turn   50  clans  14  people  102.5
large           turn  100  clans  24  people  203.8
large           turn  150  clans  34  people  271.6
large           turn  200  clans  42  people  341.7
large           turn  250  clans  47  people  376.7
large           turn  300  clans  52  people  409.0
large           turn  350  clans  53  people  421.6
large           turn  400  clans  54  people  435.3
large           turn  450  clans  55  people  440.1
large           turn  500  clans  56  people  444.4
large  seed  10: ended 499 by limit       centres 3 topped up 0 (0/0/0)  cover 0.21  clans  65  people  538.8  markets  9  hearths  65  closed 8/4/8  1.0 ms a turn
large           turn    0  clans   3  people   18.0
large           turn   50  clans  11  people   89.0
large           turn  100  clans  21  people  175.5
large           turn  150  clans  32  people  246.2
large           turn  200  clans  38  people  303.8
large           turn  250  clans  40  people  329.7
large           turn  300  clans  42  people  357.7
large           turn  350  clans  48  people  420.4
large           turn  400  clans  57  people  478.6
large           turn  450  clans  63  people  521.4
large           turn  500  clans  65  people  538.8
large: the top-up took 9 existing markets, 4 tended tiles, 1 other grain tiles
large: 0 of 10 ended on their own, 7 needed the top-up, 3 ended on the limit with three centres already (cover short), median end turn 499
large: people over the last 100 turns before the end changed by +5% +4% +3% +5% +7% -2% +16% +6% +2% +13%: 8 of 10 levelled off (under 10%)
```
