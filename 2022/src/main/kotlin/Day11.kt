typealias Worry = ULong

data class Monkey(val id: Int, var items: ArrayList<Worry>, val op: String, val testDivisor: Worry, val throwTrue: Int, val throwFalse: Int, var reduceWorry: Boolean = true) {
    var opMult: Worry = 1u
    var opAdd: Worry = 0u
    var opSquare: Boolean = false
    var inspections = 0
    var modulus : ULong = 1u

    init {
        assert(op.startsWith("new = old "))
        val words = op.split(' ')
        when (words[4]) {
            "+" -> opAdd = words[5].toULong()
            "*" -> if (words[5] == "old") {
                opSquare = true
            } else {
                opMult = words[5].toULong()
            }
        }
    }

    fun round(allMonkeys : HashMap<Int, Monkey>) {
        items.forEach {
            inspections++
            val worry = worryLevel(it)
            val throwTarget = if (worry % testDivisor == 0.toULong()) throwTrue else throwFalse
            allMonkeys[throwTarget]!!.items.add(worry)
            // println("M $id, throw $it->$worry to M $throwTarget")
        }
        items.clear()
    }

    private fun worryLevel(worry: Worry): Worry {
        val new = if (opSquare) {
            worry * worry
        } else if (opMult != 1.toULong()) {
            worry * opMult
        } else {
            worry + opAdd
        }
        return if (reduceWorry) {
            (new / 3.toULong())
        } else {
            new % modulus
        }
    }

    companion object MonkeyBuilder {
        fun fromString(lines: List<String>, reduceWorry: Boolean = true) : Monkey {
            assert(lines[0].startsWith("Monkey "))
            val id = lines[0][7].digitToInt()

            assert(lines[1].startsWith("  Starting items: "))
            val (_, itemlist) = lines[1].split(':')
            val items = itemlist.split(", ")
                .map { it.strip().toULong() }

            assert(lines[2].startsWith("  Operation "))
            val op = lines[2].split(':').last()

            assert(lines[3].startsWith("  Test: divisible by "))
            val testDiv = lines[3].split(' ').last().toULong()

            assert(lines[4].startsWith("    If true: throw to monkey "))
            assert(lines[5].startsWith("    If false: throw to monkey "))
            val trueMonkey = lines[4].split(' ').last().toInt()
            val falseMonkey = lines[5].split(' ').last().toInt()

            return Monkey(id, ArrayList(items), op, testDiv, trueMonkey, falseMonkey, reduceWorry)
        }
    }
}


fun day11(test: Boolean = false) {
    val inputText = if (test)
        """
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3
            
            Monkey 1:
              Starting items: 54, 65, 75, 74
              Operation: new = old + 6
              Test: divisible by 19
                If true: throw to monkey 2
                If false: throw to monkey 0
            
            Monkey 2:
              Starting items: 79, 60, 97
              Operation: new = old * old
              Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3
            
            Monkey 3:
              Starting items: 74
              Operation: new = old + 3
              Test: divisible by 17
                If true: throw to monkey 0
                If false: throw to monkey 1
            
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 11)

    partA(inputText)
    partB(inputText, 10000)
}

private fun partA(inputText: String, maxRounds: Int = 20) {
    val allMonkeys: HashMap<Int, Monkey> = hashMapOf()
    inputText
        .split("\n\n")
        .filter { it.isNotEmpty() }
        .also(::println)
        .map { Monkey.fromString(it.split('\n')) }
        .also(::println)
        .forEach { allMonkeys[it.id] = it }

    for (round in 1..maxRounds) {
        allMonkeys
            .forEach { (i, m) ->
                m.round(allMonkeys)
                // println("round $round, Monkey $i: " + m.items.joinToString(", "))
            }
    }
    printInspectionTimes(allMonkeys)
}

private fun partB(inputText: String, maxRounds: Int) {
    val allMonkeys: HashMap<Int, Monkey> = hashMapOf()
    inputText
        .split("\n\n")
        .filter { it.isNotEmpty() }
        .map { Monkey.fromString(it.split('\n'), reduceWorry = false) }
        .also(::println)
        .forEach { allMonkeys[it.id] = it }

    val modulus = allMonkeys
        .map { (_,m) -> m.opMult to m.testDivisor }
        .flatMap { (i,j) ->
            println("modulus factors $i,$j")
            listOf(i, j) }
        .reduce { acc, bigInteger -> acc * bigInteger }
        .also { println("modulus is $it")}
    allMonkeys.forEach { (_, m) -> m.modulus = modulus }
    for (round in 1..maxRounds) {
        allMonkeys
            .forEach { (i, m) ->
                m.round(allMonkeys)
                if (round % 2000 == 0)
                    println("round $round, Monkey $i inspected " + m.inspections + " times.")
            }
    }
    printInspectionTimes(allMonkeys)
}

private fun printInspectionTimes(allMonkeys: HashMap<Int, Monkey>) {
    allMonkeys
        .map { (i, m) ->
            val inspect = m.inspections
            println("Monkey $i inspected items $inspect times")
            inspect
        }
        .sorted().reversed().take(2)
        .also(::println)
        .map { it.toULong() }
        .reduce { acc: ULong, i: ULong -> acc * i }
        .also { println(it.toString()) }
}
