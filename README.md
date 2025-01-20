
# Summary
GCAD (short for Geometry CAD) is a text-based CAD format based on geometry notation designed for use with sketch-less geometric constraint solvers.

# Quick Reference
```
                    Proper          Improper     
 ═══════════════╤══════════════╤═════════════════
  Distance      ╷ |A B| = 1    ╷              
  Angle         ╷ ∠A B C = 1   ╷ <A B C = 1   
  Parallel      ╷ A B ∥ C D    ╷ A B || C D   
  Perpendicular ╷ A B ⟂ C D    ╷ A B _|_ C D  
  Collinear     ╷ A-B-C        ╷
  Polarity      ╷ ±∠A B C, ... ╷ +/-<A B C, ...
```

# Syntax

GCAD makes use of mathematical symbols commonly used in notating geometry. This includes "∠" for notating angles, "∥" for notating parallel lines, and "⟂" for notating perpendicular lines.

Since these Unicode symbols are not on keyboards, an alternate "improper" notation is made using ASCII approximations:
- "∠" becomes "<"
- "∥" becomes "||"
- "⟂" becomes "_|_"

There are also unique notations created for GCAD, like "-" to denote collinear points.

GCAD uses significant whitespace. Line breaks end statements and spaces separate the names of points.

# Solving

Geometric constraint solving (GCS) is an NP problem, solving complexity increases exponentially. As such, the current philosophy of GCAD solving is that solvers should...
- return a single result without regard for other possible solutions.
- timeout according to reasonable limits on resources.
- not give false confidence (ie. "there is no solution" or "these are the only solutions" without having proven so).

## GSolver
This current GCS solver is of my own design. It converts constraints into the geometry which represents all points which satisfy it, called the possibility space. These are then intersected to create a discrete set of points from which one is chosen to be the assumed position to inform the solving for the next point. If a point's constraints have no valid intersections, the solver backtracks.

To determine the order in which to solve the points, GSolver uses a breadth-first search from some origin/root point. Constraints determine which points they can be applied to given the currently known points. A point is considered discrete (and therefore known for ordering purposes) when two or more constraints are applied. This is assumed because two 1D elements (lines, including curves) intersect at a finite set of 0D elements (points) as long as they are not identical for some continuous range.

Constraints whose possibility space is a 2D element (area) are applied only to sets of points and are not counted towards the two constraints needed for discretizing.
