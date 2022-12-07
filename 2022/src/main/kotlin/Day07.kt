enum class FsType {
    File,
    Dir,
}
// Tree representation of Files and Directories
data class FsEntry(val name: String, val type: FsType, val size: Int, val content: HashMap<String,FsEntry>, val parent: FsEntry?) {
    override fun toString() : String {
        val path = getPath()
        return if (type == FsType.Dir) {
            val subs = content.keys.sorted()
            "$path (dir, entries=$subs)"
        } else {
            "$path (file, size=$size)"
        }
    }

    fun prettyPrint(): String {
        return "$this\n" + content.keys.sorted().joinToString("") { key ->
            content[key]!!.prettyPrint()
        }
    }

    fun getPath() : String {
        return if (parent == null)
            "/"
        else {
            val parentName = parent.getPath()
            val name = "$parentName/$name"
            name.replaceFirst("""//""", "/")
        }
    }

    fun getTotalSize() : Int {
        return size + content.map { (_, entry) -> entry.getTotalSize() }.sum()
    }

    fun getAllDirs() : List<FsEntry>? {
        return if (type == FsType.File)
            null
        else {
            content.values.mapNotNull { it.getAllDirs() }.flatten() + this
        }
    }
}

fun day07(test: Boolean = true) {
    val inputText = if (test)
        """
            $ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 7)

    val fsRoot = FsEntry("/", FsType.Dir, 0, HashMap(), null)
    var fsCurrent = fsRoot

    inputText
        .splitToSequence(Regex("""\n?\$ """))
        .filter { it.isNotEmpty() }
        .also(::println)
        .forEach {
            val lines = it.split('\n').filter { line -> line.isNotEmpty() }
            val commandline = lines[0].split(' ')
            when (commandline.first()) {
                "cd" -> {
                    assert(commandline.size == 2)
                    val dir = commandline.last()
                    when (dir) {
                        ".." -> {
                            assert(fsCurrent.parent != null) { "cannot cd.. in root" }
                            fsCurrent = fsCurrent.parent!!
                        }
                        "/" -> {
                            fsCurrent = fsRoot
                        }
                        else -> {
                            if (dir !in fsCurrent.content) {
                                val subDir = FsEntry(dir, FsType.Dir, 0, HashMap(), fsCurrent)
                                fsCurrent.content[dir] = subDir
                            }
                            fsCurrent = fsCurrent.content[dir]!!
                        }
                    }
                    println("cd $dir, now in " + fsCurrent.getPath())
                }
                "ls" -> {
                    assert(commandline.size == 1)
                    println("ls, in " + fsCurrent.getPath())
                    lines.drop(1).forEach { line ->
                        val (size, name) = line.split(' ')
                        if (name !in fsCurrent.content) {
                            fsCurrent.content[name] = if (size == "dir") {
                                FsEntry(name, FsType.Dir, 0, HashMap(), fsCurrent)
                            }
                            else {
                                FsEntry(name, FsType.File, size.toInt(), HashMap(), fsCurrent)
                            }
                        }
                    }
                    println("--> content in " + fsCurrent.getPath() + " after ls: " + fsCurrent.content.keys.sorted())
                }
            }
        }

    // part A
    val allDirSizes = fsRoot.getAllDirs()!!.map { it.getPath() to it.getTotalSize() }
        .also(::println)
    allDirSizes.filter { it.second <= 100000 }
        .sumOf {it.second}
        .also(::println)
    // part B
    val totalSpaceAvailable = 70000000
    val spaceNeeded = 30000000
    val currentlyUsed = fsRoot.getTotalSize()
    val currentlyUnused = totalSpaceAvailable - currentlyUsed

    allDirSizes
        .sortedBy { it.second } //.also(::println)
        .find { it.second > (spaceNeeded - currentlyUnused) }.also(::println)
}
