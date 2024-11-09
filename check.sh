tac dump.tests | grep FAIL: | awk '{ print $3 }' - | awk '{ printf("FAILED: test-suite/test/core/%s\n", $1); }' -
echo "💅: ${#${$(head -n 1 dump.tests)//[^.]}}"
echo "💩: ${#${$(head -n 1 dump.tests)//[^F]}}"

cat readme.base.md > readme.md

echo "($(date +%y-%m-%d\ %H:%M))" >> readme.md
echo "💅: ${#${$(head -n 1 dump.tests)//[^.]}}\\" >> readme.md
echo "💩: ${#${$(head -n 1 dump.tests)//[^F]}}\\" >> readme.md
cat dump.tests | grep FAIL: | awk '{ print $3 }' - | awk '{ printf("FAILED: test-suite/test/core/%s\\\n", $1); }' - >> readme.md