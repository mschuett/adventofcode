data class Array2Dint(val inputText: String) {
    private var rows = ArrayList<ArrayList<Int>>()
    private var columns = ArrayList<ArrayList<Int>>()

    init {
        rows = inputText
                .split('\n')
                .filter { it.isNotEmpty() }
                .map {
                    it.map {
                            c -> c.digitToInt()
                    }
                } as ArrayList<ArrayList<Int>>
        columns = IntRange(0, rows[0].size-1)
            .map {i ->
                rows.map { row ->
                    row[i]
                }
            } as ArrayList<ArrayList<Int>>
    }

    private fun getUp(x: Int, y: Int) : List<Int> = columns[x].slice(0 until y)
    private fun getDown(x: Int, y: Int) : List<Int> = columns[x].slice(y+1 until columns[x].size)
    private fun getLeft(x: Int, y: Int) : List<Int> = rows[y].slice(0 until x)
    private fun getRight(x: Int, y: Int) : List<Int> = rows[y].slice(x+1 until rows[y].size)
    fun countExternallyVisible(printDebugOutput: Boolean = false) : Int {
        val visibleTrees = ArrayList<Pair<Int,Int>>()
        for (x in 0 until columns.size) {
            for (y in 0 until columns[x].size) {
                // using columns for 'correct' x,y coordinates
                if (x == 0 || y == 0 || x == columns.size-1 || y == rows.size-1) {
                    visibleTrees.add(x to y)
                    continue
                }
                val height = columns[x][y]
                val left  = getLeft(x, y)
                val right = getRight(x, y)
                val up   = getUp(x, y)
                val down = getDown(x, y)

                // many checks, because I had logic problems :(
                assert(left.size + 1 + right.size == 100) { "invalid row len" }
                assert(up.size + 1 + down.size == 100) { "invalid column len" }
                assert(100 == rows[y].size) { "invalid column len" }
                assert(100 == rows.size) { "invalid column len" }
                assert(100 == rows.size) { "invalid column len" }
                assert(100 == columns[x].size) { "invalid column len" }
                assert(100 == columns.size) { "invalid column len" }
                assert(100 == rows[y].size) { "invalid column len" }
                assert(left + listOf(height) + right == rows[y]) { "invalid row" }
                assert(up + listOf(height) + down == columns[x]) { "invalid column" }
//                println("= Tree $x,$y; h$height")
//                println("left/right: " + left.size.toString() + " + " + right.size.toString() + " = "
//                        +(left.size + right.size))
//                println("up/down   : " + up.size.toString() + " + " + down.size.toString() + " = "
//                        +(up.size + down.size))

                if (left.all { it < height }
                 || right.all { it < height }
                 || up.all { it < height }
                 || down.all { it < height }
                ) {
                    visibleTrees.add(x to y)
                }
            }
        }

        if (printDebugOutput) prettyPrint(visibleTrees)
        return visibleTrees.size
    }

    private fun findInternallyVisible(x: Int, y: Int) : Pair<Int, ArrayList<Pair<Int,Int>>> {
        val visibleTrees = ArrayList<Pair<Int,Int>>()
        visibleTrees.add(x to y)

        val height = columns[x][y]
        val left  = arrayListOf(0) + getLeft(x, y).asReversed()
        val right = arrayListOf(0) + getRight(x, y)
        val up    = arrayListOf(0) + getUp(x, y).asReversed()
        val down  = arrayListOf(0) + getDown(x, y)

        var leftScore = left.size - 1
        var rightScore = right.size - 1
        var upScore = up.size - 1
        var downScore = down.size - 1

        // this is all quite ugly, because the `dist` has two functions:
        // it is a) the array index, and b) the score multiplier
        // that causes all kinds of of-by-one errors  :-(
        for (dist in 1 until left.size) {
            assert(left[dist] == columns[x-dist][y])
            visibleTrees.add(x-dist to y)
            if (left[dist] >= height) {
                leftScore = dist
                break
            }
        }
        for (dist in 1 until right.size) {
            assert(right[dist] == columns[x+dist][y])
            visibleTrees.add(x+dist to y)
            if (right[dist] >= height) {
                rightScore = dist
                break
            }
        }
        for (dist in 1 until up.size) {
            assert(up[dist] == columns[x][y-dist])
            visibleTrees.add(x to y-dist)
            if (up[dist] >= height) {
                upScore = dist
                break
            }
        }
        for (dist in 1 until down.size) {
            assert(down[dist] == columns[x][y+dist])
            visibleTrees.add(x to y+dist)
            if (down[dist] >= height) {
                downScore = dist
                break
            }
        }

        val score = leftScore * rightScore * upScore * downScore
        return score to visibleTrees
    }

    fun maxInternallyVisible(printDebugOutput: Boolean = false) : Int {
        var maxVisibleTrees = ArrayList<Pair<Int,Int>>()
        var maxScore = 0

        for (x in 0 until columns.size) {
            for (y in 0 until columns[x].size) {
                val (score, visibleTrees) = findInternallyVisible(x, y)
                if (score > maxScore) {
                    maxVisibleTrees = visibleTrees
                    maxScore = score
                }
            }
        }
        if (printDebugOutput) prettyPrint(maxVisibleTrees)
        return maxScore
    }

    private fun prettyPrint(maxVisibleTrees: ArrayList<Pair<Int, Int>>) {
        val red = "\u001b[31m"
        val reset = "\u001b[0m"
        rows.mapIndexed { y, row ->
            row.mapIndexed { x, i ->
                if ((x to y) in maxVisibleTrees) "$red$i$reset" else "$i"
            }.joinToString("")
        }.joinToString("\n").also(::println)
    }
}

fun day08(test: Boolean = true) {
    val inputText = if (test)
        """
            30373
            25512
            65332
            33549
            35390
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 8)

    val arr = Array2Dint(inputText)
    // Part A
    println("visible from the outside: " + arr.countExternallyVisible())
    // Part B
    println("highest scenic score: " + arr.maxInternallyVisible(true))
}
