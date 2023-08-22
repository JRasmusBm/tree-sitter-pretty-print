package ast

import (
	"errors"
	"fmt"
	"strings"
)

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
