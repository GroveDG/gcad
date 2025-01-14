
# Summary
GCAD (short for Geometry CAD) is a text-based CAD format based on geometry notation designed for use with sketch-less geometric constraint solvers.

# Syntax

GCAD uses significant whitespace. Line breaks end statements and spaces separate the names of points.

GCAD defines two dialects: proper (Unicode) and improper (ASCII).

 - Proper GCAD uses the correct symbols from Unicode whenever possible.
 - Improper GCAD uses approximate symbols constructed of ASCII for type-ability.

# Reference
|               | Proper       | Improper      |
|---------------|--------------|---------------|
| Distance      | <code>`|A B| = 1`</code>  |               |
| Angle         | <code>`∠A B C = 1`</code> | <code>`<A B C = 1`</code>  |
| Parallel      | <code>`A B ∥ C D`</code>  | <code>`A B || C D`</code>  |
| Perpendicular | <code>`A B ⟂ C D`</code>  | <code>`A B _|_ C D`</code> |
| Collinear     | <code>`A-B-C`</code>      |               |

