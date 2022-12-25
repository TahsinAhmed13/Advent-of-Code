import qualified Data.Set as Set
import qualified Data.Map as Map
import Data.Maybe

rockOffsets = 
    [[(0, 0), (0, 1), (0, 2), (0, 3)],
    [(2, 1), (1, 0), (1, 1), (1, 2), (0, 1)], 
    [(2, 2), (1, 2), (0, 0), (0, 1), (0, 2)], 
    [(3, 0), (2, 0), (1, 0), (0, 0)], 
    [(1, 0), (1, 1), (0, 0), (0, 1)]]

isSafe (y, x) grid = 
    let inBounds = 0 <= x && x < 7 && 0 <= y
        bump = (y, x) `Set.member` grid
    in  inBounds && not bump

moveLeft i (y, x) grid = 
    let rock = map (\(dy, dx) -> (y+dy, x+dx-1)) $ rockOffsets !! i
    in  if all ((flip isSafe) grid) rock then (y, x-1)
        else (y, x)

moveRight i (y, x) grid = 
    let rock = map (\(dy, dx) -> (y+dy, x+dx+1)) $ rockOffsets !! i
    in  if all ((flip isSafe) grid) rock then (y, x+1)
        else (y, x)

moveDown i (y, x) grid = 
    let rock = map (\(dy, dx) -> (y+dy-1, x+dx)) $ rockOffsets !! i
    in  if all ((flip isSafe) grid) rock then (y-1, x)
        else (y, x)
                
dropRock wind i j (y, x) grid =
    let (y', x') = if wind !! j == '<' then moveLeft i (y, x) grid
                   else moveRight i (y, x) grid
        (y'', x'') = moveDown i (y', x') grid
        rock = map (\(dy, dx) -> (y'+dy, x'+dx)) $ rockOffsets !! i
        grid' = foldr (\coords acc -> Set.insert coords acc) grid rock
        j' = (j+1) `mod` (length wind) 
    in  if y == y'' then (grid', j')
        else dropRock wind i j' (y'', x'') grid

simulate wind 0 = (Set.empty, 0, [])
simulate wind steps = 
    let (grid, j, hs) = simulate wind (steps-1) 
        i = (steps-1) `mod` (length rockOffsets)
        h = if null hs then (-1) else (head hs)-1
        (grid', j') = dropRock wind i j (h+4, 2) grid
        h' = (fst $ Set.findMax grid')+1 
    in  (grid', j', h':hs)

part1 input = 
    let (_, _, heights) = simulate input 2022 
    in  head heights

normalize grid = 
    let (dy, _) = Set.findMin grid
    in  Set.map (\(y, x) -> (y-dy, x)) grid

clearRows grid = 
    let freq = Set.foldr (\(y, _) acc -> Map.insert y ((fromMaybe 0 $ Map.lookup y acc)+1) acc) Map.empty grid
        full = Map.filter (>=7) freq
        (row, _) = Map.findMax full
        grid' = normalize $ Set.filter (\(y, _) -> y > row) grid
    in  if null full then grid 
        else grid'

findCycle wind states i j grid =
    let states' = Map.insert (i `mod` (length rockOffsets), j, grid) i states 
        h = fromMaybe (-1) $ Set.lookupMax grid >>= return . fst
        i' = i+1
        (grid', j') = dropRock wind (i `mod` (length rockOffsets)) j (h+4, 2) grid
        grid'' = clearRows grid'
        pi = fromMaybe (-1) $ Map.lookup (i' `mod` (length rockOffsets), j', grid'') states
    in  if pi >= 0 then (pi, i') 
        else findCycle wind states' i' j' grid''

part2 input = 
    let (a, b) = findCycle input Map.empty 0 0 Set.empty
        steps = 1000000000000-a
        (_, _, hs) = simulate input b
        heights = drop (a-1) $ reverse hs
        heights' = map (subtract $ head heights) heights
    in  (head heights) + (last heights')*(steps `div` (b-a)) + (heights' !! (steps `mod` (b-a)))
    
main = do 
    input <- getContents
    putStrLn $ show (part1 input)
    putStrLn $ show (part2 input)
