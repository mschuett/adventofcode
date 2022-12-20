data class Ring(val work: MutableList<Pair<Int,Long>>) {

    fun getWorkList() : List<Long> = work.map { it.second }

    fun decryptValues(key : Long) {
        for (i in work.indices) {
            val tmp = work[i]
            work[i] = tmp.first to (tmp.second * key)
        }
    }

    override fun toString() : String = buildString { this.append(work.map { it.second } ) }

    fun coordinates() : List<Long> {
        val nullPos = work.indexOfFirst { it.second == 0L }
        return listOf(1000, 2000, 3000)
            .map {
                work[(nullPos + it) % work.size].second
            }
    }

    fun coordinatesSum(): Long = coordinates().sum()

    fun mix() {
        for (i in work.indices) {
            val oldPos = work.indexOfFirst { it.first == i }
            transition(oldPos)
        }
    }

    fun transition(oldPos: Int) {
        val tmp = work.removeAt(oldPos)
        val value = tmp.second
        // note: size is one less after the removal
        val newPos = (oldPos + value).mod(work.size)
        work.add(newPos, tmp)
    }

    companion object {
        fun fromList(input: List<Long>) : Ring {
            val work = input.mapIndexed { index, i -> index to i }.toMutableList()
            return Ring(work)
        }
        fun fromString(input: String) : Ring {
            val list = input
                .split('\n')
                .filter { it.isNotEmpty() }
                .map { it.toLong() }
            return fromList(list)
        }
    }
}
fun day20(test: Boolean = true) {
    val inputText = if (test)
        """
        1
        2
        -3
        3
        -2
        0
        4
        """.trimIndent()
      else
        InputTextDownloader().getText(2022, 20)

    // Part A
    val r = Ring.fromString(inputText)
    r.mix()
    println(r.coordinates())
    println(r.coordinatesSum())

    // Part B
    val r2 = Ring.fromString(inputText)
    r2.decryptValues(811589153L)
    repeat(10) { r2.mix() }
    println(r2.coordinatesSum())
}
