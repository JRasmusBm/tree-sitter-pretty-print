package ast_test

import (
	"github.com/jrasmusbm/tree-sitter-pretty-print/pkg/ast"
	"testing"
)

func TestTreeString(t *testing.T) {
	t.Run("Renders the name of the node when the format is empty", func(t *testing.T) {
		got, err := ast.New("MyNode", "hello", []ast.Node{}).TreeString()
		want := "(MyNode)"

		assertStringsMatch(t, got, want, err)
	})

}
