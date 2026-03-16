
typealias MachinePart = Map<Char, Int>

data class MPExpression(val result: String, val attrib: Char? = null, val cmp: Char? = null, val value: Int = 0) {
    companion object Factory {
        fun fromString(inputText: String): MPExpression {
            if (':' in inputText) {
                val (condText, result) = inputText.split(':')
                if ('<' in condText) {
                    val (attrib, value) = condText.split('<')
                    return MPExpression(result, attrib[0], '<', value.toInt())
                } else {
                    val (attrib, value) = condText.split('>')
                    return MPExpression(result, attrib[0], '>', value.toInt())
                }
            } else {
                return MPExpression(inputText)
            }
        }
    }
    fun apply(part: MachinePart): String? {
        // if expression matches return next step as String
        // otherwise return null
        return if (cmp == '<' && part[attrib]!! < value) result
        else if (cmp == '>' && part[attrib]!! > value) result
        else if (cmp == null) result
        else null
    }
}

data class MPWorkflow(val name: String, val rules: List<MPExpression>) {
    companion object Factory {
        fun fromString(inputText: String): MPWorkflow {
            assert(inputText.last() == '}')
            val (name, allRulesText) = inputText.trim('}').split('{')
            val rules = allRulesText.split(',').map {
                MPExpression.fromString(it)
            }
            return MPWorkflow(name, rules)
        }
    }
    fun apply(part: MachinePart): String {
        // return follow-up workflow
        var result: String? = null
        for (expr in rules) {
            result = expr.apply(part)
            if (result != null)
                return result
        }
        TODO()
    }
}

fun day19(test: Boolean = false) {
    val inputText = if (test)
        """
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}
        
        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 19)

    // parsing
    val (rulesText,partsText) = inputText.trim()
        .split("\n\n")
    val rules = rulesText.split('\n').associate {
        val workflow = MPWorkflow.fromString(it)
        workflow.name to workflow
    }
    val parts = partsText.split('\n').map { line ->
        assert(line.first() == '{')
        assert(line.last() == '}')
        line.trim('{', '}')
            .split(',').associate {
                val (attrib, valText) = it.split('=')
                attrib[0] to valText.toInt()
            }
    }

    // Part One
    val acceptedParts: MutableList<MachinePart> = mutableListOf()
    for (part in parts) {
        var result = "in"
        while (result != "A" && result != "R") {
            result = rules[result]!!.apply(part)
            if (result == "A")
                acceptedParts.add(part)
        }
        println("$part --> $result")
    }
    println(acceptedParts.sumOf { it.values.sum() })

    // Part Two

}
