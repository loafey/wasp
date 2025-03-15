tac dump.tests | grep FAIL: | awk '{ print $3 }' - | awk '{ printf("FAILED: test-suite/test/core/%s\n", $1); }' -
echo "💅: ${#${$(head -n 1 dump.tests)//[^.]}}"
echo "💩: ${#${$(head -n 1 dump.tests)//[^F]}}"

cat readme.base.md > readme.md

echo "" >> readme.md
# echo "($(date +%y-%m-%d\ %H:%M))" >> readme.md
echo "💅: ${#${$(head -n 1 dump.tests)//[^.]}}\\" >> readme.md
echo "💩: ${#${$(head -n 1 dump.tests)//[^F]}}" >> readme.md

FAILED=$(cat dump.tests | grep FAIL: | awk '{ print $3 }' - | awk '{ printf("test-suite/test/core/%s\n", $1); }' -)
echo $FAILED |
while IFS= read -r line; do
    echo "## Failed: $line" >> readme.md
    echo "\`\`\`bash" >> readme.md
    (./target/debug/wasp "$line" || true) >> readme.md 2>&1
    echo "\`\`\`" >> readme.md
    echo "" >> readme.md
done
f=$(cat readme.md | sed -r "s/\x1B\[([0-9]{1,3}(;[0-9]{1,2};?)?)?[mGK]//g" | grep -v "  INFO ") 
echo $f > readme.md

echo "\n# Opinionated order of tests" >> readme.md
num=1
cat test-order.txt |
while IFS= read -r line; do
    if  grep -q "/$line.wast" "readme.md"
    then
        echo "$num. ❌ $line" >> readme.md
    else
        echo "$num. ✅ $line" >> readme.md
    fi
    num=$((num+1))
done