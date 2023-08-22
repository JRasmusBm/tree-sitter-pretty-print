package ast

import (
	"errors"
	"fmt"
	"strings"
)

const PLACEHOLDER string = "{::}"

type Node interface {
	CodeString() (string, error)
}

type composite struct {
	layout   string
	children []Node
}

func NewComposite(layout string, children []Node) *composite {
	return &composite{layout: layout, children: children}
}

func validateChildren(c *composite) error {
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

func (c *composite) CodeString() (string, error) {
	err := validateChildren(c)
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
