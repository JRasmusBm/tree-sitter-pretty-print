package ast

import (
	"fmt"
)

func (c *node) TreeString() (string, error) {
	return fmt.Sprintf("(%s)", c.name), nil
}
