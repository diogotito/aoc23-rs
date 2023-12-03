Game = Struct.new(:id, :cube_sets)
CubeSet = Struct.new(:red, :green, :blue)

games = File.foreach("../input/day2.txt").collect do |line|
  /Game (\d+): (.+)/ =~ line
  Game.new \
    $1.to_i,
    $2.scan(/(((?<r>\d+) red|(?<g>\d+) green|(?<b>\d+) blue)(, )?){1,3}/)
      .map { |set| CubeSet.new(*set.map { |cubes| (cubes || 0).to_i }) }
end

part1 = games.reject { |game|
  game.cube_sets.any? { _1.red > 12 || _1.green > 13 || _1.blue > 14 }
}.sum &:id

part2 = games.map { |game|
  CubeSet.members.map {
    game.cube_sets.map(&_1).max
  }.reduce :*
}.sum

pp part1:, part2:

# Part 1: 2176
# Part 2: 63700