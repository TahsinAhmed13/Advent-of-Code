import qualified Data.Set as Set

parseBlueprint input = 
    let tokens = words input 
        n = (read . init $ tokens !! 1) :: Int
        ore = [(read $ tokens !! 6) :: Int, 0, 0, 0]
        clay = [(read $ tokens !! 12) :: Int, 0, 0, 0]
        obsidian = [(read $ tokens !! 18) :: Int, (read $ tokens !! 21) :: Int, 0, 0]
        geode = [(read $ tokens !! 27) :: Int, 0, (read $ tokens !! 30) :: Int, 0]
    in  (n, [ore, clay, obsidian, geode])

parse input = map parseBlueprint input

constraints bp = 
    [maximum $ map (!!i) bp | i <- [0..(length $ bp !! 0)-1]] 

time reqs robots materials = 
    let can = and $ map (\i -> (robots!!i) > 0 || (reqs!!i) <= (materials!!i)) [0..(length reqs)-1]
        calculate a b r = ceiling $ (fromIntegral (max 0 (b-a))) / (fromIntegral r)
    in  if can then maximum $ zipWith3 calculate materials reqs robots
        else (-1)

geodes _ 0 _ materials = last materials 
geodes bp m robots materials = 
    let build i = 
            let t = time (bp!!i) robots materials
                can = 0 <= t && t < m && (i == (length bp)-1 || (robots!!i) < ((constraints bp)!!i))
                (first, x:second) = splitAt i robots
                robots' = first ++ (x+1):second
                materials' = zipWith (-) (zipWith (+) materials $ map (*(t+1)) robots) (bp!!i)
            in  if can then geodes bp (m-t-1) robots' materials'
                else 0
        g = last . zipWith (+) materials $ map (*m) robots
    in  max g . maximum $ map build [0..(length bp)-1]

part1 input = 
    let blueprints = parse input
        qualities = map (\(i, bp) -> i*(geodes bp 24 [1, 0, 0, 0] [0, 0, 0, 0])) blueprints
    in  sum qualities

part2 input = 
    let blueprints = take 3 $ parse input
        gs = map (\(_, bp) -> geodes bp 32 [1, 0, 0, 0] [0, 0, 0, 0]) blueprints
    in  product gs

main = do 
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)
