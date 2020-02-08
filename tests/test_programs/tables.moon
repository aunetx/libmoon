var: &a, int
set: &a, 0
var: &b, int
set: &b, 0

flg: table1

flg: table2

cmul: &a, &b
prt: -

add: &b, 1

csub: 11, &b
jne: -, table2

set: &b, 0

add: &a, 1

csub: 11, &a
jne: -, table1