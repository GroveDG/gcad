
# Summary
GCAD (short for Geometry CAD) is a text-based CAD format based on geometry notation designed for use with sketch-less geometric constraint solvers.

# Quick Reference
```
 ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
  CONSTRAINTS         Proper          Improper    
 ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┬┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┬┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
  Distance      ╷ |A B| = 1      ╷              
  Angle         ╷ ∠A B C = 1     ╷ <A B C = 1   
  Parallel      ╷ A B ∥ C D  ... ╷ A B || C D  ...
  Perpendicular ╷ A B ⟂ C D  ... ╷ A B _|_ C D ...
  Collinear     ╷ A-B-C-     ... ╷
  Chirality     ╷ ±∠A B C,       ╷ +/-<A B C,    
                ╷ ∓∠A B C,   ... ╷ -/+<A B C,  ...
 ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
  DRAWING             Proper          Improper
 ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┬┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┬┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
  Line          ╷ A→B            ╷ A->B
  Quadratic     ╷ A-B→C          ╷ A-B->C
  Cubic         ╷ A-B-C→D        ╷ A-B-C->D
 ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
```

# Syntax

GCAD makes use of mathematical symbols commonly used in notating geometry. This includes "∠" for notating angles, "∥" for notating parallel lines, and "⟂" for notating perpendicular lines.

Since these Unicode symbols are not on keyboards, an alternate "improper" notation is made using ASCII approximations:
- "∠" becomes "<"
- "∥" becomes "||"
- "⟂" becomes "_|_"

There are also unique notations created for GCAD, like "-" to denote collinear points.

GCAD uses significant whitespace. Line breaks end statements and spaces separate the names of points.

Comments are enclosed in quotation marks (") with whitespace allowed before and after.


# Expressions
## Constraints
Most GCAD constraints should be familiar to those who remember geometry
class (that is the point after all). However you might be unfamiliar
with Collinear and Chirality.

Collinear states that all specified points are on the same line. Any two points are collinear by definition, so Collinear constraints must have at least 3 points.

Chirality states that all specified angles marked with "±" or "+/-" (Pro) have the opposite sign from angles marked with "∓" or "-/+" (Anti). Since GCAD does not use signed angles, this is used to relate the directions of angles.
- For example, all interior angles of a convex polygon have the same sign and any exterior angles have the opposite sign.
## Drawing
How drawing is implemented is dependent on the output format. GCAD drawing is intended for use with SVG and compatible path standards.

A path is drawn by chaining path segments together.
- For example, "A→B" or "A->B" draws a line from A to B, "B→C" or "B->C" draws a line from B to C, "A→B→C" or "A->B->C" draws a line from A to B to C.

If a path ends where it begins, the path should be closed if possible.
## Comments
Anything enclosed in double quotes (") is commented out. This mirrors document writing where text is presumed to be plain unless it is enclosed in a math field. Here text is presumed to be math unless it is enclosed in a text field (quotes).

Inline comments are currently unimplemented.

# Solving
See [gsolve](https://github.com/GroveDG/gsolve).

# TODO

- Math
- Figure orientation
- Figure positioning
- Cropping undrawn points
- Right-to-left text support
  - i.e. "<-" instead of "->"
