This is not a real README

Instead of T for Stemtype, use S to avoid confusion with Rust's Generic Type pararmeter.
Instead of TH for Stemtype, use Z to keep everything momospaced aligned. 

Upper dist limits
J | self.dist() + self.outer_radius() + self.thick() <= self.parent_inner()
B | self.dist() + self.outer_radius() <= self.parent_outer() + self.parent_thick()
S | self.dist() - self.inner_radius() <= self.parent_inner()
Z | self.dist() <= self.parent_outer() //this one could be relaxed for style
Upper radius limits
J | self.outer_radius() + self.dist() + self.thick() <= self.parent_inner()
B | self.outer_radius() + self.dist() <= self.parent_outer() + self.parent_thick()
S | INF
Z | self.outer_radius() - self.dist() <= 2.0*self.parent_outer() - self.parent_radius() 
Upper thick limits
J | 0.0
B | 0.0
S | 0.0
Z | 0.0

Lower dist limits
J | 0.0
B | self.dist() + self.outer_radius() >= self.parent_outer()
S | self.dist() >= self.parent_inner()
Z | self.dist() >= self.parent_inner()
Lower radius limits
J | 0.0
B | self.outer_radius() + self.dist() >= self.parent_outer()
S | 0.0
Z | 0.0
Lower thick limits
J | 0.0
B | 0.0
S | 0.0
Z | 0.0