# Land sweep: #54 build-2

`cargo build --release -p sansfiat && ./target/release/sansfiat sweep --size all --every 50`
(seeds 1–10 on each size) after build-2 (depletion, trails from walking, open-ended taming).
The numbers are the starting numbers build-1 and build-2 pick, not tuned (build-5 tunes them).
Lines with `depleted` count the tiles where some food's wear is at least `DEPLETION.depleted_at`.

## How it runs

- **No seed ends on its own.** Every seed on every size reaches the turn limit (turn 499) and
  the top-up: small 10 of 10, medium 10 of 10, large 10 of 10 (the clay, build-1: the same, 0 of
  10 on their own on small and medium and large). Medium seed 10 ends with two centres, as the clay did.
- **Depletion is common.** At the end, small maps have 17–55 depleted tiles, medium 11–193,
  large 44–320. Depleted tiles roughly track the clan count (large 8: 162 clans, 320 tiles; medium 5:
  103 clans, 164 tiles; medium 3: 6 clans, 11 tiles).
- **People levelled off less.** Change in people over the last 100 turns before the end, under
  10% counts as levelled: small 7 of 10 (clay: 9 of 10), medium 4 of 10 (clay: 7), large 1 of
  10 (clay: 8). Large seeds grow 6–39% over those turns. Build-2 adds two rules that can raise
  food (grain past a hearth, now with no ceiling, and trails that shorten walks); I have not
  isolated which one moves people, and the numbers are untuned. build-5 settles it.
- **Speed.** Ms a turn on the sweep: small 0.3–1.0 (clay 0.1–0.7), medium 0.4–1.9 (clay
  0.2–1.5), large 1.3–3.0 (clay 0.4–2.0). Within 2× of main's 3–5, 5–9, 8–14 ms.

## Output

```
small  seed   1: ended 499 by limit+topup centres 3 topped up 3 (1/0/2)  cover 0.49  clans  26  people  192.5  depleted   55  markets  5  hearths  16  closed 1/1/2  0.4 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0
small           turn   50  clans   4  people   33.3  depleted   32
small           turn  100  clans   6  people   53.4  depleted   37
small           turn  150  clans   9  people   76.9  depleted   61
small           turn  200  clans  13  people  105.6  depleted   67
small           turn  250  clans  15  people  124.8  depleted   74
small           turn  300  clans  19  people  145.4  depleted   74
small           turn  350  clans  21  people  163.6  depleted   76
small           turn  400  clans  22  people  172.8  depleted   58
small           turn  450  clans  25  people  191.6  depleted   55
small           turn  500  clans  26  people  192.5  depleted   55
small  seed   2: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.45  clans  16  people  123.7  depleted   30  markets  3  hearths  16  closed 2/1/2  0.4 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0
small           turn   50  clans   4  people   33.8  depleted   21
small           turn  100  clans   5  people   40.0  depleted   18
small           turn  150  clans   6  people   45.7  depleted   16
small           turn  200  clans   6  people   47.3  depleted   12
small           turn  250  clans   8  people   56.9  depleted   25
small           turn  300  clans   9  people   70.9  depleted   21
small           turn  350  clans  11  people   90.6  depleted   34
small           turn  400  clans  13  people   97.7  depleted   34
small           turn  450  clans  16  people  113.0  depleted   28
small           turn  500  clans  16  people  123.7  depleted   30
small  seed   3: ended 499 by limit+topup centres 3 topped up 3 (2/1/0)  cover 0.56  clans  24  people  200.2  depleted   45  markets  4  hearths  20  closed 1/1/1  0.6 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0
small           turn   50  clans   6  people   51.8  depleted   33
small           turn  100  clans  10  people   84.1  depleted   40
small           turn  150  clans  12  people  106.1  depleted   52
small           turn  200  clans  14  people  128.9  depleted   52
small           turn  250  clans  18  people  142.9  depleted   50
small           turn  300  clans  19  people  150.1  depleted   45
small           turn  350  clans  20  people  171.5  depleted   46
small           turn  400  clans  21  people  181.1  depleted   54
small           turn  450  clans  24  people  193.4  depleted   47
small           turn  500  clans  24  people  200.2  depleted   45
small  seed   4: ended 499 by limit+topup centres 3 topped up 2 (1/0/1)  cover 0.51  clans  26  people  183.4  depleted   23  markets  3  hearths  24  closed 2/1/1  0.5 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0
small           turn   50  clans   4  people   39.6  depleted   34
small           turn  100  clans   7  people   53.8  depleted   40
small           turn  150  clans  11  people   88.4  depleted   41
small           turn  200  clans  13  people  104.7  depleted   36
small           turn  250  clans  16  people  125.3  depleted   40
small           turn  300  clans  17  people  141.0  depleted   33
small           turn  350  clans  21  people  159.2  depleted   28
small           turn  400  clans  22  people  168.2  depleted   24
small           turn  450  clans  23  people  173.1  depleted   22
small           turn  500  clans  26  people  183.4  depleted   23
small  seed   5: ended 499 by limit+topup centres 3 topped up 3 (0/1/2)  cover 0.31  clans  10  people   85.1  depleted   22  markets  4  hearths  10  closed 2/1/1  0.3 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0
small           turn   50  clans   4  people   33.3  depleted   31
small           turn  100  clans   5  people   39.3  depleted   31
small           turn  150  clans   6  people   51.9  depleted   39
small           turn  200  clans   7  people   62.0  depleted   38
small           turn  250  clans   7  people   66.8  depleted   23
small           turn  300  clans   9  people   78.4  depleted   26
small           turn  350  clans  10  people   82.6  depleted   28
small           turn  400  clans  10  people   84.6  depleted   20
small           turn  450  clans  10  people   86.6  depleted   18
small           turn  500  clans  10  people   85.1  depleted   22
small  seed   6: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.78  clans  49  people  369.5  depleted   42  markets  5  hearths  85  closed 1/1/2  1.0 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0
small           turn   50  clans  11  people   76.2  depleted   76
small           turn  100  clans  17  people  129.0  depleted  101
small           turn  150  clans  23  people  189.0  depleted   89
small           turn  200  clans  30  people  239.6  depleted  122
small           turn  250  clans  40  people  291.5  depleted  115
small           turn  300  clans  44  people  313.1  depleted   99
small           turn  350  clans  45  people  338.0  depleted   68
small           turn  400  clans  46  people  356.4  depleted   60
small           turn  450  clans  48  people  368.3  depleted   41
small           turn  500  clans  49  people  369.5  depleted   42
small  seed   7: ended 499 by limit+topup centres 3 topped up 3 (1/0/2)  cover 0.23  clans  18  people  141.1  depleted   17  markets  5  hearths  23  closed 2/1/2  0.3 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0
small           turn   50  clans   5  people   33.8  depleted   22
small           turn  100  clans   6  people   48.9  depleted   27
small           turn  150  clans   9  people   66.2  depleted   34
small           turn  200  clans   9  people   77.4  depleted   37
small           turn  250  clans  13  people  104.4  depleted   54
small           turn  300  clans  16  people  120.1  depleted   56
small           turn  350  clans  16  people  128.8  depleted   44
small           turn  400  clans  17  people  137.8  depleted   26
small           turn  450  clans  18  people  140.0  depleted   20
small           turn  500  clans  18  people  141.1  depleted   17
small  seed   8: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.60  clans  42  people  304.3  depleted   45  markets  4  hearths  50  closed 2/1/2  0.6 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0
small           turn   50  clans   7  people   51.5  depleted   30
small           turn  100  clans   9  people   76.6  depleted   26
small           turn  150  clans  14  people  105.5  depleted   40
small           turn  200  clans  20  people  151.6  depleted   48
small           turn  250  clans  26  people  190.2  depleted   58
small           turn  300  clans  32  people  224.5  depleted   71
small           turn  350  clans  35  people  259.4  depleted   72
small           turn  400  clans  38  people  285.3  depleted   65
small           turn  450  clans  40  people  297.9  depleted   51
small           turn  500  clans  42  people  304.3  depleted   45
small  seed   9: ended 499 by limit+topup centres 3 topped up 3 (1/2/0)  cover 0.46  clans  18  people  135.4  depleted   22  markets  3  hearths  28  closed 2/1/1  0.4 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0
small           turn   50  clans   5  people   35.9  depleted   34
small           turn  100  clans   6  people   54.7  depleted   41
small           turn  150  clans  10  people   74.3  depleted   43
small           turn  200  clans  10  people   84.6  depleted   35
small           turn  250  clans  12  people  102.3  depleted   29
small           turn  300  clans  14  people  111.3  depleted   28
small           turn  350  clans  15  people  119.1  depleted   23
small           turn  400  clans  15  people  125.6  depleted   26
small           turn  450  clans  16  people  129.7  depleted   21
small           turn  500  clans  18  people  135.4  depleted   22
small  seed  10: ended 499 by limit+topup centres 3 topped up 3 (1/2/0)  cover 0.41  clans  20  people  154.0  depleted   27  markets  4  hearths  20  closed 1/1/2  0.4 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0
small           turn   50  clans   5  people   37.2  depleted   32
small           turn  100  clans  11  people   86.7  depleted   53
small           turn  150  clans  12  people   96.7  depleted   44
small           turn  200  clans  14  people  114.5  depleted   28
small           turn  250  clans  17  people  129.5  depleted   31
small           turn  300  clans  18  people  134.5  depleted   33
small           turn  350  clans  18  people  137.3  depleted   25
small           turn  400  clans  19  people  146.2  depleted   26
small           turn  450  clans  20  people  154.6  depleted   31
small           turn  500  clans  20  people  154.0  depleted   27
small: the top-up took 11 existing markets, 6 tended tiles, 7 other grain tiles
small: 0 of 10 ended on their own, 10 needed the top-up, 0 ended on the limit with three centres already (cover short), median end turn 499
small: people over the last 100 turns before the end changed by +11% +27% +11% +9% +1% +4% +2% +7% +8% +5%: 7 of 10 levelled off (under 10%)
medium seed   1: ended 499 by limit       centres 3 topped up 0 (0/0/0)  cover 0.49  clans  61  people  404.7  depleted   72  markets  8  hearths  61  closed 4/2/4  1.2 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0
medium          turn   50  clans   7  people   61.7  depleted   67
medium          turn  100  clans  10  people   84.9  depleted   47
medium          turn  150  clans  16  people  124.8  depleted   72
medium          turn  200  clans  22  people  178.5  depleted   89
medium          turn  250  clans  36  people  245.8  depleted   99
medium          turn  300  clans  41  people  290.3  depleted   88
medium          turn  350  clans  46  people  336.9  depleted   86
medium          turn  400  clans  53  people  369.0  depleted   71
medium          turn  450  clans  56  people  388.8  depleted   59
medium          turn  500  clans  61  people  404.7  depleted   72
medium seed   2: ended 499 by limit+topup centres 3 topped up 1 (0/1/0)  cover 0.33  clans  27  people  213.9  depleted   39  markets  4  hearths  35  closed 4/2/4  0.6 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0
medium          turn   50  clans   7  people   51.8  depleted   54
medium          turn  100  clans   8  people   61.3  depleted   44
medium          turn  150  clans  10  people   84.1  depleted   50
medium          turn  200  clans  13  people   99.8  depleted   45
medium          turn  250  clans  14  people  113.3  depleted   38
medium          turn  300  clans  14  people  119.2  depleted   28
medium          turn  350  clans  17  people  143.4  depleted   41
medium          turn  400  clans  23  people  186.6  depleted   54
medium          turn  450  clans  23  people  196.1  depleted   43
medium          turn  500  clans  27  people  213.9  depleted   39
medium seed   3: ended 499 by limit+topup centres 3 topped up 3 (0/1/2)  cover 0.08  clans   6  people   45.0  depleted   11  markets  3  hearths   4  closed 4/2/3  0.5 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0
medium          turn   50  clans   4  people   29.9  depleted   30
medium          turn  100  clans   4  people   30.2  depleted   28
medium          turn  150  clans   5  people   39.1  depleted   12
medium          turn  200  clans   5  people   40.5  depleted   10
medium          turn  250  clans   5  people   41.9  depleted    3
medium          turn  300  clans   5  people   42.4  depleted    3
medium          turn  350  clans   6  people   44.6  depleted    6
medium          turn  400  clans   6  people   46.5  depleted   15
medium          turn  450  clans   6  people   47.3  depleted   11
medium          turn  500  clans   6  people   45.0  depleted   11
medium seed   4: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.67  clans  98  people  725.7  depleted  193  markets 11  hearths 116  closed 4/2/4  1.9 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0
medium          turn   50  clans   7  people   53.9  depleted   52
medium          turn  100  clans  11  people   90.9  depleted   55
medium          turn  150  clans  19  people  152.6  depleted   96
medium          turn  200  clans  31  people  247.5  depleted  148
medium          turn  250  clans  43  people  332.9  depleted  153
medium          turn  300  clans  57  people  404.4  depleted  163
medium          turn  350  clans  65  people  500.4  depleted  194
medium          turn  400  clans  81  people  587.7  depleted  203
medium          turn  450  clans  87  people  669.1  depleted  209
medium          turn  500  clans  98  people  725.7  depleted  193
medium seed   5: ended 499 by limit       centres 3 topped up 0 (0/0/0)  cover 0.72  clans 103  people  763.5  depleted  164  markets 10  hearths 112  closed 4/2/3  1.9 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0
medium          turn   50  clans   8  people   65.8  depleted   68
medium          turn  100  clans   9  people   81.2  depleted   48
medium          turn  150  clans  19  people  153.5  depleted   83
medium          turn  200  clans  28  people  230.6  depleted  119
medium          turn  250  clans  38  people  293.9  depleted  133
medium          turn  300  clans  46  people  371.6  depleted  140
medium          turn  350  clans  57  people  471.8  depleted  168
medium          turn  400  clans  81  people  612.6  depleted  238
medium          turn  450  clans  93  people  694.6  depleted  194
medium          turn  500  clans 103  people  763.5  depleted  164
medium seed   6: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.53  clans  57  people  450.0  depleted   79  markets  5  hearths  79  closed 4/2/3  1.3 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0
medium          turn   50  clans   9  people   64.9  depleted   52
medium          turn  100  clans  20  people  143.4  depleted   75
medium          turn  150  clans  23  people  183.2  depleted   64
medium          turn  200  clans  26  people  216.6  depleted   58
medium          turn  250  clans  31  people  259.8  depleted   49
medium          turn  300  clans  37  people  287.6  depleted   48
medium          turn  350  clans  46  people  346.5  depleted   80
medium          turn  400  clans  50  people  390.9  depleted  103
medium          turn  450  clans  55  people  420.7  depleted   98
medium          turn  500  clans  57  people  450.0  depleted   79
medium seed   7: ended 499 by limit+topup centres 3 topped up 3 (2/0/1)  cover 0.38  clans  31  people  248.9  depleted   41  markets  3  hearths  38  closed 4/2/4  0.9 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0
medium          turn   50  clans   8  people   64.4  depleted   59
medium          turn  100  clans  12  people  100.6  depleted   62
medium          turn  150  clans  14  people  124.2  depleted   56
medium          turn  200  clans  22  people  167.7  depleted   67
medium          turn  250  clans  23  people  175.7  depleted   50
medium          turn  300  clans  26  people  210.7  depleted   59
medium          turn  350  clans  26  people  220.3  depleted   37
medium          turn  400  clans  28  people  231.5  depleted   38
medium          turn  450  clans  30  people  247.3  depleted   45
medium          turn  500  clans  31  people  248.9  depleted   41
medium seed   8: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.64  clans  68  people  525.5  depleted   98  markets  9  hearths  54  closed 4/2/3  1.1 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0
medium          turn   50  clans   8  people   76.1  depleted   54
medium          turn  100  clans  15  people  123.4  depleted   67
medium          turn  150  clans  22  people  181.5  depleted   91
medium          turn  200  clans  28  people  231.3  depleted   91
medium          turn  250  clans  33  people  270.1  depleted   91
medium          turn  300  clans  40  people  325.7  depleted  106
medium          turn  350  clans  47  people  381.9  depleted  130
medium          turn  400  clans  56  people  455.3  depleted  127
medium          turn  450  clans  64  people  494.9  depleted  105
medium          turn  500  clans  68  people  525.5  depleted   98
medium seed   9: ended 499 by limit+topup centres 3 topped up 3 (1/1/1)  cover 0.16  clans  11  people   74.6  depleted   25  markets  3  hearths   2  closed 4/2/3  0.4 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0
medium          turn   50  clans   5  people   39.9  depleted   47
medium          turn  100  clans   5  people   43.7  depleted   36
medium          turn  150  clans   7  people   54.7  depleted   32
medium          turn  200  clans   7  people   56.4  depleted   32
medium          turn  250  clans   7  people   61.8  depleted   21
medium          turn  300  clans   8  people   68.2  depleted   30
medium          turn  350  clans   8  people   71.9  depleted   24
medium          turn  400  clans   8  people   72.6  depleted   23
medium          turn  450  clans  10  people   75.8  depleted   21
medium          turn  500  clans  11  people   74.6  depleted   25
medium seed  10: ended 499 by limit+topup centres 2 topped up 1 (0/0/1)  cover 0.58  clans  22  people  159.6  depleted   34  markets  4  hearths  16  closed 4/2/4  0.4 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0
medium          turn   50  clans   5  people   44.4  depleted   41
medium          turn  100  clans   7  people   64.9  depleted   49
medium          turn  150  clans  10  people   90.1  depleted   57
medium          turn  200  clans  13  people  105.1  depleted   54
medium          turn  250  clans  15  people  120.3  depleted   46
medium          turn  300  clans  16  people  132.8  depleted   41
medium          turn  350  clans  17  people  138.6  depleted   30
medium          turn  400  clans  19  people  143.2  depleted   25
medium          turn  450  clans  19  people  144.5  depleted   18
medium          turn  500  clans  22  people  159.6  depleted   34
medium: the top-up took 7 existing markets, 3 tended tiles, 5 other grain tiles
medium: 0 of 10 ended on their own, 8 needed the top-up, 2 ended on the limit with three centres already (cover short), 1 with fewer than three centres, median end turn 499
medium: people over the last 100 turns before the end changed by +10% +15% -3% +23% +25% +15% +7% +15% +3% +11%: 4 of 10 levelled off (under 10%)
large  seed   1: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.28  clans  67  people  493.1  depleted   97  markets  7  hearths  75  closed 8/4/8  2.5 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0
large           turn   50  clans   7  people   66.7  depleted   62
large           turn  100  clans  10  people   83.6  depleted   58
large           turn  150  clans  17  people  137.0  depleted  101
large           turn  200  clans  23  people  168.9  depleted   89
large           turn  250  clans  30  people  214.7  depleted   84
large           turn  300  clans  39  people  301.0  depleted  119
large           turn  350  clans  46  people  344.0  depleted  114
large           turn  400  clans  52  people  412.5  depleted  103
large           turn  450  clans  61  people  459.3  depleted  110
large           turn  500  clans  67  people  493.1  depleted   97
large  seed   2: ended 499 by limit+topup centres 3 topped up 3 (2/1/0)  cover 0.18  clans  37  people  271.5  depleted   44  markets  5  hearths  43  closed 8/4/8  2.0 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0
large           turn   50  clans   5  people   45.5  depleted   34
large           turn  100  clans   9  people   76.2  depleted   53
large           turn  150  clans  14  people  109.4  depleted   74
large           turn  200  clans  20  people  159.9  depleted   83
large           turn  250  clans  28  people  204.7  depleted   83
large           turn  300  clans  30  people  224.3  depleted   74
large           turn  350  clans  32  people  238.5  depleted   56
large           turn  400  clans  35  people  255.6  depleted   57
large           turn  450  clans  36  people  265.0  depleted   54
large           turn  500  clans  37  people  271.5  depleted   44
large  seed   3: ended 499 by limit+topup centres 3 topped up 1 (0/1/0)  cover 0.22  clans  64  people  467.0  depleted   81  markets  9  hearths  64  closed 8/4/8  2.2 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0
large           turn   50  clans  14  people  123.4  depleted   87
large           turn  100  clans  24  people  185.4  depleted   88
large           turn  150  clans  31  people  244.0  depleted   86
large           turn  200  clans  37  people  291.4  depleted   74
large           turn  250  clans  45  people  332.4  depleted   70
large           turn  300  clans  47  people  353.5  depleted   55
large           turn  350  clans  52  people  373.8  depleted   59
large           turn  400  clans  57  people  418.2  depleted   85
large           turn  450  clans  61  people  448.2  depleted   87
large           turn  500  clans  64  people  467.0  depleted   81
large  seed   4: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.23  clans  73  people  501.9  depleted   56  markets  7  hearths  95  closed 8/4/8  2.1 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0
large           turn   50  clans   6  people   49.2  depleted   41
large           turn  100  clans  12  people  106.7  depleted   67
large           turn  150  clans  23  people  176.4  depleted   85
large           turn  200  clans  33  people  248.3  depleted  118
large           turn  250  clans  37  people  294.6  depleted  104
large           turn  300  clans  46  people  339.5  depleted  101
large           turn  350  clans  52  people  385.9  depleted   99
large           turn  400  clans  60  people  429.3  depleted  104
large           turn  450  clans  69  people  469.6  depleted   81
large           turn  500  clans  73  people  501.9  depleted   56
large  seed   5: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.17  clans  37  people  266.5  depleted   48  markets  6  hearths  28  closed 8/4/8  1.3 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0
large           turn   50  clans  10  people   77.1  depleted   74
large           turn  100  clans  14  people  109.4  depleted   92
large           turn  150  clans  17  people  132.9  depleted   77
large           turn  200  clans  21  people  157.7  depleted   71
large           turn  250  clans  27  people  194.6  depleted   68
large           turn  300  clans  29  people  212.2  depleted   65
large           turn  350  clans  33  people  231.4  depleted   55
large           turn  400  clans  34  people  241.0  depleted   50
large           turn  450  clans  35  people  259.2  depleted   59
large           turn  500  clans  37  people  266.5  depleted   48
large  seed   6: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.34  clans  82  people  621.2  depleted  171  markets 11  hearths  94  closed 8/4/8  1.8 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0
large           turn   50  clans   8  people   54.8  depleted   49
large           turn  100  clans  10  people   75.7  depleted   44
large           turn  150  clans  19  people  134.6  depleted   75
large           turn  200  clans  27  people  201.0  depleted  110
large           turn  250  clans  36  people  271.0  depleted  142
large           turn  300  clans  47  people  348.5  depleted  161
large           turn  350  clans  56  people  419.7  depleted  148
large           turn  400  clans  66  people  492.7  depleted  173
large           turn  450  clans  71  people  549.9  depleted  157
large           turn  500  clans  82  people  621.2  depleted  171
large  seed   7: ended 499 by limit       centres 3 topped up 0 (0/0/0)  cover 0.30  clans  86  people  643.1  depleted   96  markets  9  hearths  98  closed 8/4/8  2.2 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0
large           turn   50  clans  10  people   88.7  depleted   78
large           turn  100  clans  21  people  184.5  depleted  121
large           turn  150  clans  28  people  259.2  depleted  122
large           turn  200  clans  44  people  343.4  depleted  147
large           turn  250  clans  53  people  412.1  depleted  145
large           turn  300  clans  64  people  473.4  depleted  130
large           turn  350  clans  67  people  515.8  depleted  120
large           turn  400  clans  73  people  541.0  depleted   94
large           turn  450  clans  79  people  599.6  depleted  101
large           turn  500  clans  86  people  643.1  depleted   96
large  seed   8: ended 499 by limit       centres 3 topped up 0 (0/0/0)  cover 0.56  clans 162  people 1238.0  depleted  320  markets 17  hearths 144  closed 8/4/8  3.0 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0
large           turn   50  clans  14  people  107.4  depleted   82
large           turn  100  clans  24  people  196.9  depleted  106
large           turn  150  clans  38  people  289.1  depleted  140
large           turn  200  clans  50  people  401.2  depleted  194
large           turn  250  clans  65  people  513.4  depleted  197
large           turn  300  clans  76  people  614.4  depleted  197
large           turn  350  clans  99  people  738.8  depleted  230
large           turn  400  clans 118  people  887.5  depleted  250
large           turn  450  clans 140  people 1038.2  depleted  275
large           turn  500  clans 162  people 1238.0  depleted  320
large  seed   9: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.26  clans  45  people  344.3  depleted   82  markets  6  hearths  59  closed 8/4/8  1.4 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0
large           turn   50  clans   7  people   72.4  depleted   68
large           turn  100  clans  12  people  104.7  depleted   82
large           turn  150  clans  19  people  148.2  depleted   79
large           turn  200  clans  21  people  186.8  depleted   80
large           turn  250  clans  26  people  219.8  depleted   63
large           turn  300  clans  29  people  242.8  depleted   68
large           turn  350  clans  32  people  270.4  depleted   59
large           turn  400  clans  34  people  282.1  depleted   60
large           turn  450  clans  39  people  313.4  depleted   65
large           turn  500  clans  45  people  344.3  depleted   82
large  seed  10: ended 499 by limit       centres 3 topped up 0 (0/0/0)  cover 0.30  clans  85  people  653.2  depleted  125  markets 11  hearths  84  closed 8/4/8  1.4 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0
large           turn   50  clans   7  people   61.6  depleted   60
large           turn  100  clans  14  people  113.6  depleted   63
large           turn  150  clans  24  people  182.0  depleted   82
large           turn  200  clans  29  people  218.7  depleted   72
large           turn  250  clans  35  people  290.8  depleted  115
large           turn  300  clans  45  people  373.4  depleted  139
large           turn  350  clans  61  people  467.4  depleted  139
large           turn  400  clans  71  people  533.4  depleted  131
large           turn  450  clans  80  people  605.3  depleted  139
large           turn  500  clans  85  people  653.2  depleted  125
large: the top-up took 9 existing markets, 2 tended tiles, 0 other grain tiles
large: 0 of 10 ended on their own, 7 needed the top-up, 3 ended on the limit with three centres already (cover short), median end turn 499
large: people over the last 100 turns before the end changed by +20% +6% +12% +17% +11% +26% +19% +39% +22% +22%: 1 of 10 levelled off (under 10%)
```
