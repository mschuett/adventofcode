
data class Scratchcard(val id: Int, val winning: List<Int>, val have: List<Int>) {
    companion object Factory {
        fun fromString(inputText: String): Scratchcard {
            val (cardText, numberText) = inputText.split(':')
            val cardNum = cardText.split(" ").filterNot { it.isEmpty() }.component2().toInt()

            val (winningText, haveText) = numberText.split('|')
            val winning = winningText.split(' ').filterNot { it.isEmpty() }.map { it.toInt() }
            val have = haveText.split(' ').filterNot { it.isEmpty() }.map { it.toInt() }
            return Scratchcard(cardNum, winning, have)
        }
    }
    fun getMatches(): Int = winning.intersect(have.toSet()).size
    fun getPoints(): Int {
        val matches = getMatches()
        if (matches == 0)
            return 0
        if (matches == 1)
            return 1
        var points = 1
        for (i in 2..matches) {
            points *= 2
        }
        return points
    }
}

fun day04(test: Boolean = true) {
    val inputText = if (test)
        """
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 4)

    // parsing
    val cards = ArrayList<Scratchcard>(30)
    for (line in inputText.trim().split('\n')) {
        if (line.isNotEmpty()) {
            cards.add(Scratchcard.fromString(line))
        }
    }

    // Part One
    cards.onEach {
        println("${it.id}: ${it.getPoints()}")
    }.sumOf {
        it.getPoints()
    }.also {
        println(it)
    }

    // Part Two
    val inventory = mutableMapOf<Int,Int>()
    cards.forEach {
        inventory[it.id] = 1
    }
    for (card in cards) {
        val cardCount = inventory[card.id]!!
        val matches = card.getMatches()
        for (i in card.id+1..(card.id+matches).coerceAtMost(cards.last().id)) {
            inventory[i] = cardCount + inventory[i]!!
        }
    }
    println(inventory)
    println(inventory.values.sum())
}
