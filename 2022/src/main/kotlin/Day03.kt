fun day03(test: Boolean = true) {
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
