package main

import "testing"

func TestDummy(t *testing.T) {
	test1 := 1
	test2 := 2
	if test1 == test2 {
		t.Errorf("Uhh?")
	}
}
