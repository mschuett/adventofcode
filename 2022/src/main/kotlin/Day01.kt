fun day01(test: Boolean = true) {
    val inputText = if (test)
        """
            1000
            2000
            3000
            
            4000
            
            5000
            6000
            
            7000
            8000
            9000
            
            10000
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 1)

    val elves = ArrayList<ArrayList<Int>>(30)

    var elf = ArrayList<Int>(5)
    for (line in inputText.split('\n')) {
        if (line.isNotEmpty()) {
            elf.add(line.toInt())
        } else {
            elves.add(elf)
            elf = ArrayList(5)
        }
    }
    elves.add(elf)
    println(elves)

    // Part a
    var maxCalories = 0
    for (carry in elves) {
        val calories = carry.sum()
        if (calories > maxCalories) {
            maxCalories = calories
        }
    }
    println("max calories are $maxCalories")

    // Part b
    val sums = ArrayList(elves.map { arr -> arr.sum() })
    sums.sortDescending()

    println("sum: $sums")
    val topThree = sums.slice(0..2)
    println("top three elves are: $topThree")
    println("sum: " + topThree.sum())
}
