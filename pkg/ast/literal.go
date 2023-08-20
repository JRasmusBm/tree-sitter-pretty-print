package ast

type literal struct {
	value string
}

func NewLiteral(value string) *literal {
	return &literal{value: value}
}

func (l *literal) CodeString() string {
  return l.value 
}


