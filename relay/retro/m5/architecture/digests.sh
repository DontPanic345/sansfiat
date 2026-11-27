B=$1
for m in "small 2" "small 6" "medium 8" "large 8"; do set -- $m; for f in "" "--lead"; do echo "$1 $2 ${f:-ai}: $($B digest --size $1 --seed $2 --turns 300 $f | tail -1)"; done; done
