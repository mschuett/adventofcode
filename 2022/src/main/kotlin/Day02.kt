enum class RpsPlay(private val value: Int) {
    Rock(1),
    Paper(2),
    Scissors(3);

    fun score() : Int = value

    fun winScoreAgainst(other: RpsPlay) : Int =
        if ((this == Rock     && other == Scissors)
         || (this == Scissors && other == Paper)
         || (this == Paper    && other == Rock)
        ) {
            0
        } else if (this == other) {
            3
        } else {
            6
        }

    // for Part b
    fun counterPlay(result: RpsOutcome) : RpsPlay =
        when (result) {
            RpsOutcome.Win -> {
                when (this) {
                    Rock     -> Paper
                    Paper    -> Scissors
                    Scissors -> Rock
                }
            }
            RpsOutcome.Loose -> {
                when (this) {
                    Rock     -> Scissors
                    Paper    -> Rock
                    Scissors -> Paper
                }
            }
            RpsOutcome.Draw -> this
        }

    companion object {
        fun fromString(str: String) : RpsPlay {
            return when (str[0]) {
                'A', 'X' -> Rock
                'B', 'Y' -> Paper
                'C', 'Z' -> Scissors
                else -> throw IllegalArgumentException("unexpected input $str")
            }
        }
    }
}

enum class RpsOutcome(private val value: Int) {
    Loose(0),
    Draw(3),
    Win(6);
    fun score() : Int = value
    companion object {
        fun fromString(str: String) : RpsOutcome {
            return when (str[0]) {
                'X' -> Loose
                'Y' -> Draw
                'Z' -> Win
                else -> throw IllegalArgumentException("unexpected input $str")
            }
        }
    }
}

fun day02(test: Boolean = true) {
    val inputText = if (test)
        """
            A Y
            B X
            C Z
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 2)

    val roundsA = ArrayList<Pair<RpsPlay, RpsPlay>>(2500)
    val roundsB = ArrayList<Pair<RpsPlay, RpsOutcome>>(2500)
    for (line in inputText.split('\n').filter { it.isNotEmpty() }) {
        val (a, b) = line.split(' ')
        roundsA.add(Pair(RpsPlay.fromString(a), RpsPlay.fromString(b)))
        roundsB.add(Pair(RpsPlay.fromString(a), RpsOutcome.fromString(b)))
    }

    // Part a
    println("rounds number: " + roundsA.size)
    val totalScore = roundsA.sumOf { (play, counter) ->
        counter.score() + play.winScoreAgainst(counter)
    }
    println("total score: $totalScore")

    val totalScoreB = roundsB.sumOf { (play, result) ->
        play.counterPlay(result).score() + result.score()
    }
    println("total score: $totalScoreB")
}
