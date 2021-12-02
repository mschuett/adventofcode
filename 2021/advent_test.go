package main

import "testing"

func Test_day1aImpl(t *testing.T) {
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

func Test_day2aImpl(t *testing.T) {
	type args struct {
		inputText string
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{"example", args{"forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n"}, 150},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := day2aImpl(tt.args.inputText); got != tt.want {
				t.Errorf("day2aImpl() = %v, want %v", got, tt.want)
			}
		})
	}
}

func Test_day2bImpl(t *testing.T) {
	type args struct {
		inputText string
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{"example", args{"forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n"}, 900},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := day2bImpl(tt.args.inputText); got != tt.want {
				t.Errorf("day2bImpl() = %v, want %v", got, tt.want)
			}
		})
	}
}
