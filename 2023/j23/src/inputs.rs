#[allow(dead_code)]
pub const EXAMPLE:&str = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

#[allow(dead_code)]
pub const INPUT:&str = r"#.###########################################################################################################################################
#.#####...###...#####...#...###.......#.....#...#####...#...###...#.........###...#...#.......#...#...#...###.............###...###...###...#
#.#####.#.###.#.#####.#.#.#.###.#####.#.###.#.#.#####.#.#.#.###.#.#.#######.###.#.#.#.#.#####.#.#.#.#.#.#.###.###########.###.#.###.#.###.#.#
#...#...#...#.#.###...#...#...#.....#.#.#...#.#...#...#.#.#...#.#.#.......#...#.#.#.#.#.....#.#.#.#.#.#.#...#...........#.#...#.....#.#...#.#
###.#.#####.#.#v###.#########.#####.#.#.#.###.###.#.###.#.###.#.#.#######.###.#.#.#.#.#####.#.#.#.#.#.#.###.###########.#.#.#########.#.###.#
###.#...###.#.#.>.#.#.......#.....#.#.#.#.#...#...#...#...#...#.#.#.....#.#...#.#.#.#...###.#.#.#.#.#.#.#...#...#.....#.#...#.........#.#...#
###.###.###.#.#v#.#.#.#####.#####.#.#.#.#.#.###.#####.#####.###.#.#.###.#.#.###.#.#.###.###.#.#.#.#.#.#.#.###.#.#.###.#.#####.#########.#.###
###.#...#...#.#.#...#.#...#.#.....#.#.#.#.#.#...#...#.....#...#.#.#...#.#.#.#...#.#.#...#...#...#.#.#.#.#...#.#.#...#.#.#...#...........#...#
###.#.###.###.#.#####.#.#.#.#.#####.#.#.#.#.#.###.#.#####.###.#.#.###.#.#.#.#.###.#.#.###.#######.#.#.#.###.#.#.###.#.#.#.#.###############.#
###...###.....#...###.#.#.#...#...#.#.#.#...#...#.#.....#...#.#.#.###.#.#.#.#...#...#...#.......#.#.#.#.#...#.#...#.#.#...#.#.........#...#.#
#################.###.#.#.#####.#.#.#.#.#######.#.#####.###.#.#.#.###.#.#.#.###.#######.#######.#.#.#.#.#.###.###.#.#.#####.#.#######.#.#.#.#
#...#...#...#...#...#.#.#.#.>.>.#...#.#.#.......#.#.....###.#.#.#.###.#.#.#.###.....#...#...#...#.#.#.#.#...#.#...#.#.#...#.#.......#.#.#.#.#
#.#.#.#.#.#.#.#.###.#.#.#.#.#v#######.#.#.#######.#.#######.#.#.#.###.#.#.#.#######.#.###.#.#.###.#.#.#.###.#.#.###.#.#.#.#.#######.#.#.#.#.#
#.#...#...#...#.....#...#...#.......#.#.#.#.....#.#...>.>...#.#.#.#...#.#.#.#.>.>...#...#.#.#...#.#.#.#.#...#.#.....#...#...#.......#...#...#
#.#################################.#.#.#.#.###.#.#####v#####.#.#.#.###.#.#.#.#v#######.#.#.###.#.#.#.#.#.###.###############.###############
#.#.........#...#.........###.......#...#...#...#.#.....#...#...#.#...#.#.#.#.#.....#...#.#.###.#.#.#.#.#.###.........#...#...###...###...###
#.#.#######.#.#.#.#######.###.###############.###.#.#####.#.#####.###.#.#.#.#.#####.#.###.#.###.#.#.#.#.#.###########.#.#.#.#####.#.###.#.###
#...###...#...#...#.......#...#.......#.....#.#...#.......#.....#...#.#.#.#.#.#.....#...#.#.#...#.#.#.#.#...#...>.>.#.#.#...#.....#.....#...#
#######.#.#########.#######.###.#####.#.###.#.#.###############.###.#.#.#.#.#.#.#######.#.#.#.###.#.#.#.###.#.###v#.#.#.#####.#############.#
#.......#...#.......#...###...#.#.....#.#...#...###.............#...#.#.#.#.#.#.......#.#.#.#...#.#.#.#...#.#...#.#...#.......#...#...#...#.#
#.#########.#.#######.#.#####.#.#.#####.#.#########.#############.###.#.#.#.#.#######.#.#.#.###.#.#.#.###.#.###.#.#############.#.#.#.#.#.#.#
#.#...#...#...#.......#.#...#...#...#...#...#.....#.....#.....###.....#.#.#.#.#.......#.#.#.#...#.#.#.#...#.....#...#...........#...#.#.#...#
#.#.#.#.#.#####.#######.#.#.#######.#.#####.#.###.#####.#.###.#########.#.#.#.#.#######.#.#.#.###.#.#.#.###########.#.###############.#.#####
#...#...#...#...#.......#.#.###...#...#.....#.#...#.....#.#...#...#...#.#.#.#.#.......#.#.#.#.#...#.#.#.#...........#...............#.#.....#
###########.#.###.#######.#.###.#.#####.#####.#.###.#####.#.###.#.#.#.#.#.#.#.#######.#.#.#.#.#.###.#.#.#.#########################.#.#####.#
###...#...#...###.......#.#.#...#.#...#.....#.#...#.......#...#.#.#.#.#.#.#...#...#...#...#...#...#.#...#.........#.................#.......#
###.#.#.#.#############.#.#.#.###.#.#.#####.#.###.###########.#.#.#.#.#.#.#####.#.#.#############.#.#############.#.#########################
#...#...#.............#.#.#.#...#.#.#.#.....#.#...#...#.......#.#.#.#.#...#.....#...#...#...#...#...#...#.........#...#.....#...#...###...###
#.###################.#.#.#.###.#.#.#.#v#####.#.###.#.#.#######.#.#.#.#####.#########.#.#.#.#.#.#####.#.#.###########.#.###.#.#.#.#.###.#.###
#...................#...#.#...#.#.#.#.>.>.#...#...#.#.#.###...#.#.#.#.#...#.....###...#.#.#.#.#.#...#.#.#...........#...###...#...#.#...#...#
###################.#####.###.#.#.#.###v#.#.#####.#.#.#.###.#.#.#.#.#.#.#.#####v###.###.#.#.#.#.#.#.#.#.###########.###############.#.#####.#
#.................#.....#...#...#.#.#...#...#...#...#.#...#.#.#.#.#.#.#.#.#...>.>.#.#...#.#.#.#.#.#.#.#.#...#.......#.....#...#.....#...#...#
#.###############.#####.###.#####.#.#.#######.#.#####.###.#.#.#.#.#.#.#.#.#.###v#.#.#.###.#.#.#.#.#.#.#.#.#.#v#######.###.#.#.#.#######.#.###
#.#.....#...#...#.......#...#.....#.#.......#.#.......#...#.#...#.#.#...#.#.#...#.#.#...#.#.#.#.#.#.#.#.#.#.>.>.#...#.#...#.#...#.......#...#
#.#.###.#.#.#.#.#########.###.#####.#######.#.#########.###.#####.#.#####.#.#.###.#.###.#.#.#.#.#.#.#.#.#.###v#.#.#.#.#.###v#####.#########.#
#.#.###...#...#.....#.....###.....#.#.......#.#...#...#.###.....#.#.....#...#...#...###.#.#...#.#.#.#.#.#.#...#...#.#.#.#.>.#.....#.........#
#.#.###############.#.###########.#.#.#######.#.#.#.#.#v#######.#.#####.#######.#######.#.#####.#.#.#.#.#.#.#######.#.#.#.#v#.#####.#########
#.#.#...............#.........#...#.#...#...#...#...#.>.>.....#.#.#.....###...#.......#.#...#...#.#.#.#...#.......#.#.#.#.#.#.#.....#.......#
#.#.#.#######################.#.###.###.#.#.###########v#####.#.#.#.#######.#.#######.#.###.#.###.#.#.###########.#.#.#.#.#.#.#.#####.#####.#
#...#...........#...#.........#.....###...#...#...#...#.....#.#.#.#.....#...#.........#.....#...#.#...#...###...#.#.#.#...#.#.#.......#.....#
###############.#.#.#.#######################.#.#.#.#.#####.#.#.#.#####.#.#####################.#.#####.#.###.#.#.#.#.#####.#.#########.#####
###...#.........#.#.#.......#.................#.#...#.......#...#...#...#...#.........#...#...#.#...###.#.....#...#...#.....#.#.......#.....#
###.#.#.#########.#v#######.#.#################.###################.#.#####.#.#######.#.#.#.#.#.###.###.###############.#####.#.#####.#####.#
###.#.#...........#.>.#.....#.................#...#.......#.......#...#.....#.#.......#.#.#.#.#.#...#...#...........###.....#.#.....#.#...#.#
###.#.#############v#.#.#####################.###.#.#####.#.#####.#####.#####.#.#######.#.#.#.#.#.###.###.#########.#######.#.#####.#.#.#.#.#
###.#.###...#...#...#.#.......###.............###...#.....#.#.....#...#.......#...#...#.#...#.#.#...#...#...#.....#.....###...#...#.#.#.#.#.#
###.#.###.#.#.#.#.###.#######.###.###################.#####.#.#####.#.###########.#.#.#.#####.#.###.###.###.#.###.#####.#######.#.#.#.#.#.#.#
#...#.....#...#...#...#...#...#...#.............#...#...#...#...#...#...#####...#...#.#.#.....#...#.###...#.#...#.......#.......#...#...#...#
#.#################.###.#.#.###.###.###########.#.#.###.#.#####.#.#####.#####.#.#####.#.#.#######.#.#####.#.###.#########.###################
#.#...............#...#.#.#...#.....#.......#...#.#.#...#.#.....#.#.....#.....#.......#.#...#...#...#...#...###.........#...#...#...#...#...#
#.#.#############.###.#.#.###.#######.#####.#.###.#.#.###.#.#####.#.#####.#############.###.#.#.#####.#.###############.###.#.#.#.#.#.#.#.#.#
#...###.........#...#...#.....###...#.....#...#...#.#...#.#.#.....#.....#.............#.#...#.#.#...#.#.....###.........###...#...#...#...#.#
#######.#######.###.#############.#.#####.#####.###.###.#.#.#.#########.#############v#.#.###.#.#.#.#.#####.###.###########################.#
###...#.#.....#.....###...#...#...#.....#...###...#...#...#...#.......#.#...#.......>.>.#.###.#.#.#.#...#...#...#...#...###...#...#.......#.#
###.#.#.#.###.#########.#.#.#.#.#######.###.#####.###.#########.#####.#.#.#.#.#######v###.###.#.#.#.###.#.###.###.#.#.#.###.#.#v#.#.#####.#.#
#...#...#.###.......#...#.#.#.#.......#.....#...#.#...###...#...#...#...#.#.#...#.....#...#...#...#.#...#.###...#.#.#.#.#...#.>.#.#...###...#
#.#######.#########.#.###.#.#.#######.#######.#.#.#.#####.#.#.###.#.#####.#.###.#.#####.###.#######.#.###.#####.#.#.#.#.#.#####v#.###.#######
#...#.....###.......#...#.#.#.........#...###.#.#.#.#...#.#.#...#.#...#...#...#.#.....#...#...#.....#...#.#...#.#.#.#.#...###...#.....#...###
###.#.#######.#########.#.#.###########.#.###.#.#.#.#.#.#.#.###v#.###.#.#####.#.#####.###.###.#.#######.#.#.#.#v#.#.#.#######.#########.#.###
###.#.#.......#...#...#.#.#.............#...#.#...#.#.#...#.#.>.>.###...#...#...#.....###...#.#.#...#...#...#.>.>.#.#.#.......#...#...#.#.###
###.#.#.#######.#.#.#.#.#.#################.#.#####.#.#####.#.#v#########.#.#####.#########.#.#.#.#.#.#########v###.#.#.#######.#.#.#.#.#.###
#...#.#.#.....#.#.#.#.#.#.#...#...###...#...#.....#.#.....#...#...#...###.#.#...#.....#.....#.#.#.#.#.....#.....###.#.#.....#...#...#...#...#
#.###.#.#.###v#.#.#.#.#.#.#.#.#.#.###.#.#v#######.#.#####.#######.#.#.###.#.#.#.#####.#.#####.#.#.#.#####.#.#######.#.#####.#.#############.#
#.....#...###.>.#.#.#.#.#...#.#.#.#...#.>.>...#...#.......#...###...#...#.#...#.......#...#...#.#.#.#...#.#.#.....#...###...#.#.......#...#.#
#############v###.#.#.#.#####.#.#.#.#####v###.#.###########.#.#########.#.###############.#.###.#.#.#.#.#.#.#.###.#######.###.#.#####.#.#.#.#
#.........#...#...#.#.#...#...#.#.#.#.....###.#...........#.#...........#...............#...###.#.#.#.#.#.#...###.......#...#.#.....#.#.#.#.#
#.#######.#.###.###.#.###.#.###.#.#.#.#######.###########.#.###########################.#######.#.#.#.#.#.#############.###.#.#####.#.#.#.#.#
#...#...#...###.#...#.#...#...#.#.#.#.....###.....#...#...#.................###.........#.....#...#...#...#.............#...#.#.....#.#.#.#.#
###.#.#.#######.#.###.#.#####.#.#.#.#####.#######.#.#.#.###################.###.#########.###.#############.#############.###.#.#####.#.#.#.#
###...#.......#...###...#...#...#...#####.......#...#.#.#...................#...#...#...#...#.....###...###.............#.#...#...###...#.#.#
#############.###########.#.###################.#####.#.#.###################.###.#.#.#.###.#####.###.#.###############.#.#.#####.#######.#.#
#####...#.....#...#...#...#...#...#...#.......#...###...#.................###...#.#...#.....#.....#...#...#.............#...#####...#...#.#.#
#####.#.#.#####.#.#.#.#.#####.#.#.#.#.#.#####.###.#######################.#####.#.###########.#####.#####.#.#######################.#.#.#.#.#
#.....#...#.....#.#.#.#.#.....#.#.#.#.#.....#.....#...#...#.............#...###...#...#.....#...###...#...#.................#.......#.#.#...#
#.#########.#####.#.#.#.#.#####.#.#.#.#####v#######.#.#.#.#.###########.###.#######.#.#.###.###.#####.#.###################.#.#######.#.#####
#...#.....#.....#.#.#.#.#.....#.#.#.#.....>.>.###...#.#.#.#...#.......#.#...#...#...#...###.....#...#.#.###.................#.........#...###
###.#.###.#####.#.#.#.#.#####.#.#.#.#######v#.###.###.#.#.###.#.#####.#.#.###.#.#.###############.#.#.#.###.#############################.###
###...#...###...#.#.#.#.#.....#.#...#...#...#...#...#...#...#.#.#.....#...#...#.#.........###...#.#.#.#...#.......#.....#...#...#...#...#...#
#######.#####.###.#.#.#.#.#####.#####.#.#.#####.###.#######.#.#.#.#########.###.#########.###.#.#.#.#.###.#######.#.###.#.#.#.#.#v#.#.#.###.#
#.....#...#...###.#.#.#.#.#...#.#.....#.#.....#...#.#...#...#...#.........#...#...........#...#.#.#.#...#.#...###.#...#.#.#.#.#.>.#...#...#.#
#.###.###v#.#####.#.#.#.#.#.#.#.#.#####.#####.###.#.#.#.#.###############.###.#############.###.#.#.###.#.#.#.###v###.#.#.#.#.###v#######.#.#
#...#.###.>.#...#...#.#.#...#...#.....#.......#...#.#.#...#####...#.......###.........#...#...#.#.#...#.#.#.#...>.>.#.#.#.#.#...#.......#.#.#
###.#.###v###.#.#####.#.#############.#########.###.#.#########.#.#.#################.#.#.###.#.#.###.#.#.#.#####v#.#.#.#.#.###.#######.#.#.#
#...#.....###.#.....#...#...#.......#.........#.#...#.###...###.#.#.#.....#.....#.....#.#.#...#.#.###.#.#.#.#.....#...#...#.#...#.......#...#
#.###########.#####.#####.#.#.#####.#########.#.#.###.###.#.###.#.#v#.###.#.###.#.#####.#.#.###.#.###.#.#.#.#.#############.#.###.###########
#...#.........#...#.#.....#.#.....#.#...#...#.#...#...#...#...#.#.>.>.#...#.#...#...###.#.#...#.#...#.#.#.#.#.........#.....#.#...###...#...#
###.#.#########.#.#.#.#####.#####.#.#.#.#.#.#.#####.###.#####.#.###v###.###.#.#####v###.#.###.#.###.#.#.#.#.#########.#.#####.#.#####.#.#.#.#
###...#...#...#.#...#.....#...###.#.#.#...#.#...#...#...#####...###...#...#.#...#.>.>...#...#.#.....#.#.#...#.........#.#...#.#.......#...#.#
#######.#.#.#.#.#########.###.###.#.#.#####.###.#.###.###############.###.#.###.#.#v#######.#.#######.#.#####.#########.#.#.#.#############.#
#.......#...#...###.......#...#...#...#...#.....#.....#...#####...#...#...#.###...#...#.....#...#...#.#.#...#.........#.#.#...#...........#.#
#.#################.#######.###.#######.#.#############.#.#####.#.#.###.###.#########.#.#######.#.#.#.#.#.#.#########.#.#.#####.#########.#.#
#.............#...#.......#...#.........#...#.....#...#.#.###...#.#...#...#.#...###...#...#.....#.#.#...#.#.#.........#...#...#.........#.#.#
#############.#.#.#######.###.#############.#.###.#.#.#.#.###.###.###.###.#.#.#.###.#####.#.#####.#.#####.#.#.#############.#.#########.#.#.#
#####...#.....#.#.#...###...#...............#...#.#.#...#...#...#.....###...#.#...#.....#...#.....#.###...#...###...#...#...#.#.........#.#.#
#####.#.#.#####.#.#.#.#####.###################.#.#.#######.###.#############.###.#####.#####.#####.###.#########.#.#.#.#.###.#.#########.#.#
#.....#.#.....#.#...#.#...#...........###...#...#.#...#...#.....#...#.........###.#.....#...#.....#...#.......#...#...#.#...#.#.....#####...#
#.#####.#####.#.#####.#.#.###########.###.#.#.###.###.#.#.#######.#.#.###########.#.#####.#.#####.###.#######.#.#######.###.#.#####.#########
#.....#.......#.#.....#.#.............#...#.#...#.###.#.#.###.....#.#...........#...#...#.#.#...#.#...###...#.#...#.....#...#.....#.........#
#####.#########.#.#####.###############.###.###.#.###v#.#.###.#####.###########.#####.#.#.#.#.#.#.#.#####.#.#v###.#.#####.#######.#########.#
#.....#.........#.....#...............#...#.#...#...>.>.#...#.#.....#...........#...#.#.#.#.#.#.#.#.#...#.#.>.>.#.#.#...#.....#...#.........#
#.#####.#############.###############.###.#.#.#######v#####.#.#.#####.###########.#.#.#.#.#.#.#.#.#.#.#.#.###v#.#.#.#.#.#####.#.###v#########
#.......###...........###.............#...#.#.#.......#.....#.#.#...#...........#.#...#...#...#.#.#.#.#...###.#.#.#.#.#...#...#.#.>.#...#...#
###########.#############.#############.###.#.#.#######.#####.#.#.#.###########.#.#############.#.#.#.#######.#.#.#.#.###.#.###.#.#v#.#.#.#.#
###...#.....#.......#...#.........###...###.#.#...#...#.....#.#.#.#...#...#.....#...#.........#.#.#.#.#.......#...#.#.###.#.###...#.#.#.#.#.#
###.#.#.#####.#####.#.#.#########.###.#####.#.###.#.#.#####.#.#.#.###.#.#.#.#######.#.#######.#.#.#.#.#.###########.#.###.#.#######.#.#.#.#.#
###.#.#.......#.....#.#.#.........#...#.....#...#...#.....#.#.#.#.#...#.#.#...#...#...#.....#.#.#.#.#.#.....#...###...#...#...#.....#.#.#.#.#
###.#.#########.#####.#.#.#########.###.#######.#########.#.#.#.#.#.###.#.###.#.#.#####.###.#.#.#.#.#.#####.#.#.#######.#####.#.#####.#.#.#.#
#...#.........#.....#.#.#.......###...#.#...#...#.........#...#.#.#.###.#.#...#.#.#.....###...#.#.#...#...#...#.......#.#...#.#...#...#...#.#
#.###########.#####.#.#.#######.#####.#.#.#.#.###.#############.#.#.###.#.#.###.#.#.###########.#.#####.#.###########.#.#.#.#.###.#.#######.#
#...........#.......#.#.....#...#...#.#...#...###.............#.#.#...#.#.#...#.#.#.......#.....#...#...#...#.........#...#.#.#...#.#.....#.#
###########v#########.#####.#.###.#.#.#######################.#.#.###.#.#.###v#.#.#######.#.#######.#.#####.#.#############.#.#.###.#.###.#.#
#.......###.>...#...#.#.....#...#.#.#.........###.......#.....#.#.###...#.#.>.>.#.........#...#.....#.....#.#.#...........#.#.#...#.#...#.#.#
#.#####.###v###.#.#.#.#.#######v#.#.#########.###.#####.#.#####.#.#######.#.#v###############.#.#########.#.#.#.#########.#.#.###.#.###.#.#.#
#.....#.....###.#.#.#.#.#...#.>.>.#...#.......#...#...#...#...#...#.....#...#.#...#...#...###...###...#...#.#...#.....#...#...#...#.#...#...#
#####.#########.#.#.#.#.#.#.#.#v#####.#.#######.###.#.#####.#.#####.###.#####.#.#.#.#.#.#.#########.#.#.###.#####.###.#.#######.###.#.#######
#.....#.......#...#.#.#.#.#.#.#.#...#.#.......#...#.#...#...#.#...#.#...#.....#.#...#...#.....#...#.#.#...#.#.....###...#.....#.....#.......#
#.#####.#####.#####.#.#.#.#.#.#.#.#.#.#######.###.#.###.#.###.#.#.#.#.###.#####.#############.#.#.#.#.###.#.#.###########.###.#############.#
#.....#.#.....#...#.#.#...#...#...#.#.........###.#.#...#.#...#.#...#...#.......#.............#.#.#.#.....#.#...........#...#.#...........#.#
#####.#.#.#####.#.#.#.#############.#############.#.#.###.#.###.#######.#########.#############.#.#.#######.###########.###.#.#.#########.#.#
#.....#.#.......#.#...###...........###...###...#...#.#...#...#.#.......#.........#.........#...#.#.#.....#.....#.....#.#...#.#.........#...#
#.#####.#########.#######.#############.#.###.#.#####.#.#####.#.#.#######.#########.#######.#.###.#.#.###.#####.#.###.#.#.###.#########.#####
#.....#.#.........#.......#.......#.....#...#.#.#...#.#.#.....#.#.#...###...#.......#.......#...#.#...#...#...#...###...#.#...#.......#.....#
#####.#.#.#########.#######.#####.#.#######.#.#.#.#.#.#.#.#####.#.#.#.#####.#.#######.#########.#.#####.###.#.###########.#.###.#####.#####.#
#...#...#.....#...#.........#.....#...#.....#.#.#.#.#...#.......#...#.....#...#.......#...#...#.#.....#.#...#...........#.#.....#...#.......#
#.#.#########.#.#.###########.#######.#.#####.#.#.#.#####################.#####.#######.#.#.#.#.#####.#.#.#############.#.#######.#.#########
#.#...#.......#.#.....#...###.......#.#.###...#.#.#...###...#.............#...#.....#...#...#...#.....#...#.....#.......#.........#.........#
#.###.#.#######.#####.#.#.#########.#.#.###.###.#.###.###.#.#.#############.#.#####.#.###########.#########.###.#.#########################.#
#...#.#.......#.....#.#.#.#...#...#...#...#.#...#...#.#...#.#...........#...#.#...#...#.........#.....#...#...#.#.....#...###...#...###.....#
###.#.#######.#####.#.#.#.#.#.#.#.#######v#.#.#####.#.#.###.###########.#.###.#.#.#####.#######.#####.#.#.###.#.#####.#.#.###.#.#.#.###v#####
#...#.....#...#...#.#.#.#.#.#.#.#.#...#.>.>.#.....#.#.#...#...#...#...#.#...#.#.#.....#.......#.......#.#.....#.......#.#.#...#.#.#.#.>.#...#
#.#######.#.###.#.#.#.#.#.#.#.#.#.#.#.#.#########.#.#.###.###.#.#.#.#.#.###.#.#.#####.#######.#########.###############.#.#.###.#.#.#.#v#.#.#
#.......#.#...#.#.#.#.#.#.#.#.#.#...#.#.....#.....#.#.#...#...#.#.#.#...#...#...#.....#.......#...#...#...........#.....#.#...#.#.#.#.#.#.#.#
#######.#.###.#.#.#.#.#.#.#.#.#.#####.#####.#.#####.#.#.###.###.#.#v#####.#######.#####.#######.#.#.#.###########.#.#####.###.#.#.#.#.#.#.#.#
#.......#.....#.#...#...#...#.#...###.#.....#.#...#.#.#.#...#...#.>.>.###.......#.#...#...#...#.#.#.#.#...#...#...#.#.....#...#.#.#.#.#.#.#.#
#.#############.#############.###.###.#.#####.#.#.#.#.#.#.###.#######.#########.#.#.#.###v#.#.#.#.#.#.#.#.#.#.#.###.#.#####.###.#.#.#.#.#.#.#
#.#.......#...#...........#...#...#...#...###.#.#.#.#.#.#...#...#####...#.....#.#.#.#.#.>.>.#.#.#.#.#.#.#.#.#.#...#.#.#...#.#...#.#...#...#.#
#.#.#####.#.#.###########.#.###.###.#####.###.#.#.#.#.#.###.###.#######.#.###.#.#.#.#.#.#####.#.#.#.#.#.#.#.#.###v#.#.#.#.#.#.###.#########.#
#.#...#...#.#...#.....#...#...#...#...#...#...#.#.#.#.#...#...#...#.....#...#.#.#.#.#.#.#.....#.#.#.#.#.#.#.#.#.>.>.#.#.#.#.#...#.....#.....#
#.###.#.###.###.#.###.#.#####.###.###.#.###.###.#.#.#.###.###.###.#.#######.#.#.#.#.#.#.#.#####.#.#.#.#.#.#.#.#.#####.#.#.#.###.#####.#.#####
#.....#.....###...###...#####.....###...###.....#...#.....###.....#.........#...#...#...#.......#...#...#...#...#####...#...###.......#.....#
###########################################################################################################################################.#";
