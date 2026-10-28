# Build 3 sweep: festivals, people moving, merging

`sansfiat sweep --size all --every 100`. No seed ends by centres on its own (every one by the limit; 27 of 30 topped up). Festivals standing at the end: 4–6 small, 4–10 medium, 6–16 large. People moved at festivals over a whole run (sum of the `moved` column): 0–18 small, 0–22 medium, 1–41 large, so a trickle against 100–1000 people. People levelled off (under 10% over the last 100 turns): small 2/10, medium 6/10, large 3/10. Ms a turn: 0.3–0.9 small, 0.4–1.8 medium, 1.0–3.1 large. Build-5 tunes.

```
small  seed   1: ended 499 by limit+topup centres 3 topped up 2 (1/0/1)  cover 0.44  clans  25  people  195.0  depleted   49  markets  5  hearths  17  closed 1/1/2  0.5 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
small           turn  100  clans   5  people   46.1  depleted   33  moved    0.0
small           turn  200  clans  13  people  109.7  depleted   61  moved    0.0
small           turn  300  clans  21  people  158.4  depleted   68  moved    0.0
small           turn  400  clans  22  people  177.1  depleted   52  moved    0.1
small           turn  500  clans  25  people  195.0  depleted   49  moved    0.0
small  seed   2: ended 499 by limit+topup centres 3 topped up 3 (3/0/0)  cover 0.47  clans  18  people  136.0  depleted   22  markets  4  hearths  15  closed 2/1/2  0.3 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
small           turn  100  clans   4  people   40.3  depleted   19  moved    0.0
small           turn  200  clans   5  people   49.3  depleted   16  moved    0.8
small           turn  300  clans   8  people   56.8  depleted   13  moved    0.0
small           turn  400  clans  13  people  109.4  depleted   32  moved    0.0
small           turn  500  clans  18  people  136.0  depleted   22  moved    0.0
small  seed   3: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.59  clans  27  people  213.0  depleted   39  markets  4  hearths  23  closed 1/1/1  0.6 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
small           turn  100  clans   9  people   74.5  depleted   36  moved    1.8
small           turn  200  clans  15  people  120.1  depleted   53  moved    2.0
small           turn  300  clans  18  people  148.9  depleted   49  moved    0.0
small           turn  400  clans  24  people  193.0  depleted   58  moved    0.0
small           turn  500  clans  27  people  213.0  depleted   39  moved    0.0
small  seed   4: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.67  clans  43  people  315.6  depleted   53  markets  4  hearths  40  closed 2/1/1  0.8 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
small           turn  100  clans   7  people   56.3  depleted   43  moved    0.0
small           turn  200  clans  16  people  129.0  depleted   38  moved    2.8
small           turn  300  clans  30  people  216.7  depleted   75  moved    8.2
small           turn  400  clans  34  people  277.8  depleted   50  moved    0.0
small           turn  500  clans  43  people  315.6  depleted   53  moved    1.0
small  seed   5: ended 499 by limit+topup centres 3 topped up 3 (1/0/2)  cover 0.29  clans  11  people   86.8  depleted   25  markets  5  hearths  10  closed 2/1/1  0.3 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
small           turn  100  clans   4  people   38.3  depleted   26  moved    0.0
small           turn  200  clans   7  people   61.7  depleted   33  moved    0.0
small           turn  300  clans   8  people   70.8  depleted   20  moved    0.0
small           turn  400  clans  11  people   81.7  depleted   32  moved    2.5
small           turn  500  clans  11  people   86.8  depleted   25  moved    0.0
small  seed   6: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.70  clans  46  people  353.8  depleted   52  markets  5  hearths  74  closed 1/1/2  0.9 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
small           turn  100  clans  15  people  123.9  depleted   73  moved    2.1
small           turn  200  clans  27  people  207.3  depleted   77  moved    4.5
small           turn  300  clans  31  people  263.7  depleted   57  moved    6.0
small           turn  400  clans  38  people  320.2  depleted   71  moved    0.9
small           turn  500  clans  46  people  353.8  depleted   52  moved    1.3
small  seed   7: ended 499 by limit+topup centres 3 topped up 3 (1/0/2)  cover 0.23  clans  17  people  125.5  depleted   26  markets  4  hearths  18  closed 2/1/2  0.3 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
small           turn  100  clans   7  people   58.6  depleted   29  moved    0.5
small           turn  200  clans  11  people   81.1  depleted   32  moved    1.5
small           turn  300  clans  13  people   94.7  depleted   24  moved    0.3
small           turn  400  clans  15  people  100.6  depleted   15  moved    0.0
small           turn  500  clans  17  people  125.5  depleted   26  moved    0.0
small  seed   8: ended 499 by limit+topup centres 3 topped up 1 (0/0/1)  cover 0.62  clans  43  people  274.4  depleted   76  markets  6  hearths  41  closed 2/1/2  0.6 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
small           turn  100  clans   9  people   67.9  depleted   22  moved    0.0
small           turn  200  clans  14  people  101.7  depleted   21  moved    1.4
small           turn  300  clans  26  people  166.5  depleted   65  moved    0.8
small           turn  400  clans  35  people  234.7  depleted   78  moved   12.5
small           turn  500  clans  43  people  274.4  depleted   76  moved    3.1
small  seed   9: ended 499 by limit+topup centres 3 topped up 3 (3/0/0)  cover 0.47  clans  18  people  135.1  depleted   25  markets  4  hearths  27  closed 2/1/1  0.4 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
small           turn  100  clans   5  people   48.8  depleted   27  moved    0.0
small           turn  200  clans   9  people   82.6  depleted   33  moved    0.0
small           turn  300  clans  12  people  101.0  depleted   26  moved    0.0
small           turn  400  clans  15  people  118.0  depleted   33  moved    0.0
small           turn  500  clans  18  people  135.1  depleted   25  moved    0.9
small  seed  10: ended 499 by limit+topup centres 3 topped up 3 (2/1/0)  cover 0.34  clans  17  people  139.2  depleted   10  markets  5  hearths  18  closed 1/1/2  0.4 ms a turn
small           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
small           turn  100  clans   7  people   64.8  depleted   44  moved    0.0
small           turn  200  clans  14  people  111.3  depleted   40  moved    0.6
small           turn  300  clans  16  people  127.7  depleted   18  moved    0.0
small           turn  400  clans  17  people  135.5  depleted   13  moved    0.0
small           turn  500  clans  17  people  139.2  depleted   10  moved    0.5
small: the top-up took 16 existing markets, 1 tended tiles, 6 other grain tiles
small: 0 of 10 ended on their own, 10 needed the top-up, 0 ended on the limit with three centres already (cover short), median end turn 499
small: people over the last 100 turns before the end changed by +10% +24% +10% +14% +6% +10% +25% +17% +14% +3%: 2 of 10 levelled off (under 10%)
medium seed   1: ended 499 by limit+topup centres 3 topped up 3 (3/0/0)  cover 0.31  clans  30  people  224.6  depleted   29  markets  6  hearths  36  closed 4/2/4  0.8 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0  moved    0.0
medium          turn  100  clans   9  people   79.6  depleted   45  moved    0.0
medium          turn  200  clans  17  people  138.2  depleted   64  moved    3.0
medium          turn  300  clans  25  people  192.3  depleted   52  moved    0.2
medium          turn  400  clans  27  people  218.4  depleted   36  moved    0.4
medium          turn  500  clans  30  people  224.6  depleted   29  moved    0.0
medium seed   2: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.33  clans  31  people  233.4  depleted   23  markets  5  hearths  50  closed 4/2/4  1.2 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0  moved    0.0
medium          turn  100  clans  11  people   77.6  depleted   51  moved    1.1
medium          turn  200  clans  20  people  161.8  depleted   58  moved    0.0
medium          turn  300  clans  24  people  203.4  depleted   37  moved    1.2
medium          turn  400  clans  28  people  218.3  depleted   25  moved    0.8
medium          turn  500  clans  31  people  233.4  depleted   23  moved    0.3
medium seed   3: ended 499 by limit+topup centres 3 topped up 3 (1/0/2)  cover 0.08  clans   6  people   44.6  depleted    9  markets  5  hearths   4  closed 4/2/3  0.8 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0  moved    0.0
medium          turn  100  clans   4  people   29.6  depleted   24  moved    0.0
medium          turn  200  clans   5  people   39.8  depleted    9  moved    0.0
medium          turn  300  clans   5  people   41.1  depleted    4  moved    0.0
medium          turn  400  clans   6  people   45.8  depleted   15  moved    0.0
medium          turn  500  clans   6  people   44.6  depleted    9  moved    0.0
medium seed   4: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.55  clans  68  people  524.3  depleted  118  markets 10  hearths  90  closed 4/2/4  1.8 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0  moved    0.0
medium          turn  100  clans  13  people  101.4  depleted   70  moved    1.7
medium          turn  200  clans  26  people  221.9  depleted  135  moved    1.2
medium          turn  300  clans  44  people  369.9  depleted  164  moved    5.3
medium          turn  400  clans  61  people  461.7  depleted  119  moved    3.7
medium          turn  500  clans  68  people  524.3  depleted  118  moved    6.4
medium seed   5: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.71  clans  91  people  707.0  depleted  210  markets 10  hearths 100  closed 4/2/3  1.7 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0  moved    0.0
medium          turn  100  clans  10  people   94.6  depleted   56  moved    0.0
medium          turn  200  clans  18  people  162.3  depleted   72  moved    0.0
medium          turn  300  clans  36  people  292.5  depleted  127  moved    0.6
medium          turn  400  clans  66  people  553.0  depleted  194  moved    5.4
medium          turn  500  clans  91  people  707.0  depleted  210  moved   16.1
medium seed   6: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.55  clans  51  people  390.5  depleted   96  markets  9  hearths  62  closed 4/2/3  1.3 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0  moved    0.0
medium          turn  100  clans  20  people  146.9  depleted   86  moved    0.0
medium          turn  200  clans  28  people  213.5  depleted   68  moved    5.8
medium          turn  300  clans  34  people  258.2  depleted   68  moved    4.1
medium          turn  400  clans  42  people  298.3  depleted   52  moved    4.2
medium          turn  500  clans  51  people  390.5  depleted   96  moved    0.0
medium seed   7: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.38  clans  34  people  261.6  depleted   31  markets  6  hearths  36  closed 4/2/4  1.0 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0  moved    0.0
medium          turn  100  clans  10  people   91.7  depleted   50  moved    0.0
medium          turn  200  clans  22  people  162.7  depleted   85  moved    2.0
medium          turn  300  clans  30  people  218.9  depleted   69  moved    0.8
medium          turn  400  clans  33  people  253.5  depleted   53  moved    0.0
medium          turn  500  clans  34  people  261.6  depleted   31  moved    0.0
medium seed   8: ended 499 by limit       centres 3 topped up 0 (0/0/0)  cover 0.65  clans  72  people  538.9  depleted  141  markets 10  hearths  51  closed 4/2/3  1.2 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0  moved    0.0
medium          turn  100  clans  15  people  110.7  depleted   58  moved    1.3
medium          turn  200  clans  24  people  194.3  depleted   75  moved    0.4
medium          turn  300  clans  36  people  284.7  depleted   95  moved    1.1
medium          turn  400  clans  53  people  413.8  depleted  132  moved    4.5
medium          turn  500  clans  72  people  538.9  depleted  141  moved    0.5
medium seed   9: ended 499 by limit+topup centres 3 topped up 3 (2/1/0)  cover 0.15  clans   8  people   69.2  depleted   16  markets  4  hearths   2  closed 4/2/3  0.4 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0  moved    0.0
medium          turn  100  clans   5  people   47.5  depleted   33  moved    0.0
medium          turn  200  clans   7  people   57.6  depleted   32  moved    0.0
medium          turn  300  clans   8  people   65.3  depleted   30  moved    0.0
medium          turn  400  clans   8  people   70.0  depleted   18  moved    0.0
medium          turn  500  clans   8  people   69.2  depleted   16  moved    0.0
medium seed  10: ended 499 by limit+topup centres 3 topped up 3 (2/0/1)  cover 0.20  clans  18  people  140.1  depleted   17  markets  4  hearths  19  closed 4/2/4  0.4 ms a turn
medium          turn    0  clans   3  people   18.0  depleted    0  moved    0.0
medium          turn  100  clans   8  people   64.8  depleted   48  moved    0.1
medium          turn  200  clans  15  people  116.5  depleted   66  moved    0.3
medium          turn  300  clans  17  people  137.7  depleted   28  moved    0.0
medium          turn  400  clans  17  people  143.5  depleted   18  moved    0.0
medium          turn  500  clans  18  people  140.1  depleted   17  moved    0.0
medium: the top-up took 16 existing markets, 1 tended tiles, 3 other grain tiles
medium: 0 of 10 ended on their own, 9 needed the top-up, 1 ended on the limit with three centres already (cover short), median end turn 499
medium: people over the last 100 turns before the end changed by +3% +7% -3% +14% +28% +31% +3% +30% -1% -2%: 6 of 10 levelled off (under 10%)
large  seed   1: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.23  clans  46  people  366.6  depleted   73  markets  7  hearths  73  closed 8/4/8  2.7 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
large           turn  100  clans  12  people  103.5  depleted   67  moved    0.0
large           turn  200  clans  23  people  182.7  depleted   93  moved    0.0
large           turn  300  clans  35  people  258.2  depleted   81  moved    2.0
large           turn  400  clans  42  people  328.2  depleted   63  moved    1.5
large           turn  500  clans  46  people  366.6  depleted   73  moved    0.5
large  seed   2: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.23  clans  36  people  282.7  depleted   61  markets  7  hearths  40  closed 8/4/8  2.1 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
large           turn  100  clans   6  people   58.9  depleted   31  moved    1.1
large           turn  200  clans  12  people  100.3  depleted   48  moved    0.0
large           turn  300  clans  24  people  177.7  depleted   70  moved    0.0
large           turn  400  clans  30  people  228.2  depleted   57  moved    5.9
large           turn  500  clans  36  people  282.7  depleted   61  moved    1.1
large  seed   3: ended 499 by limit       centres 3 topped up 0 (0/0/0)  cover 0.21  clans  63  people  457.9  depleted   62  markets  8  hearths  61  closed 8/4/8  3.1 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
large           turn  100  clans  21  people  179.6  depleted   91  moved    0.0
large           turn  200  clans  39  people  299.5  depleted   78  moved    8.1
large           turn  300  clans  48  people  356.3  depleted   73  moved    2.5
large           turn  400  clans  56  people  415.1  depleted   71  moved    1.4
large           turn  500  clans  63  people  457.9  depleted   62  moved    3.6
large  seed   4: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.22  clans  66  people  494.2  depleted   47  markets  6  hearths 110  closed 8/4/8  2.2 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
large           turn  100  clans  20  people  150.1  depleted   97  moved    2.7
large           turn  200  clans  37  people  300.6  depleted  126  moved    7.1
large           turn  300  clans  53  people  403.9  depleted   97  moved   11.5
large           turn  400  clans  63  people  460.2  depleted   84  moved   12.7
large           turn  500  clans  66  people  494.2  depleted   47  moved    1.7
large  seed   5: ended 499 by limit+topup centres 3 topped up 3 (3/0/0)  cover 0.17  clans  36  people  265.3  depleted   50  markets  6  hearths  29  closed 8/4/8  1.2 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
large           turn  100  clans  11  people  104.2  depleted   74  moved    0.0
large           turn  200  clans  18  people  155.8  depleted   70  moved    0.0
large           turn  300  clans  28  people  221.4  depleted   69  moved    1.0
large           turn  400  clans  31  people  243.0  depleted   42  moved    0.0
large           turn  500  clans  36  people  265.3  depleted   50  moved    0.0
large  seed   6: ended 499 by limit+topup centres 3 topped up 3 (2/1/0)  cover 0.14  clans  29  people  232.3  depleted   89  markets  6  hearths  30  closed 8/4/8  1.0 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
large           turn  100  clans   7  people   62.2  depleted   36  moved    1.0
large           turn  200  clans  12  people  100.7  depleted   28  moved    0.0
large           turn  300  clans  14  people  111.6  depleted   19  moved    0.1
large           turn  400  clans  17  people  127.0  depleted   28  moved    1.1
large           turn  500  clans  29  people  232.3  depleted   89  moved    0.0
large  seed   7: ended 499 by limit+topup centres 3 topped up 1 (1/0/0)  cover 0.26  clans  83  people  600.5  depleted   60  markets  9  hearths 109  closed 8/4/8  2.2 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
large           turn  100  clans  20  people  163.1  depleted   98  moved    0.0
large           turn  200  clans  49  people  367.3  depleted  155  moved    7.8
large           turn  300  clans  65  people  488.5  depleted  113  moved   12.5
large           turn  400  clans  73  people  547.3  depleted   77  moved    2.3
large           turn  500  clans  83  people  600.5  depleted   60  moved    0.0
large  seed   8: ended 499 by limit       centres 3 topped up 0 (0/0/0)  cover 0.55  clans 147  people 1151.6  depleted  232  markets 16  hearths 189  closed 8/4/8  3.0 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
large           turn  100  clans  33  people  248.0  depleted  170  moved    7.5
large           turn  200  clans  64  people  511.5  depleted  208  moved    6.6
large           turn  300  clans  85  people  700.8  depleted  169  moved    7.8
large           turn  400  clans 118  people  949.5  depleted  248  moved   14.4
large           turn  500  clans 147  people 1151.6  depleted  232  moved    4.3
large  seed   9: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 0.40  clans  83  people  655.8  depleted  174  markets 12  hearths  99  closed 8/4/8  2.0 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
large           turn  100  clans  13  people  108.5  depleted   88  moved    0.0
large           turn  200  clans  26  people  219.2  depleted  118  moved    2.2
large           turn  300  clans  46  people  369.4  depleted  131  moved    4.2
large           turn  400  clans  69  people  541.1  depleted  160  moved    2.9
large           turn  500  clans  83  people  655.8  depleted  174  moved    1.5
large  seed  10: ended 499 by limit+topup centres 3 topped up 2 (1/1/0)  cover 0.21  clans  62  people  474.9  depleted  122  markets 10  hearths  54  closed 8/4/8  1.5 ms a turn
large           turn    0  clans   3  people   18.0  depleted    0  moved    0.0
large           turn  100  clans  14  people  116.8  depleted   66  moved    0.0
large           turn  200  clans  27  people  198.1  depleted   88  moved    2.0
large           turn  300  clans  31  people  244.0  depleted   57  moved    0.3
large           turn  400  clans  46  people  368.8  depleted  129  moved    2.7
large           turn  500  clans  62  people  474.9  depleted  122  moved    5.7
large: the top-up took 14 existing markets, 2 tended tiles, 0 other grain tiles
large: 0 of 10 ended on their own, 8 needed the top-up, 2 ended on the limit with three centres already (cover short), median end turn 499
large: people over the last 100 turns before the end changed by +12% +24% +10% +7% +9% +83% +10% +21% +21% +29%: 3 of 10 levelled off (under 10%)
```
