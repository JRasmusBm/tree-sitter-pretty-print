package ast

const PLACEHOLDER string = "{::}"

type Node interface {
	CodeString() (string, error)
	TreeString() (string, error)
}

type node struct {
	layout   string
	children []Node
}

func New(layout string, children []Node) *node {
	return &node{layout: layout, children: children}
}
