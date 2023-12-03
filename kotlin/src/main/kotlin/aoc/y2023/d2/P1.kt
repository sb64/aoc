package aoc.y2023.d2.p1

fun solve(input: String): String =
        input.lines()
                .map {
                    val (name, game) = it.split(": ")
                    val gamePossible =
                            game.split("; ").all {
                                it.split(", ").all {
                                    val (numCubes, color) = it.split(' ')
                                    when (color) {
                                        "red" -> numCubes.toInt() <= 12
                                        "green" -> numCubes.toInt() <= 13
                                        "blue" -> numCubes.toInt() <= 14
                                        else -> throw Exception("got weird color: $color")
                                    }
                                }
                            }
                    if (gamePossible) {
                        name.removePrefix("Game ").toInt()
                    } else {
                        0
                    }
                }
                .sum()
                .toString()
