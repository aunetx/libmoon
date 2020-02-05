var: &i, int

Variable used to check if == 0
var: &check, int

set: &i, 0
flg: a
LOOP FROM HERE
add: &i, 1
prt: &i
prt: hello

CHECK PART
set: &check, 10
sub: &check, &i
jne: &check, a
TO HERE