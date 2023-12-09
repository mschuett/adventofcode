
data class ToyBoatRace(val time: Long, val dist: Long) {
    companion object Factory {
        fun fromString(inputText: String): Sequence<ToyBoatRace> = sequence {
            val lines = inputText.trim().split('\n')
            assert(lines.size == 2)
            val times = lines[0].split(" ").filter { it.isNotEmpty() }
            val dists = lines[1].split(" ").filter { it.isNotEmpty() }

            assert(times.size == dists.size)
            assert(times[0] == "Time:")
            assert(dists[0] == "Distance:")

            for (i in 1 until times.size) {
                yield(ToyBoatRace(times[i].toLong(), dists[i].toLong()))
            }
        }
        fun fromBadKerningString(inputText: String): ToyBoatRace {
            val lines = inputText.trim().split('\n').map {
                it.replace(" ", "")
            }
            assert(lines.size == 2)
            val times = lines[0].split(":")
            val dists = lines[1].split(":")
            assert(times.size == dists.size)
            assert(times.size == 2)
            assert(times[0] == "Time")
            assert(dists[0] == "Distance")
            return ToyBoatRace(times[1].toLong(), dists[1].toLong())
        }
    }

    // I thought I needed a non-simple counter for Part Two -- but this was still fast enough
    fun simpleCountWaysToWin(): Int =
        (1 until time).map { hold ->
            (time - hold) * hold
        }.count {
            it > dist
        }
}


fun day06(test: Boolean = true) {
    val inputText = if (test)
        """
        Time:      7  15   30
        Distance:  9  40  200
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 6)

    // parsing
    // Part One
    ToyBoatRace.fromString(inputText).map { race ->
        race.simpleCountWaysToWin()
    }.reduce {
             acc, i -> acc * i
    }.also {
        println(it)
    }

    // Part Two
    println(ToyBoatRace.fromBadKerningString(inputText).simpleCountWaysToWin())
}
