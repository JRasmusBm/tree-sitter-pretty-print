# tree-sitter-pretty-print

Inspired by the amazing
[tree-sitter](git@github.com:JRasmusBm/tree-sitter-pretty-print.git) library,
which is used to make super fast parsers.

## The Problem

- It is not possible to reproduce the source code from a tree-sitter parse tree.
- This means that one can not edit the tree and get valid source code back out.

## What this project does (High Level)

- Understands tree-sitter parse trees.
- Generates a tree in a DSL based on a tree-sitter parse tree that contains enough
  information to reproduce valid code in the original language.
- Turn a tree in the DSL into code in the original language.

## Implications

- Being able to edit the syntax tree itself would open up another level of
  refactoring capabilities.

## Non-goals of this project

- Syntax on par with the latest formatters
- i.e. you will have to run black, prettier, gofmt etc. on the output.
