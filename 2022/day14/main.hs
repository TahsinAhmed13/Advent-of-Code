import qualified Data.Set as Set

parseLines input = 
    let getCoords s = 
            let x = (read $ takeWhile (/= ',') s) :: Int
                y = (read . tail $ dropWhile (/= ',') s) :: Int
            in  (x, y)
    in  map (map getCoords . filter (/= "->") . words) input

parseGrid input = 
    let lineCoords = parseLines input
        getRocks ((x1, y1):(x2, y2):coords) =
            let rocks = if x1 == x2 then Set.fromList [(x1, y) | y <- [(min y1 y2)..(max y1 y2)]]
                        else Set.fromList [(x, y1) | x <- [(min x1 x2)..(max x1 x2)]]
            in  rocks `Set.union` (getRocks ((x2, y2):coords))
        getRocks _ = Set.empty
    in  foldr (\x acc -> (getRocks x) `Set.union` acc) Set.empty lineCoords

inBounds (x, _) grid =
    let mn = fst $ Set.findMin grid
        mx = fst $ Set.findMax grid
    in  mn <= x && x <= mx

dropSand (x, y) grid
    | not $ inBounds (x, y) grid = grid
    | (x, y+1) `Set.notMember` grid = dropSand (x, y+1) grid
    | (x-1, y+1) `Set.notMember` grid = dropSand (x-1, y+1) grid
    | (x+1, y+1) `Set.notMember` grid = dropSand (x+1, y+1) grid
    | otherwise = Set.insert (x, y) grid

simulate grid = 
    let grid' = dropSand (500, 0) grid
    in  if Set.size grid == Set.size grid' then 0 
        else (simulate grid')+1

part1 input = 
    let grid = parseGrid input
    in  simulate grid

dropSand2 floor (x, y) grid
    | y+1 == floor = Set.insert (x, y) grid
    | (x, y+1) `Set.notMember` grid = dropSand2 floor (x, y+1) grid
    | (x-1, y+1) `Set.notMember` grid = dropSand2 floor (x-1, y+1) grid
    | (x+1, y+1) `Set.notMember` grid = dropSand2 floor (x+1, y+1) grid
    | otherwise = Set.insert (x, y) grid

simulate2 floor grid = 
    let grid' = dropSand2 floor (500, 0) grid
    in  if (500, 0) `Set.member` grid then 0
        else (simulate2 floor grid')+1

part2 input = 
    let grid = parseGrid input
        floor = (Set.findMax $ Set.map snd grid)+2
    in  simulate2 floor grid
        
main = do 
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)
