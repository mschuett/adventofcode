import java.util.ArrayDeque
import java.util.Deque
import kotlin.math.abs

fun Coord2D.dist(other: Coord2D): Int =
    abs(other.x - this.x) + abs(other.y - y)

data class LavaPoolFlow(val heatLoss: Int = 0,
                        val pos: Coord2D = Coord2D(0,0),
                        val hist: List<Coord2D> = listOf()): Comparable<LavaPoolFlow> {
    override fun compareTo(other: LavaPoolFlow): Int {
        return heatLoss.compareTo(other.heatLoss)
    }
}

data class LavaPoolMap2D(val lines: List<String>, val xMax: Int, val yMax: Int,
                         val flows: Deque<LavaPoolFlow> = ArrayDeque(mutableListOf(LavaPoolFlow())),
                         val targetPos: Coord2D = Coord2D(xMax, yMax),
) {
    companion object Factory {
        fun fromString(inputText: String): LavaPoolMap2D {
            val lines = inputText.trim().split('\n')
            return LavaPoolMap2D(lines, lines[0].indices.last, lines.indices.last)
        }
    }
    fun prettyPrint() {
        var outstr: String = "${this.javaClass.name}(x=${xMax}, y=${yMax})\n"
        outstr += lines.joinToString("\n")
        println(outstr)
    }

    private fun getFieldAtPos(pos: Coord2D): Int = lines[pos.y][pos.x].digitToIntOrNull()!!

    fun flow(): LavaPoolFlow? {
        var flow: LavaPoolFlow
        var bestFlow: LavaPoolFlow? = null
        var count = 100000
        while (count >= 0 && flows.isNotEmpty()) {
            // count--
            flow = flows.pop()

            // prune flow on some conditions
            if (flow.heatLoss > (10 * flow.pos.dist(Coord2D(0,0)))) {
                continue
            }
            if (bestFlow != null && bestFlow.heatLoss < flow.heatLoss) {
                continue
            }

            listOf<Coord2D>(
                flow.pos + Direction2D.N,
                flow.pos + Direction2D.S,
                flow.pos + Direction2D.E,
                flow.pos + Direction2D.W
            ).asSequence()
             .filter {  // stay inside map
                it.x in 0..xMax && it.y in 0..yMax
            }.filterNot {  // do not return
                it in flow.hist
            }.filterNot {  // 'at most three blocks straight' rule
                flow.hist.size >= 2 &&
                        (flow.hist.takeLast(2) + flow.pos).all { p ->
                    p.x == it.x || p.y == it.y
                }
            }.map {
                LavaPoolFlow(
                    flow.heatLoss + getFieldAtPos(it),
                    it,
                    flow.hist + flow.pos
                )
            }.filterNot {  // prevent too many crossings of own history
                it.hist.count { old -> old == it.pos } > 2
            }.filter {
                it.heatLoss < 110
            }.onEach {
                if (it.pos == targetPos) {
                    if (bestFlow == null || bestFlow!!.heatLoss >= it.heatLoss) {
                        bestFlow = it
                    }
                } else
                    flows.add(it)
            }.toList()
        }
        println("after steps...\nbest flow is $bestFlow\n${flows.count()} other flows still active, with heat losses of ${flows.sortedBy { it.heatLoss }.take(3).map {it.heatLoss}}")
        return bestFlow
    }
}


fun day17(test: Boolean = true) {
    val inputText = if (test)
        """
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 17)

    // Part One
    val map = LavaPoolMap2D.fromString(inputText)
    val result = map.flow()
    if (result != null)
        println(result)
    else {
        println(map.flows.count())
        // println(map.flows)
    }

    // Part Two
}
