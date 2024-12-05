# W.A.S.P

## Latest spec test (typechecking currently disabled)
💅: 43\
💩: 104
## Failed: test-suite/test/core/address.wast
```bash

1/260
1 2
1 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
        98,
        99,
        100,
        101,
        102,
        103,
        104,
        105,
        106,
        107,
        108,
        109,
        110,
        111,
        112,
        113,
        114,
        115,
        116,
        117,
        118,
        119,
        120,
        121,
        122,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory

2/260

3/260

4/260

5/260

6/260

7/260

8/260

9/260

10/260

11/260

12/260

13/260

14/260

15/260

16/260

17/260

18/260

19/260

20/260

21/260

22/260

23/260

24/260

25/260

26/260

27/260

28/260

29/260

30/260

31/260

32/260

33/260

34/260

35/260

36/260

37/260

38/260

39/260

40/260

41/260

42/260

43/260

44/260

45/260

46/260

47/260

48/260

49/260

50/260

51/260

52/260

53/260

54/260

55/260

56/260

57/260

58/260

59/260

60/260

61/260

62/260

63/260

64/260

65/260

66/260

67/260

68/260

69/260

70/260

71/260

72/260

73/260

74/260

75/260

76/260
 ERROR wasp::testsuite > test 76/260 did not fail, expected error: "out of bounds memory access" (module: 0, function "32_good5")
```

## Failed: test-suite/test/core/align.wast
```bash

1/162
1 2
0 18446744073709551615

2/162
1 2
0 18446744073709551615

3/162
1 2
0 18446744073709551615

4/162
1 2
0 18446744073709551615

5/162
1 2
0 18446744073709551615

6/162
1 2
0 18446744073709551615

7/162
1 2
0 18446744073709551615

8/162
1 2
0 18446744073709551615

9/162
1 2
0 18446744073709551615

10/162
1 2
0 18446744073709551615

11/162
1 2
0 18446744073709551615

12/162
1 2
0 18446744073709551615

13/162
1 2
0 18446744073709551615

14/162
1 2
0 18446744073709551615

15/162
1 2
0 18446744073709551615

16/162
1 2
0 18446744073709551615

17/162
1 2
0 18446744073709551615

18/162
1 2
0 18446744073709551615

19/162
1 2
0 18446744073709551615

20/162
1 2
0 18446744073709551615

21/162
1 2
0 18446744073709551615

22/162
1 2
0 18446744073709551615

23/162
1 2
0 18446744073709551615

24/162

25/162

26/162

27/162

28/162

29/162

30/162

31/162

32/162

33/162

34/162

35/162

36/162

37/162

38/162

39/162

40/162

41/162

42/162

43/162

44/162

45/162

46/162

47/162

48/162

49/162

50/162

51/162

52/162

53/162

54/162

55/162

56/162

57/162

58/162

59/162

60/162

61/162

62/162

63/162

64/162

65/162

66/162

67/162

68/162

69/162

70/162
1 2

71/162
1 2

72/162
1 2

73/162
1 2

74/162
1 2

75/162
1 2

76/162
1 2

77/162
1 2

78/162
1 2

79/162
1 2

80/162
1 2

81/162
1 2

82/162
1 2

83/162
1 2

84/162
1 2

85/162
1 2

86/162
1 2

87/162
1 2

88/162
1 2

89/162
1 2

90/162
1 2

91/162
1 2

92/162
1 2

93/162
1 2

94/162
1 2

95/162
1 2

96/162
1 2

97/162
1 2

98/162
1 2

99/162
1 2

100/162
1 2

101/162
1 2

102/162
1 2

103/162
1 2

104/162
1 2

105/162
1 2

106/162
1 2

107/162
1 2
1 18446744073709551615

108/162

109/162

110/162

111/162

112/162

113/162

114/162

115/162

116/162

117/162

118/162

119/162

120/162

121/162

122/162

123/162

124/162

125/162

126/162

127/162

128/162

129/162

130/162

131/162

132/162

133/162

134/162

135/162

136/162

137/162

138/162

139/162

140/162

141/162

142/162

143/162

144/162

145/162

146/162

147/162

148/162

149/162

150/162

151/162

152/162

153/162

154/162
1 2
1 18446744073709551615

155/162

156/162
 ERROR wasp::testsuite > test 156/162 failed (module: 24, invoke: "load", got [i32(-1)], but expected [i32(0)])
```

## Failed: test-suite/test/core/bulk.wast
```bash

1/117
1 2
1 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Passive(
    [
        102,
        111,
        111,
    ],
)

2/117
1 2
1 18446744073709551615

3/117
1 2
1 18446744073709551615

4/117

5/117

6/117

7/117

8/117

9/117

10/117

11/117

12/117

13/117

14/117

15/117
 ERROR wasp::testsuite > test 15/117 failed (module: 2, invoke: "load8_u", got [i32(1)], but expected [i32(0)])
```

## Failed: test-suite/test/core/data.wast
```bash

1/61
1 2
1 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                1,
            ),
        ],
    },
    [
        97,
        98,
        99,
        100,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
        98,
        99,
    ],
)
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                1,
            ),
        ],
    },
    [
        97,
        98,
        99,
        100,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
        98,
        99,
    ],
)
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                1,
            ),
        ],
    },
    [
        97,
        98,
        99,
        100,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
        98,
        99,
    ],
)
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                1,
            ),
        ],
    },
    [
        97,
        98,
        99,
        100,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
        98,
        99,
    ],
)
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                1,
            ),
        ],
    },
    [
        97,
        98,
        99,
        100,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
        98,
        99,
    ],
)
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                1,
            ),
        ],
    },
    [
        97,
        98,
        99,
        100,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
        98,
        99,
    ],
)
setting memory
setting memory
setting memory

2/61
1 2
1 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

3/61
1 2
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

4/61
1 2
1 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                3,
            ),
        ],
    },
    [
        98,
    ],
)
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                100,
            ),
        ],
    },
    [
        99,
        100,
        101,
    ],
)
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                5,
            ),
        ],
    },
    [
        120,
    ],
)
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                3,
            ),
        ],
    },
    [
        99,
    ],
)
setting memory

5/61
1 2
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                1,
            ),
        ],
    },
    [
        98,
    ],
)
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                2,
            ),
        ],
    },
    [
        99,
        100,
        101,
    ],
)
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                3,
            ),
        ],
    },
    [
        102,
    ],
)
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                2,
            ),
        ],
    },
    [
        103,
    ],
)
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                1,
            ),
        ],
    },
    [
        104,
    ],
)
setting memory

6/61
1 2
1 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x23_global_get(
                GlobalIdX(
                    0,
                ),
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

7/61
1 2
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x23_global_get(
                GlobalIdX(
                    0,
                ),
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

8/61
1 2
1 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x23_global_get(
                GlobalIdX(
                    0,
                ),
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

9/61
1 2
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x23_global_get(
                GlobalIdX(
                    0,
                ),
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

10/61
1 2
1 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x23_global_get(
                GlobalIdX(
                    0,
                ),
            ),
        ],
    },
    [
        97,
    ],
)

11/61
1 2
1 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x23_global_get(
                GlobalIdX(
                    0,
                ),
            ),
        ],
    },
    [
        97,
    ],
)

12/61
1 2
1 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                65535,
            ),
        ],
    },
    [
        98,
    ],
)
setting memory

13/61
1 2
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                65535,
            ),
        ],
    },
    [
        98,
    ],
)
setting memory

14/61
1 2
2 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                131071,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

15/61
1 2
0 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)

16/61
1 2
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)

17/61
1 2
0 0
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)

18/61
1 2
1 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                65536,
            ),
        ],
    },
    [],
)

19/61
1 2
0 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)

20/61
1 2
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)

21/61
1 2
0 0
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [],
)

22/61
1 2
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

23/61
1 2
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

24/61
1 2
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x23_global_get(
                GlobalIdX(
                    0,
                ),
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

25/61
1 2
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x23_global_get(
                GlobalIdX(
                    0,
                ),
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

26/61
1 2
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                1,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

27/61
1 2
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                1,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

28/61
1 2
0 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

29/61
1 2
0 0
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

30/61
1 2
0 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

31/61
1 2
0 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                1,
            ),
        ],
    },
    [],
)

32/61
1 2
0 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                1,
            ),
        ],
    },
    [],
)

33/61
1 2
0 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x23_global_get(
                GlobalIdX(
                    0,
                ),
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

34/61
1 2
1 2
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                65536,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

35/61
1 2
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                65536,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

36/61
1 2
2 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                131072,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

37/61
1 2
2 3
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                131072,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

38/61
1 2
1 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                -1,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/runtime/memory.rs:82:31:
	attempt to add with overflow
Last test (37):
	AssertUninstantiable(AssertUninstantiable { _type: MustBe!("assert_uninstantiable"), filename: "data.37.wasm", text: "out of bounds memory access", module_type: Binary })
```

## Failed: test-suite/test/core/elem.wast
```bash

1/98
1 2
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/runtime/clean_model.rs:578:34:
	not yet implemented: xd2_ref_func(FuncIdx(0))
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "elem.0.wasm" })
```

## Failed: test-suite/test/core/endianness.wast
```bash

1/69
1 2
1 18446744073709551615

2/69
 ERROR wasp::testsuite > test 2/69 failed (module: 0, invoke: "i32_load16_s", got [i32(65535)], but expected [i32(-1)])
```

## Failed: test-suite/test/core/exports.wast
```bash

1/96
1 2
1 18446744073709551615

2/96
1 2
1 18446744073709551615

3/96
1 2
1 18446744073709551615

4/96
1 2
1 18446744073709551615

5/96
1 2
1 18446744073709551615

6/96
1 2
1 18446744073709551615

7/96
1 2
1 18446744073709551615

8/96
1 2
1 18446744073709551615

9/96
1 2
1 18446744073709551615

10/96
1 2
1 18446744073709551615

11/96
1 2
1 18446744073709551615

12/96
1 2
1 18446744073709551615

13/96

14/96
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:259:17:
	not yet implemented
Last test (13):
	AssertReturn(AssertReturn { _type: MustBe!("assert_return"), action: Invoke { module: Some("$Func"), field: "e", args: [I32 { value: "42" }] }, expected: [I32 { value: "43" }] })
```

## Failed: test-suite/test/core/fac.wast
```bash

1/8
1 2
1 18446744073709551615

2/8

3/8

4/8

5/8

6/8

7/8
 ERROR wasp::testsuite > test 7/8 failed (module: 0, invoke: "fac-ssa", error: wrong type popped from stack (got BlockLock, expected i64): src/runtime/methods/step.rs:992:25)
```

## Failed: test-suite/test/core/float_exprs.wast
```bash

1/927
1 2
1 18446744073709551615

2/927

3/927

4/927

5/927

6/927

7/927
1 2
1 18446744073709551615

8/927

9/927

10/927

11/927

12/927

13/927

14/927

15/927

16/927

17/927

18/927
1 2
1 18446744073709551615

19/927

20/927

21/927

22/927

23/927
1 2
1 18446744073709551615

24/927

25/927

26/927

27/927

28/927
1 2
1 18446744073709551615

29/927

30/927

31/927
1 2
1 18446744073709551615

32/927

33/927

34/927

35/927

36/927

37/927

38/927

39/927

40/927
1 2
1 18446744073709551615

41/927

42/927

43/927
1 2
1 18446744073709551615

44/927

45/927

46/927

47/927

48/927

49/927

50/927

51/927

52/927
1 2
1 18446744073709551615

53/927

54/927

55/927
1 2
1 18446744073709551615

56/927

57/927

58/927
1 2
1 18446744073709551615

59/927

60/927

61/927
1 2
1 18446744073709551615

62/927

63/927

64/927
1 2
1 18446744073709551615

65/927

66/927

67/927
1 2
1 18446744073709551615

68/927

69/927

70/927
1 2
1 18446744073709551615

71/927

72/927

73/927

74/927

75/927
1 2
1 18446744073709551615

76/927

77/927

78/927

79/927

80/927

81/927

82/927

83/927

84/927
1 2
1 18446744073709551615

85/927

86/927

87/927

88/927

89/927

90/927

91/927

92/927

93/927

94/927

95/927
1 2
1 18446744073709551615

96/927

97/927

98/927

99/927

100/927

101/927

102/927

103/927

104/927

105/927

106/927
1 2
1 18446744073709551615

107/927

108/927

109/927

110/927

111/927

112/927

113/927

114/927

115/927

116/927

117/927
1 2
1 18446744073709551615

118/927

119/927

120/927

121/927

122/927

123/927

124/927

125/927

126/927

127/927

128/927
1 2
1 18446744073709551615

129/927

130/927

131/927

132/927

133/927

134/927

135/927

136/927

137/927

138/927

139/927
1 2
1 18446744073709551615

140/927

141/927

142/927

143/927

144/927

145/927

146/927

147/927

148/927

149/927

150/927
1 2
1 18446744073709551615

151/927

152/927

153/927

154/927

155/927

156/927

157/927

158/927

159/927

160/927

161/927
1 2
1 18446744073709551615

162/927

163/927

164/927

165/927

166/927

167/927

168/927

169/927

170/927

171/927

172/927

173/927

174/927

175/927

176/927

177/927

178/927
1 2
1 18446744073709551615

179/927

180/927

181/927

182/927

183/927

184/927

185/927

186/927

187/927

188/927

189/927

190/927

191/927

192/927

193/927

194/927

195/927
1 2
1 18446744073709551615

196/927

197/927

198/927

199/927

200/927

201/927

202/927

203/927

204/927

205/927

206/927
1 2
1 18446744073709551615

207/927

208/927

209/927

210/927

211/927

212/927
1 2
1 18446744073709551615

213/927

214/927

215/927

216/927

217/927

218/927

219/927

220/927

221/927

222/927

223/927
1 2
1 18446744073709551615

224/927

225/927

226/927

227/927

228/927

229/927
1 2
1 18446744073709551615

230/927

231/927

232/927

233/927

234/927

235/927

236/927

237/927

238/927

239/927

240/927

241/927

242/927
1 2
1 18446744073709551615

243/927

244/927

245/927

246/927

247/927

248/927

249/927

250/927

251/927

252/927

253/927
1 2
1 18446744073709551615

254/927

255/927

256/927

257/927

258/927

259/927

260/927

261/927

262/927

263/927

264/927
1 2
1 18446744073709551615

265/927

266/927

267/927

268/927

269/927

270/927

271/927

272/927

273/927

274/927

275/927
1 2
1 18446744073709551615

276/927

277/927

278/927

279/927

280/927

281/927

282/927

283/927

284/927

285/927

286/927
1 2
1 18446744073709551615

287/927

288/927

289/927
1 2
1 18446744073709551615

290/927

291/927

292/927

293/927

294/927

295/927
1 2
1 18446744073709551615

296/927

297/927

298/927

299/927

300/927

301/927

302/927

303/927

304/927

305/927

306/927

307/927

308/927

309/927
1 2
1 18446744073709551615

310/927

311/927

312/927

313/927

314/927

315/927

316/927

317/927

318/927

319/927

320/927
1 2
1 18446744073709551615

321/927

322/927

323/927

324/927

325/927

326/927

327/927

328/927

329/927

330/927

331/927
1 2
1 18446744073709551615

332/927
 ERROR wasp::testsuite > test 332/927 failed (module: 39, invoke: "no_demote_mixed_mul", got [i32(0)], but expected [i32(329178166)])
```

## Failed: test-suite/test/core/func.wast
```bash

1/172
1 2
1 18446744073709551615

2/172

3/172

4/172

5/172

6/172

7/172

8/172

9/172
 ERROR wasp::testsuite > test 9/172 failed (module: 0, invoke: "local-first-i32", error: a local is missing: src/runtime/methods/step.rs:536:51)
```

## Failed: test-suite/test/core/func_ptrs.wast
```bash

1/36
1 2
1 18446744073709551615

2/36

3/36

4/36

5/36
83

6/36
1 2
1 18446744073709551615
 ERROR wasp::testsuite > test 6/36 did not fail invalidating/parsing, expected error: "unknown table" (module: "test-suite/test/core/func_ptrs.1.wasm")
```

## Failed: test-suite/test/core/global.wast
```bash

1/110
1 2
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:374:32:
	failed to load module: GlobalWithoutOffset
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "global.0.wasm" })
```

## Failed: test-suite/test/core/if.wast
```bash

1/241
1 2
1 18446744073709551615

2/241

3/241

4/241

5/241

6/241

7/241

8/241

9/241

10/241

11/241

12/241

13/241

14/241

15/241

16/241

17/241

18/241

19/241

20/241

21/241

22/241

23/241

24/241

25/241

26/241

27/241

28/241

29/241

30/241

31/241

32/241

33/241

34/241

35/241

36/241

37/241

38/241

39/241

40/241

41/241

42/241

43/241

44/241

45/241

46/241

47/241

48/241

49/241

50/241

51/241

52/241

53/241

54/241

55/241

56/241

57/241

58/241

59/241

60/241

61/241

62/241

63/241

64/241

65/241

66/241

67/241

68/241

69/241

70/241

71/241

72/241

73/241

74/241

75/241

76/241

77/241

78/241

79/241

80/241

81/241

82/241

83/241

84/241

85/241

86/241

87/241

88/241

89/241

90/241

91/241

92/241

93/241

94/241

95/241

96/241

97/241

98/241
 ERROR wasp::testsuite > test 98/241 failed (module: 0, invoke: "params", got [i32(1)], but expected [i32(-1)])
```

## Failed: test-suite/test/core/labels.wast
```bash

1/29
1 2
1 18446744073709551615

2/29

3/29
 ERROR wasp::testsuite > test 3/29 failed (module: 0, invoke: "loop1", got [i32(1)], but expected [i32(5)])
```

## Failed: test-suite/test/core/linking.wast
```bash

1/132
1 2
1 18446744073709551615

2/132
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:615:21:
	not yet implemented: Register: "Mf" $Mf
Last test (1):
	Register(Register { _type: MustBe!("register"), name: Some("$Mf"), _as: "Mf" })
```

## Failed: test-suite/test/core/local_get.wast
```bash

1/36
1 2
1 18446744073709551615

2/36
 ERROR wasp::testsuite > test 2/36 failed (module: 0, invoke: "type-local-i32", error: a local is missing: src/runtime/methods/step.rs:536:51)
```

## Failed: test-suite/test/core/local_set.wast
```bash

1/53
1 2
1 18446744073709551615

2/53

3/53

4/53

5/53

6/53

7/53

8/53

9/53

10/53

11/53

12/53

13/53

14/53

15/53

16/53

17/53

18/53

19/53

20/53
 ERROR wasp::testsuite > test 20/53 failed (module: 0, invoke: "write", error: a local is missing: src/runtime/methods/step.rs:536:51)
```

## Failed: test-suite/test/core/local_tee.wast
```bash

1/97
1 2
1 18446744073709551615

2/97

3/97

4/97

5/97

6/97

7/97

8/97

9/97

10/97

11/97

12/97

13/97

14/97

15/97

16/97

17/97

18/97

19/97

20/97

21/97

22/97

23/97

24/97

25/97

26/97

27/97

28/97

29/97

30/97

31/97

32/97

33/97

34/97

35/97

36/97

37/97

38/97

39/97

40/97

41/97

42/97

43/97

44/97

45/97

46/97

47/97

48/97

49/97

50/97

51/97

52/97

53/97

54/97

55/97
 ERROR wasp::testsuite > test 55/97 failed (module: 0, invoke: "write", error: a local is missing: src/runtime/methods/step.rs:536:51)
```

## Failed: test-suite/test/core/loop.wast
```bash

1/120
1 2
1 18446744073709551615

2/120

3/120

4/120

5/120

6/120

7/120

8/120

9/120

10/120

11/120

12/120

13/120

14/120

15/120

16/120

17/120

18/120

19/120

20/120

21/120

22/120

23/120

24/120

25/120

26/120

27/120

28/120

29/120

30/120

31/120

32/120

33/120

34/120

35/120

36/120

37/120

38/120

39/120

40/120
 ERROR wasp::testsuite > test 40/120 failed (module: 0, invoke: "break-multi-value", got [i32(0), i32(0), i64(0)], but expected [i32(18), i32(-18), i64(18)])
```

## Failed: test-suite/test/core/memory.wast
```bash

1/88
1 2
0 18446744073709551615

2/88
1 2
1 18446744073709551615

3/88
1 2
0 0

4/88
1 2
0 1

5/88
1 2
1 256

6/88
1 2
0 65536

7/88
1 2

8/88
1 2
 ERROR wasp::testsuite > test 8/88 did not fail invalidating/parsing, expected error: "multiple memories" (module: "test-suite/test/core/memory.7.wasm")
```

## Failed: test-suite/test/core/memory_copy.wast
```bash

1/4450
1 2
1 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                2,
            ),
        ],
    },
    [
        3,
        1,
        4,
        1,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                12,
            ),
        ],
    },
    [
        7,
        5,
        2,
        3,
        6,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory

2/4450

3/4450

4/4450

5/4450

6/4450

7/4450

8/4450

9/4450

10/4450

11/4450

12/4450

13/4450

14/4450

15/4450

16/4450

17/4450

18/4450

19/4450

20/4450

21/4450

22/4450

23/4450

24/4450

25/4450

26/4450

27/4450

28/4450

29/4450

30/4450

31/4450

32/4450

33/4450
1 2
1 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                2,
            ),
        ],
    },
    [
        3,
        1,
        4,
        1,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                12,
            ),
        ],
    },
    [
        7,
        5,
        2,
        3,
        6,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory

34/4450

35/4450

36/4450

37/4450

38/4450

39/4450

40/4450

41/4450

42/4450

43/4450

44/4450

45/4450

46/4450

47/4450

48/4450

49/4450

50/4450

51/4450

52/4450

53/4450

54/4450

55/4450

56/4450

57/4450

58/4450

59/4450

60/4450

61/4450

62/4450

63/4450

64/4450

65/4450
1 2
1 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                2,
            ),
        ],
    },
    [
        3,
        1,
        4,
        1,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                12,
            ),
        ],
    },
    [
        7,
        5,
        2,
        3,
        6,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory

66/4450

67/4450

68/4450

69/4450

70/4450

71/4450

72/4450

73/4450

74/4450

75/4450

76/4450

77/4450

78/4450

79/4450

80/4450

81/4450

82/4450

83/4450

84/4450

85/4450

86/4450

87/4450

88/4450

89/4450

90/4450

91/4450

92/4450

93/4450

94/4450

95/4450

96/4450

97/4450
1 2
1 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                2,
            ),
        ],
    },
    [
        3,
        1,
        4,
        1,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                12,
            ),
        ],
    },
    [
        7,
        5,
        2,
        3,
        6,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory

98/4450

99/4450

100/4450

101/4450

102/4450

103/4450

104/4450

105/4450

106/4450

107/4450

108/4450

109/4450

110/4450

111/4450

112/4450

113/4450

114/4450

115/4450

116/4450

117/4450

118/4450

119/4450

120/4450

121/4450

122/4450

123/4450

124/4450

125/4450

126/4450

127/4450

128/4450

129/4450
1 2
1 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                2,
            ),
        ],
    },
    [
        3,
        1,
        4,
        1,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                12,
            ),
        ],
    },
    [
        7,
        5,
        2,
        3,
        6,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory

130/4450

131/4450

132/4450

133/4450

134/4450

135/4450

136/4450

137/4450

138/4450

139/4450

140/4450

141/4450

142/4450

143/4450

144/4450

145/4450

146/4450

147/4450

148/4450

149/4450

150/4450

151/4450

152/4450

153/4450

154/4450

155/4450

156/4450

157/4450

158/4450

159/4450

160/4450

161/4450
1 2
1 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                2,
            ),
        ],
    },
    [
        3,
        1,
        4,
        1,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                12,
            ),
        ],
    },
    [
        7,
        5,
        2,
        3,
        6,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory

162/4450

163/4450

164/4450

165/4450

166/4450

167/4450

168/4450

169/4450

170/4450

171/4450

172/4450

173/4450

174/4450

175/4450

176/4450

177/4450

178/4450

179/4450

180/4450

181/4450

182/4450

183/4450

184/4450

185/4450

186/4450

187/4450

188/4450

189/4450

190/4450

191/4450

192/4450

193/4450
1 2
1 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                2,
            ),
        ],
    },
    [
        3,
        1,
        4,
        1,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                12,
            ),
        ],
    },
    [
        7,
        5,
        2,
        3,
        6,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory

194/4450

195/4450

196/4450

197/4450

198/4450

199/4450

200/4450

201/4450

202/4450

203/4450

204/4450

205/4450

206/4450

207/4450

208/4450

209/4450

210/4450

211/4450

212/4450

213/4450

214/4450

215/4450

216/4450

217/4450

218/4450

219/4450

220/4450

221/4450

222/4450

223/4450

224/4450

225/4450
1 2
1 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                2,
            ),
        ],
    },
    [
        3,
        1,
        4,
        1,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                12,
            ),
        ],
    },
    [
        7,
        5,
        2,
        3,
        6,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory

226/4450

227/4450

228/4450

229/4450

230/4450

231/4450

232/4450

233/4450

234/4450

235/4450

236/4450

237/4450

238/4450

239/4450

240/4450

241/4450

242/4450

243/4450

244/4450

245/4450

246/4450

247/4450

248/4450

249/4450

250/4450

251/4450

252/4450

253/4450

254/4450

255/4450

256/4450

257/4450
1 2
1 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory

258/4450

259/4450

260/4450

261/4450

262/4450

263/4450

264/4450

265/4450

266/4450

267/4450

268/4450

269/4450

270/4450

271/4450

272/4450

273/4450

274/4450

275/4450

276/4450

277/4450

278/4450

279/4450

280/4450

281/4450

282/4450

283/4450

284/4450

285/4450

286/4450

287/4450

288/4450

289/4450

290/4450

291/4450

292/4450

293/4450

294/4450

295/4450

296/4450

297/4450

298/4450

299/4450

300/4450

301/4450

302/4450

303/4450

304/4450

305/4450

306/4450

307/4450

308/4450

309/4450

310/4450

311/4450

312/4450

313/4450

314/4450

315/4450

316/4450

317/4450

318/4450

319/4450

320/4450

321/4450

322/4450

323/4450

324/4450

325/4450

326/4450

327/4450

328/4450

329/4450

330/4450

331/4450

332/4450

333/4450

334/4450

335/4450

336/4450

337/4450

338/4450

339/4450

340/4450

341/4450

342/4450

343/4450

344/4450

345/4450

346/4450

347/4450

348/4450

349/4450

350/4450

351/4450

352/4450

353/4450

354/4450

355/4450

356/4450

357/4450

358/4450

359/4450

360/4450

361/4450

362/4450

363/4450

364/4450

365/4450

366/4450

367/4450

368/4450

369/4450

370/4450

371/4450

372/4450

373/4450

374/4450

375/4450

376/4450

377/4450

378/4450

379/4450

380/4450

381/4450

382/4450

383/4450

384/4450

385/4450

386/4450

387/4450

388/4450

389/4450

390/4450

391/4450

392/4450

393/4450

394/4450

395/4450

396/4450

397/4450

398/4450

399/4450

400/4450

401/4450

402/4450

403/4450

404/4450

405/4450

406/4450

407/4450

408/4450

409/4450

410/4450

411/4450

412/4450

413/4450

414/4450

415/4450

416/4450

417/4450

418/4450

419/4450

420/4450

421/4450

422/4450

423/4450

424/4450

425/4450

426/4450

427/4450

428/4450

429/4450

430/4450

431/4450

432/4450

433/4450

434/4450

435/4450

436/4450

437/4450

438/4450

439/4450

440/4450

441/4450

442/4450

443/4450

444/4450

445/4450

446/4450

447/4450

448/4450

449/4450

450/4450

451/4450

452/4450

453/4450

454/4450

455/4450

456/4450

457/4450

458/4450

459/4450

460/4450

461/4450

462/4450

463/4450

464/4450

465/4450

466/4450

467/4450

468/4450

469/4450

470/4450

471/4450

472/4450

473/4450

474/4450

475/4450

476/4450

477/4450

478/4450

479/4450

480/4450

481/4450

482/4450

483/4450

484/4450

485/4450

486/4450

487/4450

488/4450

489/4450

490/4450

491/4450

492/4450

493/4450

494/4450

495/4450

496/4450

497/4450

498/4450

499/4450

500/4450

501/4450

502/4450

503/4450

504/4450

505/4450

506/4450

507/4450

508/4450

509/4450

510/4450

511/4450

512/4450

513/4450

514/4450

515/4450

516/4450

517/4450

518/4450

519/4450

520/4450

521/4450

522/4450

523/4450

524/4450

525/4450

526/4450

527/4450

528/4450

529/4450

530/4450

531/4450

532/4450

533/4450

534/4450

535/4450

536/4450

537/4450

538/4450

539/4450

540/4450

541/4450

542/4450

543/4450

544/4450

545/4450

546/4450

547/4450

548/4450

549/4450

550/4450

551/4450

552/4450

553/4450

554/4450

555/4450

556/4450

557/4450

558/4450

559/4450

560/4450

561/4450

562/4450

563/4450

564/4450

565/4450

566/4450

567/4450

568/4450

569/4450

570/4450

571/4450

572/4450

573/4450

574/4450

575/4450

576/4450

577/4450

578/4450

579/4450

580/4450

581/4450

582/4450

583/4450

584/4450

585/4450

586/4450

587/4450

588/4450

589/4450

590/4450

591/4450

592/4450

593/4450

594/4450

595/4450

596/4450

597/4450

598/4450

599/4450

600/4450

601/4450

602/4450

603/4450

604/4450

605/4450

606/4450

607/4450

608/4450
1 2
1 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory

609/4450

610/4450

611/4450

612/4450

613/4450

614/4450

615/4450

616/4450

617/4450

618/4450

619/4450

620/4450

621/4450

622/4450

623/4450

624/4450

625/4450

626/4450

627/4450

628/4450

629/4450

630/4450

631/4450

632/4450

633/4450

634/4450

635/4450

636/4450

637/4450

638/4450

639/4450

640/4450

641/4450

642/4450

643/4450

644/4450

645/4450

646/4450

647/4450

648/4450

649/4450

650/4450

651/4450

652/4450

653/4450

654/4450

655/4450

656/4450

657/4450

658/4450

659/4450

660/4450

661/4450

662/4450

663/4450

664/4450

665/4450

666/4450

667/4450

668/4450

669/4450

670/4450

671/4450

672/4450

673/4450

674/4450

675/4450

676/4450

677/4450

678/4450

679/4450

680/4450

681/4450

682/4450

683/4450

684/4450

685/4450

686/4450

687/4450

688/4450

689/4450

690/4450

691/4450

692/4450

693/4450

694/4450

695/4450

696/4450

697/4450

698/4450

699/4450

700/4450

701/4450

702/4450

703/4450

704/4450

705/4450

706/4450

707/4450

708/4450

709/4450

710/4450

711/4450

712/4450

713/4450

714/4450

715/4450

716/4450

717/4450

718/4450

719/4450

720/4450

721/4450

722/4450

723/4450

724/4450

725/4450

726/4450

727/4450

728/4450

729/4450

730/4450

731/4450

732/4450

733/4450

734/4450

735/4450

736/4450

737/4450

738/4450

739/4450

740/4450

741/4450

742/4450

743/4450

744/4450

745/4450

746/4450

747/4450

748/4450

749/4450

750/4450

751/4450

752/4450

753/4450

754/4450

755/4450

756/4450

757/4450

758/4450

759/4450

760/4450

761/4450

762/4450

763/4450

764/4450

765/4450

766/4450

767/4450

768/4450

769/4450

770/4450

771/4450

772/4450

773/4450

774/4450

775/4450

776/4450

777/4450

778/4450

779/4450

780/4450

781/4450

782/4450

783/4450

784/4450

785/4450

786/4450

787/4450

788/4450

789/4450

790/4450

791/4450

792/4450

793/4450

794/4450

795/4450

796/4450

797/4450

798/4450

799/4450

800/4450

801/4450

802/4450

803/4450

804/4450

805/4450

806/4450

807/4450

808/4450

809/4450

810/4450

811/4450

812/4450

813/4450

814/4450

815/4450

816/4450

817/4450

818/4450

819/4450

820/4450

821/4450

822/4450

823/4450

824/4450

825/4450

826/4450

827/4450

828/4450

829/4450

830/4450

831/4450

832/4450

833/4450

834/4450

835/4450

836/4450

837/4450

838/4450

839/4450

840/4450

841/4450

842/4450

843/4450

844/4450

845/4450

846/4450

847/4450

848/4450

849/4450

850/4450

851/4450

852/4450

853/4450

854/4450

855/4450

856/4450

857/4450

858/4450

859/4450

860/4450

861/4450

862/4450

863/4450

864/4450

865/4450

866/4450

867/4450

868/4450

869/4450

870/4450

871/4450

872/4450

873/4450

874/4450

875/4450

876/4450

877/4450

878/4450

879/4450

880/4450

881/4450

882/4450

883/4450

884/4450

885/4450

886/4450

887/4450

888/4450

889/4450

890/4450

891/4450

892/4450

893/4450

894/4450

895/4450

896/4450

897/4450

898/4450

899/4450

900/4450

901/4450

902/4450

903/4450

904/4450

905/4450

906/4450

907/4450

908/4450

909/4450

910/4450

911/4450

912/4450

913/4450

914/4450

915/4450

916/4450

917/4450

918/4450

919/4450

920/4450

921/4450

922/4450

923/4450

924/4450

925/4450

926/4450

927/4450

928/4450

929/4450

930/4450

931/4450

932/4450

933/4450

934/4450

935/4450

936/4450

937/4450

938/4450

939/4450

940/4450

941/4450

942/4450

943/4450

944/4450

945/4450

946/4450

947/4450

948/4450

949/4450

950/4450

951/4450

952/4450

953/4450

954/4450

955/4450

956/4450

957/4450

958/4450

959/4450

960/4450
1 2
1 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                65516,
            ),
        ],
    },
    [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory
setting memory

961/4450
 ERROR wasp::testsuite > test 961/4450 did not fail, expected error: "out of bounds memory access" (module: 10, function "run")
```

## Failed: test-suite/test/core/memory_fill.wast
```bash

1/100
1 2
1 1

2/100

3/100

4/100

5/100
1 2
1 1

6/100

7/100
1 2
1 1

8/100
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/runtime/memory.rs:209:27:
	attempt to add with overflow
Last test (7):
	AssertTrap(AssertTrap { _type: MustBe!("assert_trap"), action: Invoke { module: None, field: "test", args: [] }, text: "out of bounds memory access" })
```

## Failed: test-suite/test/core/memory_grow.wast
```bash

1/104
1 2
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:374:32:
	failed to load module: ParseError(File: "test-suite/test/core/memory_grow.0.wasm"
	UnknownInstruction(<3f>), bin pos: 179, stack: [
	    "wasp::parser::instr::Instr",
	    "wasp::parser::expr::Expr",
	    "wasp::parser::func::Func",
	    "wasp::parser::code::Code",
	    "alloc::vec::Vec<wasp::parser::code::Code>",
	    "wasp::parser::codesec::CodeSection",
	    "wasp::parser::module::Module",
	])
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "memory_grow.0.wasm" })
```

## Failed: test-suite/test/core/memory_init.wast
```bash

1/240
1 2
1 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                2,
            ),
        ],
    },
    [
        3,
        1,
        4,
        1,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Passive(
    [
        2,
        7,
        1,
        8,
    ],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                12,
            ),
        ],
    },
    [
        7,
        5,
        2,
        3,
        6,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Passive(
    [
        5,
        9,
        2,
        7,
        6,
    ],
)

2/240

3/240

4/240

5/240

6/240

7/240

8/240

9/240

10/240

11/240

12/240

13/240

14/240

15/240

16/240

17/240

18/240

19/240

20/240

21/240

22/240

23/240

24/240

25/240

26/240

27/240

28/240

29/240

30/240

31/240

32/240

33/240
1 2
1 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                2,
            ),
        ],
    },
    [
        3,
        1,
        4,
        1,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Passive(
    [
        2,
        7,
        1,
        8,
    ],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                12,
            ),
        ],
    },
    [
        7,
        5,
        2,
        3,
        6,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Passive(
    [
        5,
        9,
        2,
        7,
        6,
    ],
)

34/240

35/240

36/240

37/240

38/240

39/240

40/240

41/240

42/240

43/240

44/240

45/240

46/240

47/240

48/240

49/240

50/240

51/240

52/240

53/240

54/240

55/240

56/240

57/240

58/240

59/240

60/240

61/240

62/240

63/240

64/240

65/240
1 2
1 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                2,
            ),
        ],
    },
    [
        3,
        1,
        4,
        1,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Passive(
    [
        2,
        7,
        1,
        8,
    ],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                12,
            ),
        ],
    },
    [
        7,
        5,
        2,
        3,
        6,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Passive(
    [
        5,
        9,
        2,
        7,
        6,
    ],
)

66/240

67/240

68/240

69/240

70/240

71/240

72/240

73/240

74/240

75/240

76/240

77/240

78/240

79/240

80/240

81/240

82/240

83/240

84/240

85/240

86/240

87/240

88/240

89/240

90/240

91/240

92/240

93/240

94/240

95/240

96/240

97/240
1 2
1 1
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                2,
            ),
        ],
    },
    [
        3,
        1,
        4,
        1,
    ],
)
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Passive(
    [
        2,
        7,
        1,
        8,
    ],
)
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                12,
            ),
        ],
    },
    [
        7,
        5,
        2,
        3,
        6,
    ],
)
setting memory
setting memory
setting memory
setting memory
setting memory
[src/runtime/clean_model.rs:707:15] d = Passive(
    [
        5,
        9,
        2,
        7,
        6,
    ],
)

98/240

99/240

100/240

101/240

102/240

103/240

104/240

105/240

106/240

107/240

108/240

109/240

110/240

111/240

112/240

113/240

114/240

115/240

116/240

117/240

118/240

119/240

120/240

121/240

122/240

123/240

124/240

125/240

126/240

127/240

128/240

129/240
1 2

130/240
1 2
1 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Passive(
    [
        55,
    ],
)
 ERROR wasp::testsuite > test 130/240 did not fail invalidating/parsing, expected error: "unknown data segment" (module: "test-suite/test/core/memory_init.5.wasm")
```

## Failed: test-suite/test/core/memory_size.wast
```bash

1/42
1 2
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:374:32:
	failed to load module: ParseError(File: "test-suite/test/core/memory_size.0.wasm"
	UnknownInstruction(<3f>), bin pos: 52, stack: [
	    "wasp::parser::instr::Instr",
	    "wasp::parser::expr::Expr",
	    "wasp::parser::func::Func",
	    "wasp::parser::code::Code",
	    "alloc::vec::Vec<wasp::parser::code::Code>",
	    "wasp::parser::codesec::CodeSection",
	    "wasp::parser::module::Module",
	])
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "memory_size.0.wasm" })
```

## Failed: test-suite/test/core/memory_trap.wast
```bash

1/182
1 2
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:374:32:
	failed to load module: ParseError(File: "test-suite/test/core/memory_trap.0.wasm"
	UnknownInstruction(<3f>), bin pos: 75, stack: [
	    "wasp::parser::instr::Instr",
	    "wasp::parser::expr::Expr",
	    "wasp::parser::func::Func",
	    "wasp::parser::code::Code",
	    "alloc::vec::Vec<wasp::parser::code::Code>",
	    "wasp::parser::codesec::CodeSection",
	    "wasp::parser::module::Module",
	])
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "memory_trap.0.wasm" })
```

## Failed: test-suite/test/core/ref_func.wast
```bash

1/17
1 2
1 18446744073709551615

2/17

3/17
1 2
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:374:32:
	failed to load module: ParseError(File: "test-suite/test/core/ref_func.1.wasm"
	UnknownInstruction(<d1>), bin pos: 220, stack: [
	    "wasp::parser::instr::Instr",
	    "wasp::parser::expr::Expr",
	    "wasp::parser::func::Func",
	    "wasp::parser::code::Code",
	    "alloc::vec::Vec<wasp::parser::code::Code>",
	    "wasp::parser::codesec::CodeSection",
	    "wasp::parser::module::Module",
	])
Last test (2):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "ref_func.1.wasm" })
```

## Failed: test-suite/test/core/ref_is_null.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:333:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 2)
Last test (0):
	Default
```

## Failed: test-suite/test/core/ref_null.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:333:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 5, column: 154)
Last test (0):
	Default
```

## Failed: test-suite/test/core/return.wast
```bash

1/84
1 2
1 18446744073709551615

2/84

3/84

4/84

5/84

6/84

7/84

8/84
 ERROR wasp::testsuite > test 8/84 failed (module: 0, invoke: "type-f32-value", got [--- BLOCK ---, f32(3)], but expected [i32(1073741824)])
```

## Failed: test-suite/test/core/select.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:333:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 33, column: 2)
Last test (0):
	Default
```

## Failed: test-suite/test/core/simd_address.wast
```bash
```

## Failed: test-suite/test/core/simd_align.wast
```bash
```

## Failed: test-suite/test/core/simd_bit_shift.wast
```bash
```

## Failed: test-suite/test/core/simd_bitwise.wast
```bash
```

## Failed: test-suite/test/core/simd_boolean.wast
```bash
```

## Failed: test-suite/test/core/simd_const.wast
```bash
```

## Failed: test-suite/test/core/simd_conversions.wast
```bash
```

## Failed: test-suite/test/core/simd_f32x4.wast
```bash
```

## Failed: test-suite/test/core/simd_f32x4_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_f32x4_cmp.wast
```bash
```

## Failed: test-suite/test/core/simd_f32x4_pmin_pmax.wast
```bash
```

## Failed: test-suite/test/core/simd_f32x4_rounding.wast
```bash
```

## Failed: test-suite/test/core/simd_f64x2.wast
```bash
```

## Failed: test-suite/test/core/simd_f64x2_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_f64x2_cmp.wast
```bash
```

## Failed: test-suite/test/core/simd_f64x2_pmin_pmax.wast
```bash
```

## Failed: test-suite/test/core/simd_f64x2_rounding.wast
```bash
```

## Failed: test-suite/test/core/simd_i16x8_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_i16x8_arith2.wast
```bash
```

## Failed: test-suite/test/core/simd_i16x8_cmp.wast
```bash
```

## Failed: test-suite/test/core/simd_i16x8_extadd_pairwise_i8x16.wast
```bash
```

## Failed: test-suite/test/core/simd_i16x8_extmul_i8x16.wast
```bash
```

## Failed: test-suite/test/core/simd_i16x8_q15mulr_sat_s.wast
```bash
```

## Failed: test-suite/test/core/simd_i16x8_sat_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_arith2.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_cmp.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_dot_i16x8.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_extadd_pairwise_i16x8.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_extmul_i16x8.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_trunc_sat_f32x4.wast
```bash
```

## Failed: test-suite/test/core/simd_i32x4_trunc_sat_f64x2.wast
```bash
```

## Failed: test-suite/test/core/simd_i64x2_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_i64x2_arith2.wast
```bash
```

## Failed: test-suite/test/core/simd_i64x2_cmp.wast
```bash
```

## Failed: test-suite/test/core/simd_i64x2_extmul_i32x4.wast
```bash
```

## Failed: test-suite/test/core/simd_i8x16_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_i8x16_arith2.wast
```bash
```

## Failed: test-suite/test/core/simd_i8x16_cmp.wast
```bash
```

## Failed: test-suite/test/core/simd_i8x16_sat_arith.wast
```bash
```

## Failed: test-suite/test/core/simd_int_to_int_extend.wast
```bash
```

## Failed: test-suite/test/core/simd_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_linking.wast
```bash
```

## Failed: test-suite/test/core/simd_load.wast
```bash
```

## Failed: test-suite/test/core/simd_load16_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_load32_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_load64_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_load8_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_load_extend.wast
```bash
```

## Failed: test-suite/test/core/simd_load_splat.wast
```bash
```

## Failed: test-suite/test/core/simd_load_zero.wast
```bash
```

## Failed: test-suite/test/core/simd_splat.wast
```bash
```

## Failed: test-suite/test/core/simd_store.wast
```bash
```

## Failed: test-suite/test/core/simd_store16_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_store32_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_store64_lane.wast
```bash
```

## Failed: test-suite/test/core/simd_store8_lane.wast
```bash
```

## Failed: test-suite/test/core/start.wast
```bash

1/20
1 2
1 18446744073709551615
 ERROR wasp::testsuite > test 1/20 did not fail invalidating/parsing, expected error: "unknown function" (module: "test-suite/test/core/start.0.wasm")
```

## Failed: test-suite/test/core/table.wast
```bash

1/19
1 2
1 18446744073709551615

2/19
1 2
1 18446744073709551615

3/19
1 2
1 18446744073709551615

4/19
1 2
1 18446744073709551615

5/19
1 2
1 18446744073709551615

6/19
1 2
1 18446744073709551615

7/19
1 2
memory allocation of 77309411344 bytes failed
```

## Failed: test-suite/test/core/table_copy.wast
```bash

1/1728
1 2
1 18446744073709551615

2/1728

3/1728
1 2
1 18446744073709551615
1 18446744073709551615

4/1728

5/1728

6/1728

7/1728

8/1728

9/1728

10/1728

11/1728

12/1728

13/1728

14/1728

15/1728

16/1728

17/1728

18/1728

19/1728

20/1728

21/1728

22/1728

23/1728

24/1728

25/1728

26/1728

27/1728

28/1728

29/1728

30/1728

31/1728

32/1728

33/1728

34/1728

35/1728

36/1728

37/1728

38/1728

39/1728

40/1728

41/1728

42/1728

43/1728

44/1728

45/1728

46/1728

47/1728

48/1728

49/1728

50/1728

51/1728

52/1728

53/1728

54/1728

55/1728

56/1728

57/1728

58/1728

59/1728

60/1728

61/1728

62/1728

63/1728

64/1728

65/1728
1 2
1 18446744073709551615
1 18446744073709551615

66/1728
 ERROR wasp::testsuite > test 66/1728 failed: out of bounds table access (module: 2, invoke: "test")
```

## Failed: test-suite/test/core/table_fill.wast
```bash

1/45
1 2
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:374:32:
	failed to load module: ParseError(File: "test-suite/test/core/table_fill.0.wasm"
	UnknownInstruction(<25>), bin pos: 96, stack: [
	    "wasp::parser::instr::Instr",
	    "wasp::parser::expr::Expr",
	    "wasp::parser::func::Func",
	    "wasp::parser::code::Code",
	    "alloc::vec::Vec<wasp::parser::code::Code>",
	    "wasp::parser::codesec::CodeSection",
	    "wasp::parser::module::Module",
	])
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "table_fill.0.wasm" })
```

## Failed: test-suite/test/core/table_get.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:333:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 8, column: 2)
Last test (0):
	Default
```

## Failed: test-suite/test/core/table_grow.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:333:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 44, column: 2)
Last test (0):
	Default
```

## Failed: test-suite/test/core/table_init.wast
```bash

1/780
1 2
1 18446744073709551615

2/780

3/780
1 2
1 18446744073709551615
1 18446744073709551615

4/780

5/780

6/780

7/780

8/780

9/780

10/780

11/780

12/780

13/780

14/780

15/780

16/780

17/780

18/780

19/780

20/780

21/780

22/780

23/780

24/780

25/780

26/780

27/780

28/780

29/780

30/780

31/780

32/780

33/780

34/780

35/780
1 2
1 18446744073709551615
1 18446744073709551615

36/780

37/780

38/780

39/780

40/780

41/780

42/780

43/780

44/780

45/780

46/780

47/780

48/780

49/780

50/780

51/780

52/780

53/780

54/780

55/780

56/780

57/780

58/780

59/780

60/780

61/780

62/780

63/780

64/780

65/780

66/780

67/780
1 2
1 18446744073709551615
1 18446744073709551615

68/780
 ERROR wasp::testsuite > test 68/780 failed: out of bounds table access (module: 3, invoke: "test")
```

## Failed: test-suite/test/core/table_set.wast
```bash
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:333:6:
	failed to parse test data: Error("data did not match any variant of untagged enum Case", line: 10, column: 2)
Last test (0):
	Default
```

## Failed: test-suite/test/core/table_size.wast
```bash

1/39
1 2
1 18446744073709551615

2/39
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/runtime/methods/step.rs:1747:17:
	not implemented: instruction not supported : xfc_16_table_size(TableIdX(0))
Last test (1):
	AssertReturn(AssertReturn { _type: MustBe!("assert_return"), action: Invoke { module: None, field: "size-t0", args: [] }, expected: [I32 { value: "0" }] })
```

## Failed: test-suite/test/core/token.wast
```bash

1/58

2/58

3/58
1 2
1 18446744073709551615

4/58
1 2
1 18446744073709551615

5/58
1 2
1 18446744073709551615

6/58
1 2
1 18446744073709551615

7/58
1 2
1 18446744073709551615

8/58
1 2
1 18446744073709551615

9/58
1 2
1 18446744073709551615

10/58
1 2
1 18446744073709551615

11/58
1 2
1 18446744073709551615
[src/runtime/clean_model.rs:707:15] d = Active(
    Expr {
        instrs: [
            x41_i32_const(
                0,
            ),
        ],
    },
    [
        97,
    ],
)
setting memory

12/58
1 2
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:374:32:
	failed to load module: unknown import: src/runtime/clean_model.rs:122:82
Last test (11):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "token.11.wasm" })
```

## Failed: test-suite/test/core/traps.wast
```bash

1/36
1 2
1 18446744073709551615

2/36

3/36

4/36

5/36

6/36

7/36

8/36
1 2
1 18446744073709551615

9/36

10/36

11/36

12/36

13/36
1 2
1 18446744073709551615

14/36

15/36

16/36

17/36

18/36

19/36

20/36

21/36

22/36
1 2
1 18446744073709551615

23/36
 ERROR wasp::testsuite > test 23/36 did not fail, expected error: "out of bounds memory access" (module: 3, function "no_dce.i32.load")
```

## Failed: test-suite/test/core/unreachable.wast
```bash

1/64
1 2
1 18446744073709551615

2/64

3/64

4/64

5/64

6/64

7/64

8/64

9/64

10/64

11/64

12/64

13/64

14/64

15/64

16/64

17/64

18/64

19/64

20/64

21/64

22/64

23/64

24/64

25/64

26/64

27/64

28/64

29/64

30/64

31/64

32/64

33/64

34/64
 ERROR wasp::testsuite > test 34/64 did not fail, expected error: "unreachable" (module: 0, function "as-if-then-no-else")
```

## Failed: test-suite/test/core/unreached-invalid.wast
```bash

1/118
1 2
1 18446744073709551615
 ERROR wasp::testsuite > test 1/118 did not fail invalidating/parsing, expected error: "unknown local" (module: "test-suite/test/core/unreached-invalid.0.wasm")
```

## Failed: test-suite/test/core/unreached-valid.wast
```bash

1/7
1 2
 ERROR wasp::testsuite > oops the test-suite panicked!
Reason:
	panicked at src/testsuite.rs:374:32:
	failed to load module: ParseError(File: "test-suite/test/core/unreached-valid.0.wasm"
	UnknownInstruction(<d1>), bin pos: 273, stack: [
	    "wasp::parser::instr::Instr",
	    "wasp::parser::expr::Expr",
	    "wasp::parser::func::Func",
	    "wasp::parser::code::Code",
	    "alloc::vec::Vec<wasp::parser::code::Code>",
	    "wasp::parser::codesec::CodeSection",
	    "wasp::parser::module::Module",
	])
Last test (0):
	Module(Module { _type: MustBe!("module"), _name: None, filename: "unreached-valid.0.wasm" })
```

## Failed: test-suite/test/core/unwind.wast
```bash

1/50
1 2
1 18446744073709551615

2/50

3/50
 ERROR wasp::testsuite > test 3/50 failed (module: 0, invoke: "func-unwind-by-br", error: missing jump label: src/runtime/methods/step.rs:292:26)
```

## Failed: test-suite/test/core/utf8-custom-section-id.wast
```bash

1/176
1 2
1 18446744073709551615
 ERROR wasp::testsuite > test 1/176 did not fail invalidating/parsing, expected error: "malformed UTF-8 encoding" (module: "test-suite/test/core/utf8-custom-section-id.0.wasm")
```

