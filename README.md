# Crafting Intepreters

This is a rust implementation of JLox from the Robert Nystrom book [crafting interpreters](https://craftinginterpreters.com/).

Utils links 

https://www.amazon.com/Practical-Foundations-Programming-Languages-Professor/dp/1107029570/ref=pd_sim_b_5?ie=UTF8&refRID=0N8T19WYAA7QYEFXNDRN

https://www.amazon.com/Formal-Semantics-Programming-Languages-Winskel/dp/0262731037/ref=sr_1_1?s=books&ie=UTF8&qid=1419297786&sr=1-1&keywords=formal+semantics+of+programming+languages

https://www.amazon.com/Elements-Computing-Systems-Building-Principles/dp/0262640686

## Chapter 4 - Scanning

### Questions
1. The lexical grammars of Python and Haskell are not regular. What does that mean, and why aren’t they?

2. Aside from separating tokens—distinguishing print foo from printfoo—spaces aren’t used for much in most languages. However, in a couple of dark corners, a space does affect how code is parsed in CoffeeScript, Ruby, and the C preprocessor. Where and what effect does it have in each of those languages?

3. Our scanner here, like most, discards comments and whitespace since those aren’t needed by the parser. Why might you want to write a scanner that does not discard those? What would it be useful for?
  A: It can be useful to scan comments to generate documentation.

4. Add support to Lox’s scanner for C-style /* ... */ block comments. Make sure to handle newlines in them. Consider allowing them to nest. Is adding support for nesting more work than you expected? Why?
  A: Done!
