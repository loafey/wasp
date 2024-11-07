tac dump.tests | grep FAIL: | awk '{ print $3 }' - | awk '{ printf("FAILED: test-suite/test/core/%s\n", $1); }' -
echo "💅: ${#${$(head -n 1 dump.tests)//[^.]}}"
echo "💩: ${#${$(head -n 1 dump.tests)//[^F]}}"