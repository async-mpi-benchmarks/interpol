sed -i 's/[][]//g' *.json
echo , | tee -a *.json
cat *.json > result.json
sed -i '$ d' *.json
sed -i '1s/^/[/' *.json
echo ] | tee -a *.json
