import java.net.URI
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse

/* helper function to download input texts */
class InputTextDownloader {
    private val sessionId = System.getenv("ADVENT_AUTH_SESSION_ID")
    private val client = HttpClient.newBuilder()
        .followRedirects(HttpClient.Redirect.ALWAYS)
        .build()

    fun getText(year: Int, day: Int) : String {
        val request = HttpRequest.newBuilder()
            .uri(URI.create("https://adventofcode.com/${year}/day/${day}/input"))
            .header("Cookie", "session=$sessionId")
            .build()
        val response = client.send(request, HttpResponse.BodyHandlers.ofString())
        return response.body()
    }
}

fun day1(test: Boolean = true) {
    val inputText = if (test)
        """
            1000
            2000
            3000
            
            4000
            
            5000
            6000
            
            7000
            8000
            9000
            
            10000
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 1)

    val elves = ArrayList<ArrayList<Int>>(30)

    var elf = ArrayList<Int>(5)
    for (line in inputText.split('\n')) {
        if (line.isNotEmpty()) {
            elf.add(line.toInt())
        } else {
            elves.add(elf)
            elf = ArrayList(5)
        }
    }
    elves.add(elf)
    println(elves)

    // Part a
    var maxCalories = 0
    for (carry in elves) {
        val calories = carry.sum()
        if (calories > maxCalories) {
            maxCalories = calories
        }
    }
    println("max calories are $maxCalories")

    // Part b
    val sums = ArrayList(elves.map { arr -> arr.sum() })
    sums.sortDescending()

    println("sum: $sums")
    val topThree = sums.slice(0..2)
    println("top three elves are: $topThree")
    println("sum: " + topThree.sum())
}

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

fun day2(test: Boolean = true) {
    val inputText = if (test)
        """
            A Y
            B X
            C Z
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 2)

    val roundsA = ArrayList<Pair<RpsPlay,RpsPlay>>(2500)
    val roundsB = ArrayList<Pair<RpsPlay,RpsOutcome>>(2500)
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

fun day3(test: Boolean = true) {
    val inputText = if (test)
        """
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 3)

    class RucksackContent(input: String) {
        var first : String = ""
        var second: String = ""

        init {
            require (input.length % 2 == 0)
            val half = input.length/2
            first = input.slice(0 until half)
            second = input.slice(half until input.length)
        }

        override fun toString(): String = "RucksackContent($first $second)"
        fun commonItem(): Char {
            val common = first.toSet() intersect second.toSet()
            assert(common.size == 1)
            return common.first()
        }
        fun fullContent(): String = first + second
    }
    fun Char.priority() : Int {
        return when (this) {
            in 'a'..'z' -> {
                this - 'a' + 1
            }
            in 'A' .. 'Z' -> {
                this - 'A' + 27
            }
            else -> {
                throw IllegalArgumentException("unexpected char $this")
            }
        }
    }
    val rucksacks = inputText.split('\n')
        .filter { it.isNotEmpty() }
        .map { RucksackContent(it) }

    // part A
    rucksacks.map { it.commonItem() }
        .sumOf { it.priority() }
        .also { println("prioSum: $it") }

    // part B
    assert(rucksacks.size % 3 == 0)
    fun commonItemInRucksacks(sacks : List<RucksackContent>) : Char {
        require(sacks.size >= 2)
        val common : Set<Char> = sacks
            .map { it.fullContent().toSet() }
            .reduce { acc, it -> it intersect acc }
        assert (common.size == 1)
        return common.first()
    }
    rucksacks.chunked(3)
        .map { commonItemInRucksacks(it) }
        .sumOf { it.priority() }
        .also { println("common items prio sum: $it") }
}

fun main() {
    day3(true)
}
