var: &a, int
set: &a, 0
var: &b, int
set: &b, 0
var: &tmp, int
flg: table1

flg: table2

set: &tmp, &a
mul: &a, &b
prt: &a
set: &a, &tmp

add: &b, 1

ERROR HERE, WE WANT TO BE ABLE TO SUBSTRACT A VARIABLE TO A GIVEN VALUE
sub: 11, &a
jne: &_res, table1