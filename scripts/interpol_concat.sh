#!/bin/bash

output=$1
if [ -z $output ]; then
    output="interpol_traces.json"
fi

printf "\033[1;34m==>\033[0m Concatening \`Interpol\` traces into \033[35m%s\033[0m...\n" "$output"
ranks=$(ls -d rank*_traces.json)

start=$(date +%s.%N)
echo "[" > $output
mkdir -p tmp
for file in $ranks; do
    tail -n +2 $file > tmp/a && mv tmp/a $file
    sed -i '$ d' $file
    sed -i '$ d' $file
    echo "  }," >> $file
done
rm -rf tmp/
cat rank*_traces.json >> $output
rm -rf rank*_traces.json
echo "]" >> $output
end=$(date +%s.%N)

pwd=$(pwd)
printf "\033[1;32m[+]\033[0m %s\n" "${pwd}/${output}"

elapsed=$(echo "scale=4; $end - $start" | bc -l)
printf "\nFinished concatening traces in \033[36m%.2fs\033[0m.\n" $elapsed
