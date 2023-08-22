package ast_test

import (
	"fmt"
	"github.com/jrasmusbm/tree-sitter-pretty-print/pkg/ast"
	"testing"
)

func TestCompositeCodeString(t *testing.T) {
	t.Run("Applies formatting to the children", func(t *testing.T) {
		got, err := ast.New(fmt.Sprintf(`if (%s) {
  %s
} else {
  %s
}`, ast.PLACEHOLDER, ast.PLACEHOLDER, ast.PLACEHOLDER), []ast.Node{
			ast.New("isOpen", []ast.Node{}),
			ast.New(fmt.Sprintf("%s(%s);", ast.PLACEHOLDER, ast.PLACEHOLDER), []ast.Node{
				ast.New(fmt.Sprintf("%s.%s", ast.PLACEHOLDER, ast.PLACEHOLDER), []ast.Node{
					ast.New("console", []ast.Node{}),
					ast.New("log", []ast.Node{}),
				}),
				ast.New("42", []ast.Node{}),
			}),
			ast.New(fmt.Sprintf("return %s;", ast.PLACEHOLDER), []ast.Node{
				ast.New("null", []ast.Node{}),
			}),
		}).CodeString()
		want := `if (isOpen) {
  console.log(42);
} else {
  return null;
}`

		assertStringsMatch(t, got, want, err)
	})

	t.Run("Renders nodes without placeholders", func(t *testing.T) {
		got, err := ast.New(fmt.Sprintf("hello"), []ast.Node{}).CodeString()
		want := `hello`

		assertStringsMatch(t, got, want, err)
	})

	t.Run("Returns error on placeholder mismatch", func(t *testing.T) {
		_, err := ast.New(
			fmt.Sprintf("(%s, %s, %s)", ast.PLACEHOLDER, ast.PLACEHOLDER, ast.PLACEHOLDER),
			[]ast.Node{
				ast.New("hello", []ast.Node{}),
				ast.New("world", []ast.Node{}),
			},
		).CodeString()

		assertError(t, err, "Layout mismatch, 3 placeholders and 2 children.")
	})
}
