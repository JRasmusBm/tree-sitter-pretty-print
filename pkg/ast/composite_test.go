package ast_test

import (
	"fmt"
	"github.com/jrasmusbm/tree-sitter-pretty-print/pkg/ast"
	"testing"
)

func TestCompositeCodeString(t *testing.T) {
	t.Run("Applies formatting to the children", func(t *testing.T) {

		got, _ := ast.NewComposite(fmt.Sprintf("(%s)", ast.PLACEHOLDER), []ast.Node{
			ast.NewLiteral("hello"),
		}).CodeString()
		want := "(hello)"

		if got != want {
			t.Errorf("got %v want %v", got, want)
		}
	})
}
