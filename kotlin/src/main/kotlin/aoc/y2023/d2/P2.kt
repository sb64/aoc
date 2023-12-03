package aoc.y2023.d2.p2

fun solve(input: String): String =
        input.lines()
                .map {
                    val (_, game) = it.split(": ")
                    val maximumColor = { targetColor: String ->
                        game.split("; ")
                                .map {
                                    it.split(", ")
                                            .map {
                                                val (numCubes, color) = it.split(' ')
                                                if (color == targetColor) {
                                                    numCubes.toInt()
                                                } else {
                                                    0
                                                }
                                            }
                                            .max()
                                }
                                .max()
                    }
                    val maximumRed = maximumColor("red")
                    val maximumGreen = maximumColor("green")
                    val maximumBlue = maximumColor("blue")
                    maximumRed * maximumGreen * maximumBlue
                }
                .sum()
                .toString()
