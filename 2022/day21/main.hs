import qualified Data.Map as Map 

parseOp '+' = (+)
parseOp '-' = (-)
parseOp '*' = (*)
parseOp '/' = (/)

parseLine input = 
    let tokens = words input
        name = init $ tokens !! 0 
        val = (read $ tokens !! 1) :: Double
        x = tokens !! 1 
        y = tokens !! 3
        op = parseOp . head $ tokens !! 2
    in  if length tokens < 3 then (name, Left val)
        else (name, Right (x, y, op))

parse = Map.fromList . map parseLine

evaluate name monkeys = 
    case Map.lookup name monkeys of 
        (Just (Left x)) -> x
        (Just (Right (x, y, op))) -> op (evaluate x monkeys) (evaluate y monkeys)  
        Nothing -> (-1)

part1 input = 
    let monkeys = parse input
    in  floor $ evaluate "root" monkeys

diff name monkeys =
    let (Just (Right (x, y, _))) = Map.lookup name monkeys
    in  signum $ (evaluate x monkeys) - (evaluate y monkeys)

search monkeys l r
    | l > r = (-1)
    | val == 0 = mid 
    | val == start = search monkeys mid r
    | otherwise = search monkeys l mid 
    where start = diff "root" $ Map.insert "humn" (Left 1) monkeys
          mid = (l+r)/2
          val = diff "root" $ Map.insert "humn" (Left mid) monkeys

part2 input = 
    let monkeys = parse input
        upper = fromIntegral (maxBound :: Int)
    in  floor $ search monkeys 1 upper

main = do 
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)

