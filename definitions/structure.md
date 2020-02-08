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
- `mod: var, (var|value)`                DIVIDE VALUE WITH VARIABLE (gives the rest)
- `cadd: (var|value), (var|value)`                ADD VALUE TO VARIABLE INTO CARRY
- `csub: (var|value), (var|value)`                SUBTRACT VALUE TO VARIABLE INTO CARRY
- `cmul: (var|value), (var|value)`                MULTIPLY VALUE TO VARIABLE INTO CARRY
- `cdiv: (var|value), (var|value)`                DIVIDE VALUE WITH VARIABLE INTO CARRY
- `cmod: (var|value), (var|value)`                DIVIDE VALUE WITH VARIABLE (gives the rest) INTO CARRY
- `gto: flag`                            GO TO INSTRUCTION
- `jmp: var, flag`                       IF var IS 0 JUMP TO flag
- `jne: var, flag`                       IF var IS NOT 0 JUMP TO flag
- `flg: flag`                            CREATE FLAG
- `nll: nll`                             DO NOTHING AND IS IGNORED
- `prt: (var|value)`                     PRINT VALUE TO THE SCREEN

The possible types of operands are :

- `var` -> a variable name, preceded with `&` and containing only `a..z`, `A..Z`, `0..9` or `_` (e.g. `&my_var2`) OR the name `-` reffering to the carry variable
- `value` -> a hard-coded value of type : `int`, `flt` or `chr`
- `(var|value)` -> either a `var` or a `value`, depending of the operand nature (if `var`, begins with `&`) : type of value is inferred by first operand of the instruction
- `type` -> a variable type : `int`, `flt`, `chr`
- `flag` -> a flag name
- `nll` -> a null operand, used only for `nll` instruction (should not be used)

Comments can be written with plain-text, but without `:` (if this token is present, the current line will be parsed as an instruction)

The variable `-` is special : it is the "carry" variable.
It is overwrote by arithmetic operations preceded by `c` : `cadd`, `cmul`...
They are useful to check value of a variable for example, when we do not want to change its value.
Also, those alternative does not require `op1` to be a variable : it can also be a real value.

`pas`: pause the execution until restored?

## Operations

Operations `add`, `sub`, `mul`, `div` and `mod` work this way :

1. the interpreter checks the op1 and op2 have same type
2. it performs the operation : `op1 {operation} op2`, e.g. `op1 + op2` or `op1 / op2`
3. the result is placed in `op1`

The carry alternatives, namely `cadd`, `csub`, `cmul`, `cdiv` and `cmod`, work this way :

1. the interpreter checks the op1 and op2 have same type
2. it performs the operation : `op1 {operation} op2`, e.g. `op1 + op2` or `op1 / op2`
3. the type of the variable named `-` is changed to correspond to `op2`'s type
4. the result is placed into variable `-`
