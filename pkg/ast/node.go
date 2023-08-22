package ast

import (
	"errors"
	"fmt"
	"strings"
)

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

func validateLayout(c *node) error {
	nPlaceholders := strings.Count(c.layout, PLACEHOLDER)
	nChildren := len(c.children)
	if nChildren != nPlaceholders {
		return errors.New(
			fmt.Sprintf(
				"Layout mismatch, %d placeholders and %d children.",
				nPlaceholders,
				nChildren,
			),
		)
	}

	return nil
}

func (c *node) CodeString() (string, error) {
	err := validateLayout(c)
	if err != nil {
		return "", err
	}

	result := c.layout

	for _, child := range c.children {
		childRepr, err := child.CodeString()

		if err != nil {
			return "", err
		}

		result = strings.Replace(result, PLACEHOLDER, childRepr, 1)
	}

	return result, err
}

func (c *node) TreeString() (string, error) {
	return "", errors.New("Not implemented!")
}
