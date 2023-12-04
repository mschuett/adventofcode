
data class D2Game(val num: Int, val reveals: List<Reveal>) {
    data class Reveal(val blue: Int, val red: Int, val green: Int) {
        companion object Factory {
            fun fromString(inputText: String): Reveal {
                val dicePairs = inputText
                    .split(',')
                    .associate {
                        val words = it.trim().split(" ")
                        words.last() to words.first().toInt()
                    }
                return Reveal(
                    dicePairs["blue"] ?: 0,
                    dicePairs["red"] ?: 0,
                    dicePairs["green"] ?: 0
                )
            }
        }
    }

    companion object Factory {
        fun fromString(inputText: String): D2Game {
            val (gamePart, revealPart) = inputText.split(':')
            val gameNum = gamePart.split(" ").component2().toInt()
            val reveals = revealPart
                .split(';')
                .map {
                    Reveal.fromString(it.trim())
                } as ArrayList<Reveal>
            return D2Game(gameNum, reveals)
        }
    }

    fun isPossibleWith(blue: Int, red: Int, green: Int): Boolean {
        return this.reveals.all {
            it.blue <= blue && it.red <= red && it.green <= green
        }
    }

    fun powerOfFewestNumberOfCubes(): Int {
        return this.reveals.maxOf{ it.blue } *
                this.reveals.maxOf{ it.red } *
                this.reveals.maxOf{ it.green }
    }
}

fun day02(test: Boolean = true) {
    val inputText = if (test)
        """
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 2)

    // parsing
    val games = ArrayList<D2Game>(30)
    for (line in inputText.split('\n')) {
        if (line.isNotEmpty()) {
            games.add(D2Game.fromString(line))
        }
    }
    println(games)

    // Part One
    games
        .filter { it.isPossibleWith(14,12,13) }
        .sumOf { it.num }
        .also { println(it) }

    // Part Two
    games
        .map { it.powerOfFewestNumberOfCubes() }
        .also { println(it) }
        .sum()
        .also { println(it) }
}
