package ast_test

import (
	"github.com/jrasmusbm/tree-sitter-pretty-print/pkg/ast"
	"testing"
)

func TestLiteralCodeString(t *testing.T) {
	t.Run("Returns the value from the Literal", func(t *testing.T) {
		got, _ := ast.NewLiteral("hello").CodeString()
		want := "hello"

		if got != want {
			t.Errorf("got %v want %v", got, want)
		}
	})

}
