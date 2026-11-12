cs="ec1e47b 99a6904 b377337 270d0d3 659d4d7 929b84a 06ee26b b345517 7417e2d"
printf "%-40s" file; for c in $cs; do printf "%8s" $c|cut -c1-8 | tr -d '\n'; done; echo
for f in $(git ls-tree -r --name-only HEAD engine | grep -E '\.(rs|js|mjs)$'); do
  last=$(git show HEAD:$f | wc -l)
  [ $last -lt ${MIN:-450} ] && continue
  printf "%-40s" ${f#engine/}
  for c in $cs; do n=$(git show $c:$f 2>/dev/null | wc -l); printf "%8s" $n; done; echo
done
