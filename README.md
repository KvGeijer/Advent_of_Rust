# Advent_of_Rust
Advent of code is an advent calendar with programming problems. This year I decided to do them all in Rust to get to know the language better. My goal was to write quite nicely st4ructured idiomatic code. Prefferably it should also be fast and it would be nice if the solutions to all days ran in under a second combined.

I managed to convince some friends to also do it in Rust. Then we could discuss design and every week met to discuss sulutions över some mulled wine and gingerbread cookies. You can check out the code of two of them here: 
- Samuel: https://gitlab.com/samuelselleck/advent-of-code-rust/-/tree/master,
- Anton: coming soon (https://github.com/AAlmqvist)

## Conclusion

I just about had time to solve all puzzles during the intended days. The last days wre in my opinion a bit too hard considering you often want to spend those days with friends and family. 

All in all the execution time took around 600 ms for all days combined which I count as a success. Some days (see day 23) were not idiomatic or pretty, but most of the solutions were quite nice. 

# Thoughts on Rust

I really enjoyed the parsing ones and the discovery of the power of enums in Rust. Using them in combination with trats and generics you can basically do subclassing if needed, but under no delusions of dynamic dispatch or similar. Also the overall support for functional programming was nice and fun to play around with. I did not do basically any macros which was a bit sad. But I guess that will have to wait until some other time. 

One issue I had with the language was that initiating arrays was a bit of a hassle. Also the borrowing rules are some times a bit annoying, like when traversing graphs I found. Overall I also missed a matrix package such as numpy (ndarray was good, but it has some ways to go to fully compare). Also stack based arrays with sizes determined at runtime feels like a nice addition.

Overall I enjoyed the language. Compared to C you lack certain freedoms to do clever solutions at times. If I would do research I would use C to get that extra freedom, but if I would do a project or a real product I would definetely consider Rust. Especially if I had a larger team I would appreciate the borrow checker and otherwise struct compiler. The compiler in combination with a good linter feels like a good environemnt for producing good software.
