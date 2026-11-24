# Lines of every engine source file at M4's end and at each M5 piece's merge.
# M4 end, L, J, S, R, C, G, P, V, Y, Q (settling), K (carry), X (exchange), fixes, art(#504 = main)
cs="7417e2d b0aa706 d66cf37 ee70756 4fea294 da69498 148528b 63b941e 4ade4df 8d1b4ed 0fd7835 d28d6e5 5da90d1 1c3f69b 4da574f"
printf "%-36s" file; for c in $cs; do printf "%8s" $c; done; echo
for f in $(git ls-tree -r --name-only 4da574f engine | grep -E '\.(rs|js|mjs)$'); do
  last=$(git show 4da574f:$f | wc -l)
  [ $last -lt ${MIN:-450} ] && continue
  printf "%-36s" ${f#engine/}
  for c in $cs; do n=$(git show $c:$f 2>/dev/null | wc -l); printf "%8s" $n; done; echo
done
