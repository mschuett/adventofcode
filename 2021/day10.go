package main

import (
	"fmt"
	"sort"
	"strings"
)

func day10() {
	// var inputText = "()\n[]\n([])\n{()()()}\n<([{}])>\n[<>({}){}[([])<>]]\n(((((((((())))))))))\n[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]\n"
	var inputText = downloadHelper(2021, 10)

	var charErrorScore = map[byte]int{
		')': 3,
		']': 57,
		'}': 1197,
		'>': 25137,
	}
	var charCompletionScore = map[byte]int{
		')': 1,
		']': 2,
		'}': 3,
		'>': 4,
	}
	var openingParen = map[byte]byte{
		')': '(',
		']': '[',
		'}': '{',
		'>': '<',
	}
	var closingParen = map[byte]byte{
		'(': ')',
		'[': ']',
		'{': '}',
		'<': '>',
	}
	var syntaxErrorScore = 0
	var completionScores = make([]int, 0)
	var completeLines = make([]string, 0)
	for _, line := range strings.Split(strings.TrimSpace(inputText), "\n") {
		var stack = Stack{}
		var complete = true

		for i := 0; i < len(line); i++ {
			char := line[i]
			if char == '(' || char == '[' || char == '{' || char == '<' {
				stack.Push(char)
				continue
			}
			if !(char == ')' || char == ']' || char == '}' || char == '>') {
				panic("unexpected char in input")
			}
			// now we have a closing parenthesis
			if stack.IsEmpty() {
				complete = false
				break
			}
			matchingOpenParen := openingParen[char]
			stackedParen, _ := stack.Peek()
			if stackedParen == matchingOpenParen {
				stack.Pop() // matches as expected
			} else {
				matchingClosingParen := closingParen[stackedParen]
				fmt.Printf("corrupted line, expected %c, found %c, in %s\n", matchingClosingParen, char, line)
				syntaxErrorScore += charErrorScore[char]
				complete = false
				break
			}
		}
		if complete && stack.IsEmpty() {
			fmt.Printf("complete line %s\n", line)
		} else if complete && !stack.IsEmpty() {
			fmt.Printf("incomplete line %s, left stack %v", line, stack)
			// Part 2, calc completion
			lineScore := 0
			for {
				if stack.IsEmpty() {
					break
				}
				stackedParen, _ := stack.Pop()
				closing := closingParen[stackedParen]
				lineScore *= 5
				lineScore += charCompletionScore[closing]
			}
			completionScores = append(completionScores, lineScore)
			fmt.Printf(" --> completion score %d\n", lineScore)
		} else if !complete && !stack.IsEmpty() {
			fmt.Printf("incompletely parsed line %s, left stack %v\n", line, stack)
		} else if !complete && stack.IsEmpty() {
			fmt.Printf("incomplete parsed line %s, left empty stack\n", line)
		}
		completeLines = append(completeLines, line)
	}
	fmt.Printf("syntax error syntaxErrorScore %v\n\n", syntaxErrorScore)
	sort.Ints(completionScores)
	fmt.Printf("middle syntax completionScore %d, middle value from %v\n", completionScores[len(completionScores)/2], completionScores)
}
