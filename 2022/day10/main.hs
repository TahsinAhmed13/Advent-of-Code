import Data.List

parse [] = []
parse (x:xs) 
    | tokens !! 0 == "noop" = 0:(parse xs)
    | otherwise = 0:(read (tokens !! 1) :: Int):(parse xs)
    where tokens = words x

part1 input = 
    let ops = parse input
        values = map ((+1) . sum) $ inits ops
        cycles = [20, 60, 100, 140, 180, 220]
    in  sum $ map (\x -> x*(values !! (x-1))) cycles

drawPixel (x, y) screen =
    let (firstRows, secondRows) = splitAt x screen
        (firstCols, secondCols) = splitAt y (head secondRows)
        newRow = firstCols ++ '#':(tail secondCols) 
    in  firstRows ++ newRow:(tail secondRows)
        
part2 input = 
    let ops = parse input
        pos = map ((+1) . sum) $ inits ops
        coords = [(x, y) | x <- [0..5], y <- [0..39]]
        screen = replicate 6 $ replicate 40 '.'
    in  foldl (\acc (p, (x, y)) -> if abs (p-y) < 2 then drawPixel (x, y) acc else acc) screen $ zip pos coords
    
main = do
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    mapM putStrLn $ (part2 $ lines input)
