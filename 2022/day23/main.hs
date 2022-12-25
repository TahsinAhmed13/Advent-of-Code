import qualified Data.Set as Set
import qualified Data.Map as Map
import Data.Maybe

parse input = 
    let h = length input
        w = length $ input !! 0 
        coords = [(y, x) | y <- [0..(h-1)], x <- [0..(w-1)], (input !! y) !! x == '#']
    in  Set.fromList coords

getNeighbors (y, x) = [(y+dy, x+dx) | dy <- [(-1)..1], dx <- [(-1)..1], dx /= 0 || dy /= 0]

isAlone elves (y, x) = and $ map (`Set.notMember` elves) (getNeighbors (y, x)) 

isClear elves (y, x) 0 = and $ map (`Set.notMember` elves) [(y-1, nx) | nx <- [(x-1)..(x+1)]]
isClear elves (y, x) 1 = and $ map (`Set.notMember` elves) [(y+1, nx) | nx <- [(x-1)..(x+1)]] 
isClear elves (y, x) 2 = and $ map (`Set.notMember` elves) [(ny, x-1) | ny <- [(y-1)..(y+1)]] 
isClear elves (y, x) 3 = and $ map (`Set.notMember` elves) [(ny, x+1) | ny <- [(y-1)..(y+1)]] 

getOffset = ([(-1, 0), (1, 0), (0, -1), (0, 1)] !!)

findNext elves round (y, x) = 
    let ds = filter (isClear elves (y, x)) [(d+round) `mod` 4 | d <- [0..3]]
        (dy, dx) = getOffset $ head ds 
        (ny, nx) = (y+dy, x+dx)
    in  if isAlone elves (y, x) || null ds then (y, x) 
        else (ny, nx)

simulate elves 0 = elves
simulate elves rounds = 
    let elves' = simulate elves (rounds-1)
        next = Map.fromList [(pos, findNext elves' (rounds-1) pos) | pos <- Set.toList elves']            
        freq = Map.foldr (\x acc -> Map.insertWith (+) x 1 acc) Map.empty next
        move pos = do
            npos <- Map.lookup pos next
            f <- Map.lookup npos freq
            return $ if f < 2 then npos else pos
    in  Set.map (\pos -> fromMaybe pos $ move pos) elves'

part1 input = 
    let elves = simulate (parse input) 10
        miny = Set.findMin $ Set.map fst elves
        maxy = Set.findMax $ Set.map fst elves
        minx = Set.findMin $ Set.map snd elves
        maxx = Set.findMax $ Set.map snd elves
        total = (maxy-miny+1)*(maxx-minx+1)
    in  total - (Set.size elves)

simulate2 elves round = 
    let done = and $ [isAlone elves pos | pos <- Set.toList elves] 
        next = Map.fromList [(pos, findNext elves (round-1) pos) | pos <- Set.toList elves]            
        freq = Map.foldr (\x acc -> Map.insertWith (+) x 1 acc) Map.empty next
        move pos = do
            npos <- Map.lookup pos next
            f <- Map.lookup npos freq
            return $ if f < 2 then npos else pos
        elves' = Set.map (\pos -> fromMaybe pos $ move pos) elves
    in  if done then round
        else simulate2 elves' (round+1)

part2 input = 
    let elves = parse input
    in  simulate2 elves 1

main = do 
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)
