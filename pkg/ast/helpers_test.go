package ast_test

import (
	"testing"
)

func assertStringsMatch(t testing.TB, got, want string, err error) {
	t.Helper()

	if got != want {
		t.Errorf("got %v want %v", got, want)
	}

	if err != nil {

		t.Errorf("Expected no errors, got `%s`", err.Error())
	}
}

func assertError(t testing.TB, err error, want string) {
	t.Helper()

	if err == nil {
		t.Errorf("Expected error: `%s` got nil.", want)
	}

	got := err.Error()
	if got != want {
		t.Errorf("Expected error: `%s` got `%s`.", got, want)
	}
}
