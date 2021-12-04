package main

import (
	"fmt"
	"strconv"
	"strings"
)

type BingoNumbers []int

const bingoDimensions int = 5

type BingoCard struct {
	numbers [bingoDimensions][bingoDimensions]int
	marks   [bingoDimensions][bingoDimensions]bool
}

func readBingoCard(lines []string) BingoCard {
	if len(lines) != bingoDimensions {
		panic("readBingoCard: unexpected number of lines")
	}

	tmpCard := BingoCard{}
	for i := 0; i < bingoDimensions; i++ {
		for j, numStr := range strings.Fields(lines[i]) {
			num, _ := strconv.Atoi(numStr)
			tmpCard.numbers[i][j] = num
			tmpCard.marks[i][j] = false
		}
	}
	return tmpCard
}

func (card *BingoCard) printBingoCard() string {
	outStr := make([]string, 0)
	for i := 0; i < bingoDimensions; i++ {
		for j := 0; j < bingoDimensions; j++ {
			mark := ""
			if card.marks[i][j] {
				mark = "*"
			}
			outStr = append(outStr, fmt.Sprintf("%2d%1s ", card.numbers[i][j], mark))
		}
		outStr = append(outStr, "\n")
	}
	outStr = append(outStr, "\n")
	return strings.Join(outStr, "")
}

func (card *BingoCard) updateDraw(number int) bool {
	for i := 0; i < bingoDimensions; i++ {
		for j := 0; j < bingoDimensions; j++ {
			if card.numbers[i][j] == number {
				card.marks[i][j] = true
			}
		}
	}
	return false
}

func (card *BingoCard) checkBingoCard() bool {
	markedRow := false
	for i := 0; i < bingoDimensions; i++ {
		markedRow = true
		for j := 0; j < bingoDimensions; j++ {
			if !card.marks[i][j] {
				markedRow = false
			}
		}
		if markedRow {
			return true
		}
	}

	markedColumn := false
	for j := 0; j < bingoDimensions; j++ {
		markedColumn = true
		for i := 0; i < bingoDimensions; i++ {
			if !card.marks[i][j] {
				markedColumn = false
			}
		}
		if markedColumn {
			return true
		}
	}
	return false
}

func (card *BingoCard) winningScore(lastDraw int) int {
	score := 0
	for i := 0; i < bingoDimensions; i++ {
		for j := 0; j < bingoDimensions; j++ {
			if !card.marks[i][j] {
				score += card.numbers[i][j]
			}
		}
	}
	score *= lastDraw
	return score
}

func readBingo(inputText string) (BingoNumbers, []BingoCard) {
	var lines = strings.Split(strings.TrimSpace(inputText), "\n")

	var drawnNumbers = make(BingoNumbers, 0)
	for _, numStr := range strings.Split(lines[0], ",") {
		num, _ := strconv.Atoi(numStr)
		drawnNumbers = append(drawnNumbers, num)
	}

	var bingoCards = make([]BingoCard, 0)
	for i := 2; i < len(lines); i += 6 {
		bingoCards = append(bingoCards, readBingoCard(lines[i:i+5]))
	}
	return drawnNumbers, bingoCards
}

func day4() {
	var inputText = downloadHelper(2021, 4)
	// var inputText = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7"
	drawnNumbers, bingoCards := readBingo(inputText)
	fmt.Printf("%v\n", drawnNumbers)
	winRound := make([]int, len(bingoCards))
	winScore := make([]int, len(bingoCards))

	for i, number := range drawnNumbers {
		fmt.Printf("round %d, draw %d\n", i, number)
		for j := 0; j < len(bingoCards); j++ {
			card := &bingoCards[j]
			card.updateDraw(number)
			//fmt.Printf("%s\n", card.printBingoCard())
			if winRound[j] == 0 && card.checkBingoCard() {
				winRound[j] = i
				winScore[j] = card.winningScore(number)
				fmt.Printf("found a winner in round %d:\n%v", i, card.printBingoCard())
				fmt.Printf("winning score is: %v\n", winScore[j])
			}
		}
	}
}
