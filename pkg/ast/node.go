package ast

const PLACEHOLDER string = "{::}"

type Node interface {
	CodeString() (string, error)
	TreeString() (string, error)
}

type node struct {
	name     string
	layout   string
	children []Node
}

func New(name, layout string, children []Node) *node {
	return &node{name: name, layout: layout, children: children}
}
