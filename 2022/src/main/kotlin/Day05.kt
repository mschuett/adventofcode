data class StackMoveOrder(val amount: Int, val from: Int, val to: Int) {
    companion object {
        fun fromString(input: String): StackMoveOrder {
            val words = input.split(' ')
            return StackMoveOrder(words[1].toInt(), words[3].toInt(), words[5].toInt())
        }
    }
}

typealias Box = Char
typealias BoxStack = ArrayList<Box>

fun Map<Int,BoxStack>.prettyprint() : String =
    this
        .map { (k, v) -> "$k : " + v.joinToString(" ") }
        .joinToString("\n")

fun Map<Int,BoxStack>.topBoxes() : String =
    this
        .map { (_,stack) -> stack.last() }
        .joinToString("")

fun parseInputToStacks(input1: String) : Map<Int,BoxStack> {
    // read text columns of each stack
    val stackPositions = input1
        .split('\n').last()
        .mapIndexed { idx, value -> idx to value }
        .filter { it.second != ' ' }
        .map { (idx, value) -> Triple(idx, "$value".toInt(), BoxStack()) }
        // .also(::println)
    // probably way too over-complicated way to read the data into a Map of BoxStacks (=ArrayList<Box>)
    val workingStacks = input1.split('\n')
        .dropLast(1)
        .asReversed()
        .map { stackPositions
            .map { (idx, stacknum, stack) ->
                val box = it[idx]
                if (box != ' ') {
                    stack.add(box)
                }
                Pair (stacknum, stack)
            }
        }
        .last()
        // .also(::println)
        .toMap()
        // .also(::println)
    return workingStacks
}

fun parseInputToOrders(input2: String) : List<StackMoveOrder> {
    return input2
        .split("\n")
        .filter { it.isNotEmpty() }
        .map { StackMoveOrder.fromString(it) }
        // .also(::println)
}

fun day05(test: Boolean = true) {
    val inputText = if (test)
        """
                [D]    
            [N] [C]    
            [Z] [M] [P]
             1   2   3 
            
            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 5)

    val (input1, input2) = inputText.split("\n\n")
    val orders = parseInputToOrders(input2)

    // part A
    val workingStacks = parseInputToStacks(input1)
    orders.forEach {
        for (i in 1..it.amount) {
            require(workingStacks[it.from] != null)
            require(workingStacks[it.to] != null)
            val box = workingStacks[it.from]!!.removeLast()
            workingStacks[it.to]!!.add(box)
        }
    }
    println("\nCrateMover 9000:\n"
            + workingStacks.prettyprint()
            + "\ntop boxes: "
            + workingStacks.topBoxes())

    // part B
    val workingStacks2 = parseInputToStacks(input1)
    orders.forEach {
        require(workingStacks2[it.from] != null)
        require(workingStacks2[it.to] != null)
        val boxes = workingStacks2[it.from]!!.takeLast(it.amount)
        for (i in 1..it.amount) {
            workingStacks2[it.from]!!.removeLast()
        }
        workingStacks2[it.to]!! += boxes
    }

    println("\nCrateMover 9001:\n"
            + workingStacks2.prettyprint()
            + "\ntop boxes: "
            + workingStacks2.topBoxes())
}
