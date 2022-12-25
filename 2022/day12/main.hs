import Data.Char
import qualified Data.Set as Set
import qualified Data.Map as Map
import Data.Maybe

findStart grid = 
    let r = length $ takeWhile ('S' `notElem`) grid
        c = length . takeWhile (/= 'S') $ grid !! r
    in  (r, c)

findEnd grid = 
    let r = length $ takeWhile ('E' `notElem`) grid
        c = length . takeWhile (/= 'E') $ grid !! r
    in (r, c)

findAs grid = 
    let h = length grid
        w = length $ grid !! 0 
    in  filter (\(x, y) -> (grid !! x) !! y == 'a') [(x, y) | x <- [0..h-1], y <- [0..w-1]]

isValid (x, y) (nx, ny) grid = 
    let h = length grid
        w = length $ grid !! 0 
        inBounds = 0 <= nx && nx < h && 0 <= ny && ny < w
        a = (grid !! x) !! y
        b = (grid !! nx) !! ny
        value 'S' = ord 'a'
        value 'E' = ord 'z'
        value c = ord c
    in  inBounds && (value b - value a) < 2 

getAdj (x, y) grid =  
    let dx = [-1, 1, 0, 0]
        dy = [0, 0, -1, 1]
    in  foldr (\(dx, dy) acc -> if isValid (x, y) (x+dx, y+dy) grid then (x+dx, y+dy):acc else acc) [] $ zip dx dy

bfs xs ys grid = 
    let h = length grid
        w = length $ grid !! 0 
        dist = Map.empty
        pq = Set.fromList $ zip3 (replicate (length xs) 0) xs ys
        traverse pq dist 
            | Set.size pq == 0 = dist
            | otherwise = 
                let ((d, x, y), pq') = Set.deleteFindMin pq
                    dist' = Map.insert (x, y) d dist
                    adj = filter (not . flip Map.member dist') $ getAdj (x, y) grid 
                    pq'' = foldr (\(x, y) acc -> Set.insert (d+1, x, y) acc) pq' adj
                in  if Map.member (x, y) dist then traverse pq' dist
                    else traverse pq'' dist'
    in  traverse pq dist

part1 grid = 
    let (sx, sy) = findStart grid
        (ex, ey) = findEnd grid
        dist = bfs [sx] [sy] grid 
    in  fromMaybe (-1) (Map.lookup (ex, ey) dist)

part2 grid = 
    let (sx, sy) = findStart grid
        (ex, ey) = findEnd grid
        xs = sx:(map fst $ findAs grid)
        ys = sy:(map snd $ findAs grid)
        dist = bfs xs ys grid
    in  fromMaybe (-1) (Map.lookup (ex, ey) dist)

main = do 
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)
