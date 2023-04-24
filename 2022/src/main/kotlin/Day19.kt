enum class BpMaterial(val i: Int) {
    NONE(i=0),
    ORE(i=1),
    CLAY(i=2),
    OBSIDIAN(i=3),
    GEODE(i=4),
}

data class Blueprint(
    val id: Int,
    val oreRobotCost: Int,
    val clayRobotCost: Int,
    val obsidianRobotCost: Pair<Int,Int>,  // ore + clay
    val geodeRobotCost: Pair<Int,Int>,     // ore + obsidian
    ) {
    val maxOreCost = maxOf(oreRobotCost, clayRobotCost, obsidianRobotCost.first, geodeRobotCost.first)

    companion object {
        fun fromString(input: String) : Blueprint {
            val (idPart, specPart) = input.split(": ")
            val id = idPart.split(' ')[1].toInt()
            var oreRobotCost = 0
            var clayRobotCost = 0
            var obsidianRobotCost = 0 to 0
            var geodeRobotCost = 0 to 0
            specPart.split('.')
                .filter { it.isNotEmpty() }
                .forEach {
                    val words = it.trim().split(' ')
                    require(words[0] == "Each")
                    require(words[2] == "robot")
                    require(words[3] == "costs")
                    require(words[5] == "ore")
                    require(words.size == 6 || words.size == 9)
                    when (words[1]) {
                        "ore"      -> oreRobotCost = words[4].toInt()
                        "clay"     -> clayRobotCost = words[4].toInt()
                        "obsidian" -> obsidianRobotCost = words[4].toInt() to words[7].toInt()
                        "geode"    -> geodeRobotCost = words[4].toInt() to words[7].toInt()
                    }
            }
            return Blueprint(id, oreRobotCost, clayRobotCost, obsidianRobotCost, geodeRobotCost)
        }
    }
}

/* state is always at the _start_ of the minute, before any action */
data class SimulationState(
    val blueprint: Blueprint,
    val maxMinutes : Int = 24,
    val minute: Int = 1,
    val robots: List<Int> = listOf(0,1,0,0,0),
    val inventory: List<Int> = listOf(0,0,0,0,0),
    val productionTarget: BpMaterial = BpMaterial.NONE,
) {
    override fun toString(): String {
        var outstr = "== Minute $minute == Target: ${productionTarget.toString().lowercase()}\n"
        BpMaterial.values().forEach {
            if (robots[it.i] > 0) {
                outstr += "${robots[it.i]} ${it.toString().lowercase()}-collecting robots collect ${robots[it.i]} ${it.toString().lowercase()};" +
                        " you now have ${inventory[it.i] + robots[it.i]} ${it.toString().lowercase()}.\n"
            }
        }
        if (canBuild && productionTarget != BpMaterial.NONE) {
            outstr += "The new ${productionTarget.toString().lowercase()}-collecting robot is ready; you now have ${robots[productionTarget.i] + 1} of them.\n"
        }
        return outstr
    }

    private val canBuild: Boolean =
        when (productionTarget) {
            BpMaterial.NONE -> true
            BpMaterial.ORE -> inventory[BpMaterial.ORE.i] >= blueprint.oreRobotCost
            BpMaterial.CLAY -> inventory[BpMaterial.ORE.i] >= blueprint.clayRobotCost
            BpMaterial.OBSIDIAN -> inventory[BpMaterial.ORE.i] >= blueprint.obsidianRobotCost.first && inventory[BpMaterial.CLAY.i] >= blueprint.obsidianRobotCost.second
            BpMaterial.GEODE -> inventory[BpMaterial.ORE.i] >= blueprint.geodeRobotCost.first && inventory[BpMaterial.OBSIDIAN.i] >= blueprint.geodeRobotCost.second
        }

    private val possibleBuilds : List<BpMaterial> =
        BpMaterial.values().reversed()
            .filter { target ->
                when (target) {
                    BpMaterial.NONE -> false
                    BpMaterial.ORE -> (robots[BpMaterial.ORE.i] < blueprint.maxOreCost)
                    BpMaterial.CLAY -> (robots[BpMaterial.CLAY.i] < blueprint.obsidianRobotCost.second)
                    BpMaterial.OBSIDIAN -> robots[BpMaterial.CLAY.i] > 0 && (robots[BpMaterial.OBSIDIAN.i] < blueprint.geodeRobotCost.second)
                    BpMaterial.GEODE -> robots[BpMaterial.OBSIDIAN.i] > 0
                }
            }

    private fun doBuilding(oldInventory: List<Int>, oldRobots: List<Int>): Pair<MutableList<Int>, MutableList<Int>> {
        val inventory = oldInventory.toMutableList()
        val robots = oldRobots.toMutableList()
        if (canBuild) {
            when (productionTarget) {
                BpMaterial.NONE -> {}
                BpMaterial.ORE -> {
                    inventory[BpMaterial.ORE.i] -= blueprint.oreRobotCost
                    robots[productionTarget.i] += 1
                }

                BpMaterial.CLAY -> {
                    inventory[BpMaterial.ORE.i] -= blueprint.clayRobotCost
                    robots[productionTarget.i] += 1
                }

                BpMaterial.OBSIDIAN -> {
                    inventory[BpMaterial.ORE.i] -= blueprint.obsidianRobotCost.first
                    inventory[BpMaterial.CLAY.i] -= blueprint.obsidianRobotCost.second
                    robots[productionTarget.i] += 1
                }

                BpMaterial.GEODE -> {
                    inventory[BpMaterial.ORE.i] -= blueprint.geodeRobotCost.first
                    inventory[BpMaterial.OBSIDIAN.i] -= blueprint.geodeRobotCost.second
                    robots[productionTarget.i] += 1
                }
            }
        }
        return inventory to robots
    }

    private fun doRobots(inventory: List<Int>, robots: List<Int>) : MutableList<Int> {
        // optimized, the list builder is really slow...
        return mutableListOf(
            inventory[0] + robots[0],
            inventory[1] + robots[1],
            inventory[2] + robots[2],
            inventory[3] + robots[3],
            inventory[4] + robots[4],
        )
    }

    fun iterate(): List<SimulationState> {
        var (newInventory, newRobots) = doBuilding(inventory, robots)
        newInventory = doRobots(newInventory, robots)

        if (canBuild) {
            require(possibleBuilds.isNotEmpty())
            return possibleBuilds.map { newTarget ->
                SimulationState(
                    blueprint,
                    maxMinutes,
                    minute + 1,
                    newRobots,
                    newInventory,
                    newTarget,
                )
            }
        } else {
            return listOf(SimulationState(
                blueprint,
                maxMinutes,
                minute + 1,
                robots,
                newInventory.toMutableList(),
                productionTarget,
            ))
        }
    }
}

// Part A
fun getMaxGeodes(history: List<SimulationState>): Pair<Int, List<SimulationState>> {
    val state: SimulationState = history.last()
    return if (state.minute > state.maxMinutes) {
        state.inventory[BpMaterial.GEODE.i] to history
    } else {
        state.iterate()
            .map {
                getMaxGeodes(history + it)
            }
            .maxBy { it.first }
    }
}

// optimized for Part B,
// discard history and follow todd.ginsberg.com with using a work queue or a stack
fun getMaxGeodes2(init: SimulationState): Int {
    val workQueue = ArrayDeque<SimulationState>()
    workQueue.add(init)
    var maxGeodes = 0

    while (workQueue.isNotEmpty()) {
        val state = workQueue.removeFirst()

        // check new best value
        if (state.minute > state.maxMinutes) {
            if (state.inventory[BpMaterial.GEODE.i] > maxGeodes) {
                maxGeodes = state.inventory[BpMaterial.GEODE.i]
                // println("found $maxGeodes...")
            }
            continue
        }

        // prune check if best value can be reached
        val minLeft = state.maxMinutes + 1 - state.minute
        val maxPossibleGeodes = state.inventory[BpMaterial.GEODE.i] + (state.robots[BpMaterial.GEODE.i] * minLeft) + (minLeft*minLeft)
        if ( maxPossibleGeodes < maxGeodes )
            continue

        // iterate
        workQueue.addAll(0, state.iterate())
    }
    return maxGeodes
}


fun day19(test: Boolean = false) {
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
            getMaxGeodes(listOf(SimulationState(it)))
        }
        .sumOf {
            println("Blueprint ${it.second.first().blueprint.id} with ${it.first} max geodes  -> quality level ${it.first * it.second.first().blueprint.id}")
            it.first * it.second.first().blueprint.id
        }
        .also { println("puzzle answer: $it") }

    // Part B
    blueprints
        .take(3)
        .map {
            getMaxGeodes2(SimulationState(it, maxMinutes = 32))
        }
        .onEach {
            println("$it max geodes")
        }
        .reduce { product, element -> product * element }
        .also { println("puzzle answer: $it") }
}
