
data class CamelCardHand(val cards: List<Card>, val bid: Int) : Comparable<CamelCardHand> {
    enum class Card(val c: Char) : Comparable<Card> {
        Two('2'),
        Three('3'),
        Four('4'),
        Five('5'),
        Six('6'),
        Seven('7'),
        Eight('8'),
        Nine('9'),
        Ten('T'),
        Jack('J'),
        Queen('Q'),
        King('K'),
        Ace('A');
        companion object {
            fun from(c: Char): Card = Card.values().firstOrNull { it.c == c }!!
        }
        override fun toString(): String {
            return c.toString()
        }
    }
    enum class HandType : Comparable<HandType> {
        High,
        One,
        Two,
        Three,
        FullHouse,
        Four,
        Five,
    }
    companion object Factory {
        fun fromString(inputText: String): CamelCardHand {
            val strings = inputText.trim().split(' ')
            return CamelCardHand(strings[0].map{ Card.from(it) }, strings[1].toInt())
        }
    }

    override fun toString(): String {
        val cardString = cards.joinToString("") { it.toString() }
        return "$cardString (${getHandType()}) ${bid}"
    }

    fun getHandType(): HandType =
        when (cards.groupingBy{it}.eachCount().values.sorted()) {
            listOf(5) -> HandType.Five
            listOf(1, 4) -> HandType.Four
            listOf(2, 3) -> HandType.FullHouse
            listOf(1, 1, 3) -> HandType.Three
            listOf(1, 2, 2) -> HandType.Two
            listOf(1, 1 ,1, 2) -> HandType.One
            listOf(1, 1, 1, 1, 1) -> HandType.High
            else -> TODO("invalid state")
        }

    override fun compareTo(other: CamelCardHand): Int {
        val typeResult = getHandType().compareTo(other.getHandType())
        if (typeResult != 0)
            return typeResult
        else {
            for (i in cards.indices) {
                val cardComparison = cards[i].compareTo(other.cards[i])
                if (cardComparison != 0)
                    return cardComparison
            }
            return 0
        }
    }
}

// could not find a good way to merge the differet cases, so this is mostly copy & paste
class CamelCardJokerHand(val cards: List<Card>, val bid: Int) : Comparable<CamelCardJokerHand> {
    enum class Card(val c: Char) : Comparable<Card> {
        Joker('J'),
        Two('2'),
        Three('3'),
        Four('4'),
        Five('5'),
        Six('6'),
        Seven('7'),
        Eight('8'),
        Nine('9'),
        Ten('T'),
        Queen('Q'),
        King('K'),
        Ace('A');
        companion object {
            fun from(c: Char): Card = Card.values().firstOrNull { it.c == c }!!
        }
        override fun toString(): String {
            return c.toString()
        }
    }
    enum class HandType : Comparable<HandType> {
        High,
        One,
        Two,
        Three,
        FullHouse,
        Four,
        Five,
    }
    companion object Factory {
        fun fromString(inputText: String): CamelCardJokerHand {
            val strings = inputText.trim().split(' ')
            return CamelCardJokerHand(strings[0].map{ Card.from(it) }, strings[1].toInt())
        }
    }

    override fun toString(): String {
        val cardString = cards.joinToString("") { it.toString() }
        return "$cardString (${getHandType()}) ${bid}"
    }

    fun getHandType(): HandType {
        val numberOfJokers = cards.count { it == Card.Joker}
        return when (cards.groupingBy { it }.eachCount().values.sorted()) {
            listOf(5) -> {
                HandType.Five
            }
            listOf(1, 4) -> {
                when {
                    numberOfJokers >= 1 -> HandType.Five
                    else -> HandType.Four
                }
            }
            listOf(2, 3) -> {
                when {
                    numberOfJokers >= 2 -> HandType.Five
                    else -> HandType.FullHouse
                }
            }
            listOf(1, 1, 3) -> {
                when (numberOfJokers) {
                    3 -> HandType.Four
                    1 -> HandType.Four
                    else -> HandType.Three
                }
            }
            listOf(1, 2, 2) -> {
                when (numberOfJokers) {
                    2 -> HandType.Four
                    1 -> HandType.FullHouse
                    else -> HandType.Two
                }
            }
            listOf(1, 1, 1, 2) -> {
                when (numberOfJokers) {
                    2 -> HandType.Three
                    1 -> HandType.Three
                    else -> HandType.One
                }
            }
            listOf(1, 1, 1, 1, 1) -> {
                when (numberOfJokers) {
                    1 -> HandType.One
                    else -> HandType.High
                }
            }
            else -> {
                TODO("invalid state")
            }
        }
    }

    override fun compareTo(other: CamelCardJokerHand): Int {
        val typeResult = getHandType().compareTo(other.getHandType())
        if (typeResult != 0)
            return typeResult
        else {
            for (i in cards.indices) {
                val cardComparison = cards[i].compareTo(other.cards[i])
                if (cardComparison != 0)
                    return cardComparison
            }
            return 0
        }
    }
}


fun day07(test: Boolean = true) {
    val inputText = if (test)
        """
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 7)

    // Part One
    assert(CamelCardHand.fromString("QQQJA 483") > CamelCardHand.fromString("T55J5 684"))
    assert(CamelCardHand.fromString("33332 0") > CamelCardHand.fromString("2AAAA 0"))
    assert(CamelCardHand.fromString("77888 0") > CamelCardHand.fromString("77788 0"))
    val lines = inputText.trim().split('\n')
    val hands = lines.map { CamelCardHand.fromString(it) }.sorted().onEach(::println)

    hands.indices
        .map {
            val rank = it + 1
            rank * hands[it].bid
        }
        .also(::println)
        .sum()
        .also(::println)

    // Part Two
    assert(CamelCardJokerHand.fromString("QQQJA 0") > CamelCardJokerHand.fromString("T55J5 0"))
    assert(CamelCardJokerHand.fromString("33332 0") > CamelCardJokerHand.fromString("2AAAA 0"))
    assert(CamelCardJokerHand.fromString("77888 0") > CamelCardJokerHand.fromString("77788 0"))
    assert(CamelCardJokerHand.fromString("77A88 0") < CamelCardJokerHand.fromString("77J88 0"))
    assert(CamelCardJokerHand.fromString("77788 0") > CamelCardJokerHand.fromString("77J88 0"))
    assert(CamelCardJokerHand.fromString("77788 0") > CamelCardJokerHand.fromString("J7788 0"))
    val lines2 = inputText.trim().split('\n')
    val hands2 = lines2.map { CamelCardJokerHand.fromString(it) }.sorted().onEach(::println)

    hands2.indices
        .map {
            val rank = it + 1
            rank * hands2[it].bid
        }
        .also(::println)
        .sum()
        .also(::println)
}
