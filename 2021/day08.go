package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func readSegmentSignalsAsSlices(inputText string) ([]string, []string) {
	var patterns []string = make([]string, 10)
	var outputs []string = make([]string, 4)
	for i, word := range strings.Split(strings.TrimSpace(inputText), " ") {
		switch {
		case i < 10:
			patterns[i] = word
		case i == 10:
			if word != "|" {
				panic("unexpected input line format")
			}
		case i > 10:
			outputs[i-11] = word
		}

	}
	return patterns, outputs
}

func SortString(w string) string {
	s := strings.Split(w, "")
	sort.Strings(s)
	return strings.Join(s, "")
}

func StringSubsetOf(s1, s2 string) bool {
	for i := 0; i < len(s1); i++ {
		foundChar := false
		for j := 0; j < len(s2); j++ {
			if s1[i] == s2[j] {
				foundChar = true
				break
			}
		}
		if !foundChar {
			return false
		}
	}
	return true
}

func day8() {
	// var inputText = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
	var inputText = downloadHelper(2021, 8)

	var inputData = make([][2][]string, 0)
	for _, line := range strings.Split(strings.TrimSpace(inputText), "\n") {
		patterns, outputs := readSegmentSignalsAsSlices(line)
		pair := [2][]string{patterns, outputs}
		inputData = append(inputData, pair)
	}
	fmt.Printf("%v\n", inputData)

	var digitCount map[int]int = make(map[int]int)
	for _, pair := range inputData {
		outputs := pair[1]
		for i := 0; i < len(outputs); i++ {
			digitCount[len(outputs[i])]++
		}
	}
	fmt.Printf("%v\n", digitCount)
	fmt.Printf("count of digits 1, 4, 7, or 8: %d\n", digitCount[2]+digitCount[4]+digitCount[3]+digitCount[7])

	// part 2
	decodedNumbers := make([]int, 0)
	for _, pair := range inputData {
		var decode = make(map[string]int)

		// should we sort them by length first?
		var wordLen = make(map[int]string)
		for _, word := range pair[0] {
			wordLen[len(word)] = SortString(word)
		}
		// the simple ones
		decode[wordLen[2]] = 1
		decode[wordLen[3]] = 7
		decode[wordLen[4]] = 4
		decode[wordLen[7]] = 8
		// deduct the others
		// length 6: either 0, 6, or 9
		var word9 = "x"
		for _, word := range pair[0] {
			if len(word) == 6 {
				if StringSubsetOf(wordLen[4], word) {
					decode[SortString(word)] = 9
					word9 = SortString(word)
				} else if StringSubsetOf(wordLen[3], word) {
					decode[SortString(word)] = 0
				} else {
					decode[SortString(word)] = 6
				}
			}
		}
		// length 5: can be 2, 3, or 5
		for _, word := range pair[0] {
			if len(word) == 5 {
				if StringSubsetOf(wordLen[3], word) {
					decode[SortString(word)] = 3
				} else if StringSubsetOf(word, word9) {
					decode[SortString(word)] = 5
				} else {
					decode[SortString(word)] = 2
				}
			}
		}

		var decodedWords [4]int
		for i, word := range pair[1] {
			sword := SortString(word)
			decodedWords[i] = decode[sword]
		}

		number, _ := strconv.Atoi(fmt.Sprintf("%d%d%d%d", decodedWords[0], decodedWords[1], decodedWords[2], decodedWords[3]))
		decodedNumbers = append(decodedNumbers, number)
		fmt.Printf("decode: %v -> %v\n", pair[1], number)
	}
	sum := 0
	for i := 0; i < len(decodedNumbers); i++ {
		sum += decodedNumbers[i]
	}
	fmt.Printf("sum: %d\n", sum)
}
