# Language structure

## Line structure

A line is defined as : `instruction: operand` or `instruction: operand1, operand2` depending of the nature of the instruction.

The instructions are the following :

Instructions :

- `var: var, type`                       CREATE VARIABLE
- `set: var, (var|value)`                SET VARIABLE VALUE
- `add: var, (var|value)`                ADD VALUE TO VARIABLE
- `sub: var, (var|value)`                SUBTRACT VALUE TO VARIABLE
- `mul: var, (var|value)`                MULTIPLY VALUE TO VARIABLE
- `div: var, (var|value)`                DIVIDE VALUE WITH VARIABLE
- `rst: var, (var|value)`                DIVIDE VALUE WITH VARIABLE (gives the rest)
- `ret: var`                             `set: var, _res` -> SET res VARIABLE INTO VAR
- `gto: flag`                            GO TO INSTRUCTION
- `jmp: var, flag`                       IF op1 IS 0 JUMP TO op2 FLAG
- `jne: var, flag`                       IF op1 IS NOT 0 JUMP TO op2 FLAG
- `flg: flag`                            CREATE FLAG
- `nll: empty`                           DO NOTHING AND IS IGNORED BY INTERPRETER
- `prt: (var|value)`                     PRINT VALUE TO THE SCREEN

The possible types of operands are :

- `var` -> a variable name
- `value` -> a hard-coded value of type : `int`, `flt` or `chr`
- `(var|value)` -> either a `var` or a `value`, depending of the operand nature (if `var`, begins with `&`)
- `type` -> a variable type : `int`, `flt`, `chr`
- `flag` -> a flag name

`pas`: pause the execution until restored?
