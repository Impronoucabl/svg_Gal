This is not a real README

Unicode mappings from private use space (as of 23/03/2024)
'\u{e000}'..='\u{e0ff}' | Unused
'\u{e100}'              | "CH"
'\u{e101}'..='\u{e1ff}' | Reserved for CH variants
'\u{e200}'              | "ND"
'\u{e201}'..='\u{e2ff}' | Reserved for ND variants
'\u{e300}'              | "PH"
'\u{e301}'..='\u{e3ff}' | Reserved for PH variants
'\u{e400}'              | "WH"
'\u{e401}'..='\u{e4ff}' | Reserved for WH variants
'\u{e500}'              | "SH"
'\u{e501}'..='\u{e5ff}' | Reserved for SH variants
'\u{e600}'              | "NT"
'\u{e601}'..='\u{e6ff}' | Reserved for NT variants
'\u{e700}'              | "GH"
'\u{e701}'..='\u{e7ff}' | Reserved for GH variants
'\u{e800}'              | "NG"
'\u{e801}'..='\u{e8ff}' | Reserved for NG variants
'\u{e900}'              | "QU"
'\u{e901}'..='\u{e9ff}' | Reserved for NG variants
'\u{ea00}'              | Unused
'\u{ea01}'              | "AA"
'\u{ea02}'              | "BB"
'\u{ea03}'              | "CC"
'\u{ea04}'              | "DD"
'\u{ea05}'              | "EE"
'\u{ea06}'              | "FF"
'\u{ea07}'              | "GG"
'\u{ea08}'              | "HH"
'\u{ea09}'              | "II"
'\u{ea0a}'              | "JJ"
'\u{ea0b}'              | "KK"
'\u{ea0c}'              | "LL"
'\u{ea0d}'              | "MM"
'\u{ea0e}'              | "NN"
'\u{ea0f}'              | "OO"
'\u{ea10}'              | "PP"
'\u{ea11}'              | "QQ"
'\u{ea12}'              | "RR"
'\u{ea13}'              | "SS"
'\u{ea14}'              | "TT"
'\u{ea15}'              | "UU"
'\u{ea16}'              | "VV"
'\u{ea17}'              | "WW"
'\u{ea18}'              | "XX"
'\u{ea19}'              | "YY"
'\u{ea1a}'              | "ZZ"
'\u{ea1b}'..='\u{ea20}' | Unused
'\u{ea21}'              | "AAA"
'\u{ea22}'              | "BBB"
'\u{ea23}'              | "CCC"
'\u{ea24}'              | "DDD"
'\u{ea25}'              | "EEE"
'\u{ea26}'              | "FFF"
'\u{ea27}'              | "GGG"
'\u{ea28}'              | "HHH"
'\u{ea29}'              | "III"
'\u{ea2a}'              | "JJJ"
'\u{ea2b}'              | "KKK"
'\u{ea2c}'              | "LLL"
'\u{ea2d}'              | "MMM"
'\u{ea2e}'              | "NNN"
'\u{ea2f}'              | "OOO"
'\u{ea30}'              | "PPP"
'\u{ea31}'              | "QQQ"
'\u{ea32}'              | "RRR"
'\u{ea33}'              | "SSS"
'\u{ea34}'              | "TTT"
'\u{ea35}'              | "UUU"
'\u{ea36}'              | "VVV"
'\u{ea37}'              | "WWW"
'\u{ea38}'              | "XXX"
'\u{ea39}'              | "YYY"
'\u{ea3a}'              | "ZZZ"
'\u{ea3b}'..='\u{f8ff}' | Unused

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