package main

import "testing"

func Test_day1impl(t *testing.T) {
	type args struct {
		inputText string
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{"example", args{"199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n"}, 7},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := day1aImpl(tt.args.inputText); got != tt.want {
				t.Errorf("day1aImpl() = %v, want %v", got, tt.want)
			}
		})
	}
}
