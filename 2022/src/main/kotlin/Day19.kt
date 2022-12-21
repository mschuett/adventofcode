enum class BpMaterial(val i: Int) {
    NONE(i=0),
    ORE(i=0),
    CLAY(i=1),
    OBSIDIAN(i=2),
    GEODE(i=3),
}
data class Blueprint(
    val id: Int,
    val robotCosts: Map<BpMaterial,Map<BpMaterial,Int>>,
//    val oreRobotCost: Int,
//    val clayRobotCost: Int,
//    val obsidianRobotCost: Pair<Int,Int>,  // ore + clay
//    val geodeRobotCost: Pair<Int,Int>,     // ore + obsidian
    ) {
    companion object {
        fun fromString(input: String) : Blueprint {
            val (idPart, specPart) = input.split(": ")
            val id = idPart.split(' ')[1].toInt()
            val spec : MutableMap<BpMaterial,MutableMap<BpMaterial,Int>> = mutableMapOf()

            specPart.split('.')
                .filter { it.isNotEmpty() }
                .forEach {
                    val words = it.trim().split(' ')
                    require(words[0] == "Each")
                    require(words[2] == "robot")
                    require(words[3] == "costs")
                    require(words[5] == "ore")
                    val targetMaterial = BpMaterial.valueOf(words[1].uppercase())
                    val costOre = words[4].toInt()
                    spec[targetMaterial] = mutableMapOf()
                    spec[targetMaterial]!![BpMaterial.ORE] = costOre
                    if (words.size > 6 && words[6] == "and") {  // second material
                        require(words.size == 9)
                        val costOther = words[7].toInt()
                        val materialOther = BpMaterial.valueOf(words[8].uppercase())
                        spec[targetMaterial]!![materialOther] = costOther
                    }
            }
            // special recipe: we can always do nothing
            spec[BpMaterial.NONE] = mutableMapOf(BpMaterial.ORE to 0)
            return Blueprint(id, spec)
        }
    }
}

data class FactorySim(val blueprint: Blueprint,
                      var minutes: Int = 1,
                      val maxMinutes : Int = 24,
                      val robots: MutableList<Int> = mutableListOf(1,0,0,0),
                      val inventory: MutableList<Int> = mutableListOf(0,0,0,0),
                      val nextTarget: BpMaterial = BpMaterial.ORE,
) {
    private val maxOreCost = blueprint.robotCosts.maxOf { (_,cost) ->
        cost.getOrDefault(BpMaterial.ORE, 0)
    }
    private val maxClayCost = blueprint.robotCosts.maxOf { (_,cost) ->
        cost.getOrDefault(BpMaterial.CLAY, 0)
    }
    fun canBuild(target: BpMaterial): Boolean =
        (target == BpMaterial.NONE)
                || blueprint.robotCosts[target]!!.all { (type, cost) ->
                inventory[type.i] >= cost
            }
    fun doBuild(target: BpMaterial) : FactorySim {
        if (target == BpMaterial.NONE) return this
        blueprint.robotCosts[target]!!.forEach { (type, cost) ->
            require(inventory[type.i] >= cost)
            inventory[type.i] -= cost
        }
        robots[target.i]++
        return this
    }

    fun iterate(): Int {
        print("iterate bp#${blueprint.id} min $minutes, R:$robots, I:$inventory")
        if (minutes == maxMinutes) {
            println("  -> end with ${inventory[BpMaterial.GEODE.i]} geodes\n")
            return inventory[BpMaterial.GEODE.i]
        }
        // robots active
        for (i in robots.indices) {
            inventory[i] += robots[i]
        }
        val possibleBuilds : List<BpMaterial> = BpMaterial.values().reversed()
                .filter { target ->
                    val can = canBuild(target)
                    when (target) {
                        BpMaterial.NONE -> can && !canBuild(BpMaterial.CLAY) && (robots.sum() > minutes/2)
                        BpMaterial.ORE -> can && (robots[BpMaterial.ORE.i] < maxOreCost) && (inventory[BpMaterial.ORE.i] < maxOreCost)
                        BpMaterial.CLAY -> can && (robots[BpMaterial.CLAY.i] < maxClayCost) && (inventory[BpMaterial.CLAY.i] < maxClayCost)
                        else -> can
                    }
                }

        if (possibleBuilds.isEmpty()) {
            println("  -> build NONE")
            return this.copy(minutes = minutes + 1, robots = robots.toMutableList(), inventory = inventory.toMutableList()).iterate()
        }
        else if (possibleBuilds.first() == BpMaterial.GEODE) {
            // for better robot: only build this
            doBuild(possibleBuilds.first())
            println("  -> build ${possibleBuilds.first()}")
            return this.copy(minutes = minutes + 1, robots = robots.toMutableList(), inventory = inventory.toMutableList()).iterate()
        }
        else { // decision/branching time
            return possibleBuilds.map { target ->
                println("  -> take option for $target")
                this.copy(minutes = minutes + 1, robots = robots.toMutableList(), inventory = inventory.toMutableList())
                .doBuild(target)
                .iterate()
            }.max()
        }
    }
}

fun day19(test: Boolean = true) {
    val inputText = if (test)
        """
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
        """.trimIndent()
      else
        InputTextDownloader().getText(2022, 19)

    val blueprints = inputText
        .split('\n')
        .filter { it.isNotEmpty() }
        .map {
            Blueprint.fromString(it)
        }
        .onEach(::println)

    // Part A
    blueprints
        .map {
            FactorySim(it)
        }
        .onEach(::println)
        .map {
            it.iterate() to it
        }
        .onEach(::println)
//        .maxBy{ it.first }
        .map { (num, sim) ->
            num * sim.blueprint.id
        }
        .also(::println)
        .sum()
        .also(::println)


    // Part B

}
