fun day4(test: Boolean = true) {
    val inputText = if (test)
        """
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 4)

    val pairOfPairs = inputText
        .split('\n')
        .filter { it.isNotEmpty() }
        .map { it.split(',') }
        .map { (firstElf, secondElf) ->
            val firstRange = firstElf.split('-').map { it.toInt() }
            val secondRange = secondElf.split('-').map { it.toInt() }
            firstRange to secondRange
        }
        .also { println("total input lines: " + it.count()) }

    // part A
    pairOfPairs.filter {(a, b) ->
            ((a.first() <= b.first()) && (a.last() >= b.last())) ||
            ((a.first() >= b.first()) && (a.last() <= b.last()))
        }
        .count()
        .also { println("fully contained lines: $it") }

    // part B
    pairOfPairs
        .filter {(a, b) ->
            !(a.last() < b.first() || b.last() < a.first())
        }
        // .also(::println)
        .count()
        .also { println("overlapping lines: $it") }
}
