package ast

import "errors"

type literal struct {
	value string
}

func NewLiteral(value string) *literal {
	return &literal{value: value}
}

func (l *literal) CodeString() (string, error) {
	return l.value, nil
}

func (l *literal) TreeString() (string, error) {
	return "", errors.New("Not implemented!")
}
