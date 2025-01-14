
# Summary
GCAD (short for Geometry CAD) is a text-based CAD format based on geometry notation designed for use with sketch-less geometric constraint solvers.

# Syntax

GCAD uses significant whitespace. Line breaks end statements and spaces separate the names of points.

GCAD defines two dialects: proper (Unicode) and improper (ASCII).

 - Proper GCAD uses the correct symbols from Unicode whenever possible.
 - Improper GCAD uses approximate symbols constructed of ASCII for type-ability.

# Reference
```
                    Proper        Improper     
 ═══════════════╤═════════════╤══════════════ 
  Distance      ╷ |A B| = 1   ╷              
  Angle         ╷ ∠A B C = 1  ╷  <A B C = 1   
  Parallel      ╷ A B ∥ C D   ╷  A B || C D   
  Perpendicular ╷ A B ⟂ C D   ╷  A B _|_ C D  
  Collinear     ╷ A-B-C       ╷              
```

