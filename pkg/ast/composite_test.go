package ast_test

import (
	"fmt"
	"github.com/jrasmusbm/tree-sitter-pretty-print/pkg/ast"
	"testing"
)

func assertStringsMatch(t testing.TB, got, want string, err error) {
	t.Helper()

	if got != want {
		t.Errorf("got %v want %v", got, want)
	}

	if err != nil {

		t.Errorf("Expected no errors, got `%s`", err.Error())
	}
}

func assertError(t testing.TB, err error, want string) {
	t.Helper()

	if err == nil {
		t.Errorf("Expected error: `%s` got nil.", want)
	}

	got := err.Error()
	if got != want {
		t.Errorf("Expected error: `%s` got `%s`.", got, want)
	}
}

func TestCompositeCodeString(t *testing.T) {
	t.Run("Applies formatting to the children", func(t *testing.T) {

		got, err := ast.NewComposite(fmt.Sprintf(`if (%s) {
  %s
} else {
  %s
}`, ast.PLACEHOLDER, ast.PLACEHOLDER, ast.PLACEHOLDER), []ast.Node{
			ast.NewLiteral("isOpen"),
			ast.NewComposite(fmt.Sprintf("%s(%s);", ast.PLACEHOLDER, ast.PLACEHOLDER), []ast.Node{
				ast.NewComposite(fmt.Sprintf("%s.%s", ast.PLACEHOLDER, ast.PLACEHOLDER), []ast.Node{
					ast.NewLiteral("console"),
					ast.NewLiteral("log"),
				}),
				ast.NewLiteral("42"),
			}),
			ast.NewComposite(fmt.Sprintf("return %s;", ast.PLACEHOLDER), []ast.Node{
				ast.NewLiteral("null"),
			}),
		}).CodeString()
		want := `if (isOpen) {
  console.log(42);
} else {
  return null;
}`

		assertStringsMatch(t, got, want, err)
	})

	t.Run("Returns error on placeholder mismatch", func(t *testing.T) {
		_, err := ast.NewComposite(
			fmt.Sprintf("(%s, %s, %s)", ast.PLACEHOLDER, ast.PLACEHOLDER, ast.PLACEHOLDER),
			[]ast.Node{
				ast.NewLiteral("hello"),
				ast.NewLiteral("world"),
			},
		).CodeString()

		assertError(t, err, "Layout mismatch, 3 placeholders and 2 children.")
	})
}
