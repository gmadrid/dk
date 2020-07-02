#### making a language...

basically need two things:
- assignment
- call operations

foo = read("filename")
trimmed = trim(foo, 2)
newish = pad(trimmed, h=3, v=2)

write("outfile")
write_chart("chartname")

Types:
  string = '"' .* '"'  // no escapes to start
  number = [0-9]+
  ident = [_a-zA-Z][_a-zA-Z0-9]*
  bool  = 'true' | 'false'
  
Terminals:
  STRING
  NUMBER
  IDENT
  BOOL
  EQ
  LPAREN
  RPAREN
  COMMA
  
```  
Non-terminals
  program = stmts
  stmts = stmt stmts
        = <empty>
  stmt = assign
       = call
  assign = variable EQ call
  call = ident LPAREN args RPAREN
  variable = ident
  args = args COMMA arg
       = arg
  arg = ident argtail
  argtail = EQ value
          = <empty>
  value = ident
        = number
        = string
        = bool
```
  